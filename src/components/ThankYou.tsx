import { useEffect, useMemo, useState } from "react";

const COLORES = ["#2bb089", "#38bdf8", "#f5d90a", "#ffffff", "#a855f7", "#f26c25"];

interface Spark {
  id: number;
  angle: number;
  dist: number;
  delay: number;
  color: string;
  size: number;
}

/**
 * Animacion de agradecimiento cuando el colaborador escanea su gafete
 * y el registro queda correcto.
 */
export function ThankYou({
  operador,
  serie,
  visible,
}: {
  operador: string;
  serie: string;
  visible: boolean;
}) {
  const sparks = useMemo<Spark[]>(
    () =>
      Array.from({ length: 60 }, (_, i) => ({
        id: i,
        angle: (i / 60) * Math.PI * 2 + Math.random() * 0.2,
        dist: 160 + Math.random() * 220,
        delay: Math.random() * 0.7,
        color: COLORES[i % COLORES.length],
        size: 5 + Math.random() * 8,
      })),
    [visible],
  );

  const [dismissed, setDismissed] = useState(false);
  useEffect(() => {
    if (visible) setDismissed(false);
  }, [visible]);

  if (!visible || dismissed) return null;

  return (
    <div className="celebration thankyou" onClick={() => setDismissed(true)}>
      <div className="thankyou-bg" />
      <div className="celebrate-rays" />

      <div className="thankyou-ringwrap">
        <span className="thankyou-ring r1" />
        <span className="thankyou-ring r2" />
        <span className="thankyou-ring r3" />
        <div className="thankyou-check">✓</div>
        {sparks.map((s) => (
          <span
            key={s.id}
            className="thankyou-spark"
            style={{
              ["--dx" as string]: `${Math.cos(s.angle) * s.dist}px`,
              ["--dy" as string]: `${Math.sin(s.angle) * s.dist}px`,
              width: s.size,
              height: s.size,
              background: s.color,
              animationDelay: `${s.delay}s`,
            }}
          />
        ))}
      </div>

      <div className="thankyou-content">
        <div className="thankyou-kicker">REGISTRO CORRECTO</div>
        <h2 className="thankyou-title">¡GRACIAS!</h2>
        <div className="thankyou-name">{operador}</div>
        <div className="thankyou-msg">
          Tu registro del número de serie <strong>{serie}</strong> quedó guardado
          correctamente
        </div>
        <div className="thankyou-next">
          ▶ Listo para el siguiente equipo
        </div>
      </div>
    </div>
  );
}
