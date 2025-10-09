import { MouseEventHandler } from "react";
import { ORBIT_03, ORBIT_05 } from "../../constants";

interface HeaderSectionProps {
  onOpenSettings: MouseEventHandler<HTMLButtonElement>;
}

export function HeaderSection({ onOpenSettings }: HeaderSectionProps) {
  return (
    <div
      style={{
        display: "flex",
        alignItems: "center",
        justifyContent: "space-between",
        maxWidth: "600px",
        gap: "1rem",
      }}
    >
      <div>
        <div style={{ display: "flex", alignItems: "center" }}>
          <span />
          <button
            type="button"
            aria-label="Open settings"
            onClick={onOpenSettings}
            style={{
              display: "inline-flex",
              alignItems: "center",
              justifyContent: "center",
              width: "2.25rem",
              height: "2.25rem",
              borderRadius: "50%",
              border: "1px solid rgba(248, 225, 200, 0.4)",
              background: "rgba(0,0,0,0.35)",
              color: ORBIT_03,
              cursor: "pointer",
              marginLeft: "auto",
              marginRight: 0,
            }}
          >
            <svg width="28" height="28" viewBox="0 0 24 24" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
              <path d="M11.983 2a1 1 0 0 1 .993.883l.007.117v1.19a5.52 5.52 0 0 1 1.45.6l.84-.84a1 1 0 0 1 1.497 1.32l-.083.094-.84.84a5.52 5.52 0 0 1 .6 1.451h1.19a1 1 0 0 1 .117 1.993l-.117.007h-1.19a5.52 5.52 0 0 1-.6 1.45l.84.841a1 1 0 0 1-1.32 1.497l-.094-.083-.84-.84a5.52 5.52 0 0 1-1.451.6v1.19a1 1 0 0 1-1.993.117l-.007-.117v-1.19a5.52 5.52 0 0 1-1.45-.6l-.84.84a1 1 0 0 1-1.497-1.32l.083-.094.84-.84a5.52 5.52 0 0 1-.6-1.451h-1.19a1 1 0 0 1-.117-1.993l.117-.007h1.19a5.52 5.52 0 0 1 .6-1.45l-.84-.841a1 1 0 0 1 1.32-1.497l.094.083.84.84a5.52 5.52 0 0 1 1.451-.6v-1.19A1 1 0 0 1 11.983 2Zm.017 5a3 3 0 1 0 0 6a3 3 0 0 0 0-6Z" />
            </svg>
          </button>
        </div>
        <h1
          style={{
            color: ORBIT_03,
            fontFamily: "Orbitron, 'Trebuchet MS', sans-serif",
            fontSize: "clamp(2.5rem, 3vw, 3.5rem)",
            letterSpacing: "0.12em",
            textTransform: "uppercase",
            margin: 0,
          }}
        >
          Your Planet
        </h1>
        <h3
          style={{
            color: ORBIT_05,
            fontFamily: "'Trebuchet MS', sans-serif",
            fontSize: "clamp(1rem, 1vw, 1rem)",
            letterSpacing: "0.05em",
            margin: "0.25rem 0 0 0",
          }}
        >
          Click around and find out.
        </h3>
      </div>
    </div>
  );
}
