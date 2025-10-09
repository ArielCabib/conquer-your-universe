import type { MouseEventHandler } from "react";
import { ORBIT_03 } from "../../constants";

interface ControlModalProps {
  isActive: boolean;
  onClose: MouseEventHandler<HTMLButtonElement>;
  pauseStatusText: string;
  onRestart: MouseEventHandler<HTMLButtonElement>;
  onSave: MouseEventHandler<HTMLButtonElement>;
  onOpenFile: MouseEventHandler<HTMLButtonElement>;
}

export function ControlModal({
  isActive,
  onClose,
  pauseStatusText,
  onRestart,
  onSave,
  onOpenFile,
}: ControlModalProps) {
  if (!isActive) {
    return null;
  }

  return (
    <div
      style={{
        position: "fixed",
        inset: 0,
        backgroundColor: "rgba(0,0,0,0.65)",
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        padding: "1.5rem",
        zIndex: 100,
      }}
    >
      <div
        style={{
          background: "rgba(28, 18, 14, 0.96)",
          border: "1px solid rgba(248, 225, 200, 0.35)",
          borderRadius: "1rem",
          padding: "1.5rem",
          width: "min(90vw, 420px)",
          display: "flex",
          flexDirection: "column",
          gap: "1.25rem",
          color: ORBIT_03,
        }}
      >
        <div
          style={{
            display: "flex",
            alignItems: "center",
            justifyContent: "space-between",
            gap: "0.75rem",
          }}
        >
          <h2
            style={{
              margin: 0,
              fontSize: "1.35rem",
              letterSpacing: "0.08em",
              textTransform: "uppercase",
              fontFamily: "Orbitron, 'Trebuchet MS', sans-serif",
              color: ORBIT_03,
            }}
          >
            Command Center
          </h2>
          <button
            type="button"
            onClick={onClose}
            style={{
              border: "none",
              background: "rgba(0,0,0,0.25)",
              color: "inherit",
              padding: "0.35rem 0.75rem",
              borderRadius: "0.5rem",
              cursor: "pointer",
              fontSize: "0.9rem",
              letterSpacing: "0.06em",
            }}
          >
            Close
          </button>
        </div>
        <p
          style={{
            margin: 0,
            textAlign: "left",
            fontSize: "0.95rem",
            letterSpacing: "0.04em",
            color: "rgba(248, 225, 200, 0.85)",
          }}
        >
          {pauseStatusText}
        </p>
        <div
          style={{
            display: "flex",
            flexDirection: "column",
            gap: "0.75rem",
          }}
        >
          <button
            type="button"
            onClick={onRestart}
            style={{
              padding: "0.75rem 1rem",
              borderRadius: "0.75rem",
              border: "1px solid rgba(248,225,200,0.35)",
              background: "rgba(0,0,0,0.35)",
              color: ORBIT_03,
              fontSize: "1rem",
              letterSpacing: "0.06em",
              cursor: "pointer",
            }}
          >
            Restart Game
          </button>
          <button
            type="button"
            onClick={onSave}
            style={{
              padding: "0.75rem 1rem",
              borderRadius: "0.75rem",
              border: "1px solid rgba(248,225,200,0.35)",
              background: "rgba(0,0,0,0.35)",
              color: ORBIT_03,
              fontSize: "1rem",
              letterSpacing: "0.06em",
              cursor: "pointer",
            }}
          >
            Save Game
          </button>
          <button
            type="button"
            onClick={onOpenFile}
            style={{
              padding: "0.75rem 1rem",
              borderRadius: "0.75rem",
              border: "1px solid rgba(248,225,200,0.35)",
              background: "rgba(0,0,0,0.35)",
              color: ORBIT_03,
              fontSize: "1rem",
              letterSpacing: "0.06em",
              cursor: "pointer",
            }}
          >
            Load Game
          </button>
        </div>
      </div>
    </div>
  );
}
