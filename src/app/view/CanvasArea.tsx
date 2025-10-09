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
  onBuildHarvester: MouseEventHandler<HTMLButtonElement>;
  canBuildHarvester: boolean;
  onBuildMarket: MouseEventHandler<HTMLButtonElement>;
  canBuildMarket: boolean;
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
  onBuildHarvester,
  canBuildHarvester,
  onBuildMarket,
  canBuildMarket,
}: CanvasAreaProps) {
  const pausedOverlay = isPaused ? (
    <div className="pointer-events-none absolute inset-0 flex items-center justify-center bg-overlay text-[1.1rem] font-orbitron uppercase tracking-[0.08em] text-orbit-03">
      Paused
    </div>
  ) : null;

  const menuActions: Array<{
    key: string;
    label: string;
    onClick: MouseEventHandler<HTMLButtonElement>;
  }> = [];

  if (canBuildHouse) {
    menuActions.push({ key: "house", label: "Build House", onClick: onBuildHouse });
  }

  if (canBuildFarm) {
    menuActions.push({ key: "farm", label: "Build Farm", onClick: onBuildFarm });
  }

  if (canBuildHarvester) {
    menuActions.push({ key: "harvester", label: "Build Harvester", onClick: onBuildHarvester });
  }

  if (canBuildMarket) {
    menuActions.push({ key: "market", label: "Build Market", onClick: onBuildMarket });
  }

  const buttonBaseClass =
    "w-full rounded-lg px-3 py-2 text-left font-trebuchet text-[0.95rem] tracking-[0.04em] transition-colors duration-150 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-orbit-04 cursor-pointer bg-transparent text-orbit-03 hover:bg-orbit-04 hover:text-orbit-01";

  const contextMenu = contextMenuState && menuActions.length > 0 ? (
    <div
      className="absolute z-10 min-w-[160px] -translate-x-1/2 transform rounded-xl border border-orbit-03/40 bg-menu p-1 shadow-context-menu"
      style={{
        left: `${contextMenuState.offsetX.toFixed(2)}px`,
        top: `${contextMenuState.offsetY.toFixed(2)}px`,
      }}
    >
      {menuActions.map((action, index) => (
        <button
          key={action.key}
          type="button"
          onClick={action.onClick}
          className={`${index > 0 ? "mt-1 " : ""}${buttonBaseClass}`}
        >
          {action.label}
        </button>
      ))}
    </div>
  ) : null;

  return (
    <div className="relative w-[min(80vw,540px)] max-w-[600px]">
      <canvas
        ref={canvasRef}
        width={600}
        height={480}
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
