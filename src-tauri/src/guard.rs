//! Circuito de seguridad para los intentos de login a SQL Server.
//!
//! Un password incorrecto en un bucle automatico provoca el bloqueo de la
//! cuenta en SQL Server (error 18456), y eso deja sin servicio a todas las
//! demas apps que comparten el mismo login. Este modulo:
//!
//! - permite como maximo `MAX_ATTEMPTS` intentos followed en cada rafaga
//! - bloquea todo intento posterior hasta que el usuario lo re-dispare
//! - distingue "no se pudo autenticar" (peligroso, bloquea) de
//!   "no hay red" (inofensivo, solo aplica backoff)

use std::sync::Mutex;
use std::time::{Duration, Instant};

/// Intentos de autenticacion seguidos antes de bloquear.
pub const MAX_ATTEMPTS: u32 = 3;

/// Separacion minima entre dos logins, evita ráfagas por clics repetidos.
pub const MIN_INTERVAL: Duration = Duration::from_secs(5);

#[derive(Default)]
struct Inner {
    /// Fallos de autenticacion seguidos en la rafaga actual.
    auth_failures: u32,
    /// Fallos de red seguidos (no bloquean, solo hacen backoff).
    net_failures: u32,
    /// Bloqueado: no se intenta mas hasta reset manual.
    blocked: bool,
    /// Ultimo error guardado, para mostrarlo en la UI.
    last_error: String,
    /// Motivo del bloqueo.
    block_reason: String,
    /// Cuando fue el ultimo intento (para el min_interval).
    last_attempt: Option<Instant>,
    /// True si el ultimo intento fallo. El intervalo minimo solo aplica
    /// cuando fallo: si la contrasena es correcta, las llamadas en rafaga
    /// legitimas (verificar tablas, imagenes por item) deben poder pasar.
    last_failed: bool,
    /// Numero de veces que se ha bloqueado en esta sesion.
    block_count: u32,
}

#[derive(Default)]
pub struct Guard {
    inner: Mutex<Inner>,
}

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub blocked: bool,
    pub auth_failures: u32,
    pub net_failures: u32,
    pub max_attempts: u32,
    pub last_error: String,
    pub block_reason: String,
    pub block_count: u32,
}

impl Guard {
    pub fn snapshot(&self) -> Status {
        let g = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        Status {
            blocked: g.blocked,
            auth_failures: g.auth_failures,
            net_failures: g.net_failures,
            max_attempts: MAX_ATTEMPTS,
            last_error: g.last_error.clone(),
            block_reason: g.block_reason.clone(),
            block_count: g.block_count,
        }
    }

    /// Deja pasar un intento, o devuelve el motivo del bloqueo.
    pub fn check(&self) -> Result<(), String> {
        let mut g = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        if g.blocked {
            return Err(format!(
                "SQL Server en pausa: no se reintenta el login para no bloquear la cuenta. {} Pulsa 'Reintentar ahora' en Ajustes.",
                g.block_reason
            ));
        }
        // Anti-rafaga: solo tras un fallo. Con contrasena correcta no se
        // ralentiza nada porque no hay riesgo de bloqueo de la cuenta.
        if g.last_failed {
            if let Some(last) = g.last_attempt {
                let wait = MIN_INTERVAL.saturating_sub(last.elapsed());
                if !wait.is_zero() {
                    return Err(format!(
                        "Espera {}s antes de volver a intentar conectar (proteccion anti-bloqueo).",
                        wait.as_secs() + 1
                    ));
                }
            }
        }
        g.last_attempt = Some(Instant::now());
        Ok(())
    }

    /// Registra el resultado de un intento real de autenticacion.
    pub fn record(&self, res: &Result<(), String>) {
        let mut g = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        match res {
            Ok(()) => {
                g.auth_failures = 0;
                g.net_failures = 0;
                g.last_error.clear();
                g.last_failed = false;
            }
            Err(e) => {
                g.last_error = e.clone();
                g.last_failed = true;
                if is_auth_error(e) {
                    g.auth_failures += 1;
                    g.net_failures = 0;
                    if g.auth_failures >= MAX_ATTEMPTS && !g.blocked {
                        g.blocked = true;
                        g.block_count += 1;
                        g.block_reason = format!(
                            "{} intentos fallidos con usuario/contrasena. ultimo error: {}",
                            g.auth_failures, e
                        );
                    }
                } else {
                    g.net_failures += 1;
                }
            }
        }
    }

    /// Reinicia el circuito (lo dispara el usuario desde Ajustes).
    pub fn reset(&self) {
        let mut g = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        *g = Inner::default();
    }

    /// Solo para pruebas.
    pub fn force_block(&self, reason: &str) {
        let mut g = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        g.blocked = true;
        g.block_reason = reason.to_string();
        g.block_count += 1;
    }
}

/// Detecta si el error es de credenciales (no de red).
pub fn is_auth_error(msg: &str) -> bool {
    let m = msg.to_lowercase();
    const MARCAS: [&str; 10] = [
        "18456",
        "password did not match",
        "password no coinciden",
        "login failed",
        "error de autenticaci",
        "access denied",
        "denegado el acceso",
        "no puedo abrir la base de datos",
        "4060",
        "1845",
    ];
    MARCAS.iter().any(|k| m.contains(k))
}

#[cfg(test)]
mod tests {
    use super::*;

    const AUTH_ERR: &str = "Error de autenticación con 10.96.16.114: Login failed for user 'HInsightUser'. Reason: Password did not match that for the login provided.";
    const NET_ERR: &str = "Timeout conectando a 10.96.16.114 (10s)";

    fn dormir(millis: u64) {
        std::thread::sleep(Duration::from_millis(millis));
    }

    #[test]
    fn detecta_errores_de_autenticacion() {
        assert!(is_auth_error(AUTH_ERR));
        assert!(is_auth_error("Login failed for user 'x'"));
        assert!(is_auth_error("error 18456"));
        assert!(!is_auth_error(NET_ERR));
        assert!(!is_auth_error("No se pudo conectar a 10.96.16.114"));
    }

    #[test]
    fn bloquea_tras_tres_fallos_de_autenticacion() {
        let g = Guard::default();
        let err = Err(AUTH_ERR.to_string());
        for i in 1..=MAX_ATTEMPTS {
            g.check().expect("debe pasar antes de bloquear");
            g.record(&err);
            if i < MAX_ATTEMPTS {
                assert!(!g.snapshot().blocked, "no debe bloquear en el intento {}", i);
            }
            dormir((MIN_INTERVAL.as_millis() as u64) + 20);
        }
        assert!(g.snapshot().blocked, "debe quedar bloqueado");
        // Cuarto intento rechazado sin tocar la red.
        let e = g.check().unwrap_err();
        assert!(e.contains("no se reintenta"));
    }

    #[test]
    fn un_exito_reinicia_el_contador() {
        let g = Guard::default();
        dormir((MIN_INTERVAL.as_millis() as u64) + 20);
        g.check().ok();
        g.record(&Err(AUTH_ERR.to_string()));
        dormir((MIN_INTERVAL.as_millis() as u64) + 20);
        g.record(&Ok(()));
        assert_eq!(g.snapshot().auth_failures, 0);
        assert!(!g.snapshot().blocked);
    }

    #[test]
    fn fallos_de_red_no_bloquean() {
        let g = Guard::default();
        for _ in 0..6 {
            if g.check().is_ok() {
                g.record(&Err(NET_ERR.to_string()));
                dormir((MIN_INTERVAL.as_millis() as u64) + 20);
            }
        }
        let s = g.snapshot();
        assert!(!s.blocked, "sin red no se debe bloquear la cuenta");
        assert!(s.net_failures > 0);
    }

    #[test]
    fn limita_la_rafaga_solo_tras_un_fallo() {
        let g = Guard::default();
        // Con contrasena correcta, varias llamadas seguidas deben pasar:
        // verificar tablas e imagenes por item hacen rafagas legitimas.
        for _ in 0..20 {
            assert!(g.check().is_ok(), "rafaga con exito no debe frenarse");
            g.record(&Ok(()));
        }
        // Tras un fallo, el siguiente intento inmediato se frena.
        g.record(&Err(AUTH_ERR.to_string()));
        let e = g.check().unwrap_err();
        assert!(e.contains("Espera"));
    }

    #[test]
    fn tres_fallos_seguidos_bloquean_rapido() {
        // Con el intervalo real de 5s, los 3 intentos caben en ~10s.
        let g = Guard::default();
        for i in 0..MAX_ATTEMPTS {
            g.check().unwrap_or_else(|e| panic!("intento {} rechazado: {}", i + 1, e));
            g.record(&Err(AUTH_ERR.to_string()));
            dormir((MIN_INTERVAL.as_millis() as u64) + 20);
        }
        assert!(g.snapshot().blocked);
    }

    #[test]
    fn reset_manual_libera_el_bloqueo() {
        let g = Guard::default();
        g.force_block("prueba");
        assert!(g.check().is_err());
        g.reset();
        let s = g.snapshot();
        assert!(!s.blocked);
        assert_eq!(s.block_count, 0);
        assert_eq!(s.auth_failures, 0);
        assert!(g.check().is_ok(), "tras reset debe volver a intentar");
    }
}

