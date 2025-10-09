import { useState, type CSSProperties, type MouseEventHandler, type RefObject } from "react";
import { ContextMenuState } from "../types";
import { ORBIT_01, ORBIT_03, ORBIT_04 } from "../../constants";

interface CanvasAreaProps {
  canvasRef: RefObject<HTMLCanvasElement>;
  canvasStyle: CSSProperties;
  onClick: MouseEventHandler<HTMLCanvasElement>;
  onContextMenu: MouseEventHandler<HTMLCanvasElement>;
  isPaused: boolean;
  contextMenuState: ContextMenuState | null;
  onBuildHouse: MouseEventHandler<HTMLButtonElement>;
  canBuildHouse: boolean;
}

export function CanvasArea({
  canvasRef,
  canvasStyle,
  onClick,
  onContextMenu,
  isPaused,
  contextMenuState,
  onBuildHouse,
  canBuildHouse,
}: CanvasAreaProps) {
  const [isBuildActionHovered, setIsBuildActionHovered] = useState(false);
  const pausedOverlay = isPaused ? (
    <div
      style={{
        position: "absolute",
        inset: 0,
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        background: "rgba(18, 11, 8, 0.55)",
        color: ORBIT_03,
        fontFamily: "Orbitron, 'Trebuchet MS', sans-serif",
        letterSpacing: "0.08em",
        textTransform: "uppercase",
        fontSize: "1.1rem",
        pointerEvents: "none",
      }}
    >
      Paused
    </div>
  ) : null;

  const contextMenu = contextMenuState ? (
    <div
      style={{
        position: "absolute",
        left: `${contextMenuState.offsetX.toFixed(2)}px`,
        top: `${contextMenuState.offsetY.toFixed(2)}px`,
        transform: "translate(-50%, 0)",
        minWidth: "160px",
        background: "rgba(28, 18, 14, 0.94)",
        border: "1px solid rgba(248, 225, 200, 0.4)",
        borderRadius: "0.6rem",
        boxShadow: "0 12px 24px rgba(0, 0, 0, 0.35)",
        padding: "0.35rem",
        zIndex: 10,
      }}
    >
      <button
        type="button"
        onClick={onBuildHouse}
        disabled={!canBuildHouse}
        style={{
          width: "100%",
          textAlign: "left",
          padding: "0.5rem 0.75rem",
          border: "none",
          background: isBuildActionHovered ? ORBIT_04 : "transparent",
          color: isBuildActionHovered ? ORBIT_01 : ORBIT_03,
          fontFamily: "'Trebuchet MS', sans-serif",
          fontSize: "0.95rem",
          letterSpacing: "0.04em",
          borderRadius: "0.5rem",
          cursor: canBuildHouse ? "pointer" : "not-allowed",
          transition: "background 120ms ease, color 120ms ease",
        }}
        onMouseEnter={() => setIsBuildActionHovered(true)}
        onMouseLeave={() => setIsBuildActionHovered(false)}
      >
        Build House
      </button>
    </div>
  ) : null;

  return (
    <div style={{ position: "relative", width: "min(80vw, 540px)", maxWidth: "600px" }}>
      <canvas
        ref={canvasRef}
        width={600}
        height={400}
        style={canvasStyle}
        onClick={onClick}
        onContextMenu={onContextMenu}
      >
        Your browser does not support HTML canvas.
      </canvas>
      {pausedOverlay}
      {contextMenu}
    </div>
  );
}
