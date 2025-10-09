import type { MouseEventHandler, RefObject } from "react";
import { ContextMenuState } from "../types";

interface CanvasAreaProps {
  canvasRef: RefObject<HTMLCanvasElement>;
  onClick: MouseEventHandler<HTMLCanvasElement>;
  onContextMenu: MouseEventHandler<HTMLCanvasElement>;
  isPaused: boolean;
  contextMenuState: ContextMenuState | null;
  onBuildHouse: MouseEventHandler<HTMLButtonElement>;
  canBuildHouse: boolean;
  onBuildFarm: MouseEventHandler<HTMLButtonElement>;
  canBuildFarm: boolean;
}

export function CanvasArea({
  canvasRef,
  onClick,
  onContextMenu,
  isPaused,
  contextMenuState,
  onBuildHouse,
  canBuildHouse,
  onBuildFarm,
  canBuildFarm,
}: CanvasAreaProps) {
  const pausedOverlay = isPaused ? (
    <div className="pointer-events-none absolute inset-0 flex items-center justify-center bg-overlay text-[1.1rem] font-orbitron uppercase tracking-[0.08em] text-orbit-03">
      Paused
    </div>
  ) : null;

  const contextMenu = contextMenuState ? (
    <div
      className="absolute z-10 min-w-[160px] -translate-x-1/2 transform rounded-xl border border-orbit-03/40 bg-menu p-1 shadow-context-menu"
      style={{
        left: `${contextMenuState.offsetX.toFixed(2)}px`,
        top: `${contextMenuState.offsetY.toFixed(2)}px`,
      }}
    >
      <button
        type="button"
        onClick={onBuildHouse}
        disabled={!canBuildHouse}
        className={`w-full rounded-lg px-3 py-2 text-left font-trebuchet text-[0.95rem] tracking-[0.04em] transition-colors duration-150 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-orbit-04 ${
          canBuildHouse
            ? "cursor-pointer bg-transparent text-orbit-03 hover:bg-orbit-04 hover:text-orbit-01"
            : "cursor-not-allowed bg-transparent text-orbit-03/50"
        }`}
      >
        Build House
      </button>
      <button
        type="button"
        onClick={onBuildFarm}
        disabled={!canBuildFarm}
        title={canBuildFarm ? undefined : "Requires at least 10 settlers"}
        className={`mt-1 w-full rounded-lg px-3 py-2 text-left font-trebuchet text-[0.95rem] tracking-[0.04em] transition-colors duration-150 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-orbit-04 ${
          canBuildFarm
            ? "cursor-pointer bg-transparent text-orbit-03 hover:bg-orbit-04 hover:text-orbit-01"
            : "cursor-not-allowed bg-transparent text-orbit-03/50"
        }`}
      >
        Build Farm
      </button>
    </div>
  ) : null;

  return (
    <div className="relative w-[min(80vw,540px)] max-w-[600px]">
      <canvas
        ref={canvasRef}
        width={600}
        height={400}
        className={`h-auto w-full max-w-[600px] touch-manipulation ${
          isPaused ? "cursor-not-allowed pointer-events-none" : "cursor-pointer"
        }`}
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
