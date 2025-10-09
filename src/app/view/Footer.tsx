import type { CSSProperties } from "react";
import { ORBIT_02, ORBIT_03 } from "../../constants";

interface FooterInfoProps {
  minLifespanMs: number;
  maxLifespanMs: number;
}

function formatSeconds(ms: number): string {
  const seconds = ms / 1000;
  return Number.isInteger(seconds) ? `${seconds}` : seconds.toFixed(1);
}

export function FooterInfo({ minLifespanMs, maxLifespanMs }: FooterInfoProps) {
  const footerStyle: CSSProperties = {
    width: "100%",
    display: "flex",
    justifyContent: "center",
  };

  const contentStyle: CSSProperties = {
    marginTop: "1.5rem",
    padding: "0.75rem 1.25rem",
    borderTop: `1px solid ${ORBIT_02}`,
    color: ORBIT_03,
    fontSize: "0.95rem",
    width: "min(80vw, 540px)",
    maxWidth: "600px",
    display: "flex",
    justifyContent: "space-between",
    alignItems: "center",
    gap: "0.75rem",
  };

  return (
    <footer style={footerStyle}>
      <div style={contentStyle}>
        <span>Settler lifespan range</span>
        <span>
          {formatSeconds(minLifespanMs)}s – {formatSeconds(maxLifespanMs)}s
        </span>
      </div>
    </footer>
  );
}
