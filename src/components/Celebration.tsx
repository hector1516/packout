import { useMemo, useState, useEffect } from "react";

const COLORS = ["#f26c25", "#0165a4", "#2b58b8", "#f5d90a", "#2bb089", "#e63946", "#8f2bd7", "#ffffff"];

interface Piece {
  id: number;
  left: number;
  delay: number;
  duration: number;
  color: string;
  size: number;
  rotate: number;
  type: "rect" | "circle";
}

interface Burst {
  id: number;
  cx: number;
  cy: number;
}

export function Celebration({ serie, visible }: { serie: string; visible: boolean }) {
  const pieces = useMemo<Piece[]>(
    () =>
      Array.from({ length: 160 }, (_, i) => ({
        id: i,
        left: Math.random() * 100,
        delay: Math.random() * 2.2,
        duration: 2.4 + Math.random() * 2.4,
        color: COLORS[Math.floor(Math.random() * COLORS.length)],
        size: 6 + Math.random() * 10,
        rotate: Math.random() * 360,
        type: Math.random() > 0.5 ? "rect" : "circle",
      })),
    [visible],
  );

  const bursts = useMemo<Burst[]>(
    () =>
      Array.from({ length: 12 }, (_, i) => ({
        id: i,
        cx: 10 + Math.random() * 80,
        cy: 15 + Math.random() * 60,
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
      <div className="celebrate-shine" />
      {bursts.map((b) => (
        <div key={b.id} className="firework" style={{ left: `${b.cx}%`, top: `${b.cy}%` }}>
          {Array.from({ length: 14 }, (_, i) => {
            const angle = (i / 14) * Math.PI * 2;
            return (
              <span
                key={i}
                className="firework-spark"
                style={{
                  "--dx": `${Math.cos(angle) * 120}px`,
                  "--dy": `${Math.sin(angle) * 120}px`,
                  background: COLORS[i % COLORS.length],
                } as React.CSSProperties}
              />
            );
          })}
        </div>
      ))}
      {pieces.map((p) => (
        <span
          key={p.id}
          className={`confetti confetti-${p.type}`}
          style={{
            left: `${p.left}%`,
            width: p.size,
            height: p.type === "circle" ? p.size : p.size * 0.4,
            background: p.color,
            animationDelay: `${p.delay}s`,
            animationDuration: `${p.duration}s`,
            ["--r" as string]: `${p.rotate}deg`,
          }}
        />
      ))}
      <div className="celebrate-content">
        <div className="celebrate-badge">
          <span className="celebrate-star">★</span>
        </div>
        <h2 className="celebrate-title">Número de serie</h2>
        <div className="celebrate-serie">{serie}</div>
        <p className="celebrate-sub">Completo</p>
        <div className="celebrate-next">
          Ahora escanea tu gafete para terminar y pasar al siguiente equipo
        </div>
      </div>
    </div>
  );
}