import { useEffect, useMemo, useState } from "react";

const COLORES = [
  "#f26c25", "#f5d90a", "#2bb089", "#38bdf8", "#6366f1",
  "#a855f7", "#e63946", "#ffffff", "#00e5ff",
];

interface Piece {
  id: number;
  left: number;
  delay: number;
  duration: number;
  color: string;
  size: number;
  rotate: number;
  type: "rect" | "circle" | "strip";
  drift: number;
}

interface Burst {
  id: number;
  cx: number;
  cy: number;
  delay: number;
  scale: number;
}

/**
 * Pantalla de kit completo. Se lee a distancia: texto enorme, ondas de
 * choque, chispas y confeti.
 */
export function Celebration({ serie, visible }: { serie: string; visible: boolean }) {
  const pieces = useMemo<Piece[]>(
    () =>
      Array.from({ length: 260 }, (_, i) => {
        const r = Math.random();
        return {
          id: i,
          left: Math.random() * 100,
          delay: Math.random() * 2.6,
          duration: 2.8 + Math.random() * 2.6,
          color: COLORES[Math.floor(Math.random() * COLORES.length)],
          size: 7 + Math.random() * 14,
          rotate: Math.random() * 360,
          type: r > 0.86 ? "strip" : r > 0.55 ? "circle" : "rect",
          drift: (Math.random() - 0.5) * 160,
        };
      }),
    [visible],
  );

  const bursts = useMemo<Burst[]>(
    () =>
      Array.from({ length: 14 }, (_, i) => ({
        id: i,
        cx: 8 + Math.random() * 84,
        cy: 12 + Math.random() * 66,
        delay: Math.random() * 1.2,
        scale: 0.7 + Math.random() * 0.7,
      })),
    [visible],
  );

  const [dismissed, setDismissed] = useState(false);
  useEffect(() => {
    if (visible) setDismissed(false);
  }, [visible]);

  if (!visible || dismissed) return null;

  return (
    <div className="celebration" onClick={() => setDismissed(true)}>
      <div className="celebrate-rays" />
      <div className="celebrate-aurora" />
      <div className="celebrate-shine" />

      {bursts.map((b) => (
        <div
          key={b.id}
          className="firework"
          style={{
            left: `${b.cx}%`,
            top: `${b.cy}%`,
            animationDelay: `${b.delay}s`,
            ["--s" as string]: `${b.scale}`,
          }}
        >
          <span className="firework-core" />
          {Array.from({ length: 18 }, (_, i) => {
            const angle = (i / 18) * Math.PI * 2;
            return (
              <span
                key={i}
                className="firework-spark"
                style={{
                  ["--dx" as string]: `${Math.cos(angle) * 150}px`,
                  ["--dy" as string]: `${Math.sin(angle) * 150}px`,
                  background: COLORES[i % COLORES.length],
                  animationDelay: `${b.delay + Math.random() * 0.2}s`,
                }}
              />
            );
          })}
        </div>
      ))}

      <div className="celebrate-shock">
        <span />
        <span />
        <span />
      </div>

      {pieces.map((p) => (
        <span
          key={p.id}
          className={`confetti confetti-${p.type}`}
          style={{
            left: `${p.left}%`,
            width: p.size,
            height: p.type === "circle" ? p.size : p.size * 0.38,
            background: p.color,
            animationDelay: `${p.delay}s`,
            animationDuration: `${p.duration}s`,
            ["--r" as string]: `${p.rotate}deg`,
            ["--drift" as string]: `${p.drift}px`,
          }}
        />
      ))}

      <div className="celebrate-content">
        <div className="celebrate-badge">
          <span className="celebrate-ring" />
          <span className="celebrate-star">★</span>
        </div>
        <h2 className="celebrate-title">¡NÚMERO DE SERIE!</h2>
        <div className="celebrate-serie">{serie}</div>
        <div className="celebrate-stamp">COMPLETO</div>
        <div className="celebrate-next">
          <span className="celebrate-next-k">SIGUIENTE PASO</span>
          🎫 Escanea tu <strong>gafete</strong> para registrar
        </div>
      </div>
    </div>
  );
}
