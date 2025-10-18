import { useEffect, useRef } from "react";
import type { MouseEventHandler, RefObject, TouchEventHandler } from "react";
import { ContextMenuState } from "../types";

const LONG_PRESS_DURATION_MS = 500;

interface CanvasAreaProps {
  canvasRef: RefObject<HTMLCanvasElement>;
  onClick: MouseEventHandler<HTMLCanvasElement>;
  onContextMenu: MouseEventHandler<HTMLCanvasElement>;
  isPaused: boolean;
  isResearchViewActive: boolean;
  contextMenuState: ContextMenuState | null;
  onBuildHouse: MouseEventHandler<HTMLButtonElement>;
  canBuildHouse: boolean;
  onBuildFarm: MouseEventHandler<HTMLButtonElement>;
  canBuildFarm: boolean;
  onBuildHarvester: MouseEventHandler<HTMLButtonElement>;
  canBuildHarvester: boolean;
  onBuildMarket: MouseEventHandler<HTMLButtonElement>;
  canBuildMarket: boolean;
  onBuildResearcher: MouseEventHandler<HTMLButtonElement>;
  canBuildResearcher: boolean;
}

export function CanvasArea({
  canvasRef,
  onClick,
  onContextMenu,
  isPaused,
  isResearchViewActive,
  contextMenuState,
  onBuildHouse,
  canBuildHouse,
  onBuildFarm,
  canBuildFarm,
  onBuildHarvester,
  canBuildHarvester,
  onBuildMarket,
  canBuildMarket,
  onBuildResearcher,
  canBuildResearcher,
}: CanvasAreaProps) {
  const longPressTimeoutRef = useRef<number | null>(null);
  const lastTouchRef = useRef<{ identifier: number; clientX: number; clientY: number } | null>(
    null,
  );

  const longPressTriggeredRef = useRef(false);

  const clearLongPress = (options?: { preserveTrigger?: boolean }) => {
    if (longPressTimeoutRef.current !== null) {
      window.clearTimeout(longPressTimeoutRef.current);
      longPressTimeoutRef.current = null;
    }
    lastTouchRef.current = null;
    if (!options?.preserveTrigger) {
      longPressTriggeredRef.current = false;
    }
  };

  useEffect(() => {
    return () => {
      clearLongPress();
    };
  }, []);

  const scheduleLongPress = (touch: {
    identifier: number;
    clientX: number;
    clientY: number;
  }) => {
    const canvas = canvasRef.current;
    if (!canvas) {
      return;
    }

    longPressTriggeredRef.current = false;

    lastTouchRef.current = {
      identifier: touch.identifier,
      clientX: touch.clientX,
      clientY: touch.clientY,
    };

    if (longPressTimeoutRef.current !== null) {
      window.clearTimeout(longPressTimeoutRef.current);
    }

    longPressTimeoutRef.current = window.setTimeout(() => {
      if (!lastTouchRef.current) {
        return;
      }

      longPressTriggeredRef.current = true;

      canvas.dispatchEvent(
        new MouseEvent("contextmenu", {
          bubbles: true,
          cancelable: true,
          clientX: lastTouchRef.current.clientX,
          clientY: lastTouchRef.current.clientY,
        }),
      );

      clearLongPress({ preserveTrigger: true });
    }, LONG_PRESS_DURATION_MS);
  };

  const handleTouchStart: TouchEventHandler<HTMLCanvasElement> = (event) => {
    if (event.touches.length !== 1) {
      clearLongPress();
      return;
    }

    scheduleLongPress(event.touches[0]);
  };

  const handleTouchMove: TouchEventHandler<HTMLCanvasElement> = (event) => {
    if (!lastTouchRef.current) {
      return;
    }

    const touch = Array.from(event.touches).find(
      (t) => t.identifier === lastTouchRef.current?.identifier,
    );

    if (!touch) {
      clearLongPress();
      return;
    }

    lastTouchRef.current = {
      identifier: touch.identifier,
      clientX: touch.clientX,
      clientY: touch.clientY,
    };
  };

  const handleTouchEnd: TouchEventHandler<HTMLCanvasElement> = (event) => {
    if (longPressTriggeredRef.current) {
      event.preventDefault();
      event.stopPropagation();
    }

    clearLongPress();
  };

  const pausedOverlay = isPaused && !isResearchViewActive ? (
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

  if (canBuildResearcher) {
    menuActions.push({
      key: "researcher",
      label: "Build Researcher",
      onClick: onBuildResearcher,
    });
  }

  const buttonBaseClass =
    "w-full rounded-lg px-3 py-2 text-left font-trebuchet text-[0.95rem] tracking-[0.04em] transition-colors duration-150 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-orbit-04 cursor-pointer bg-transparent text-orbit-03 hover:bg-orbit-04 hover:text-orbit-01";

  const contextMenu = !isResearchViewActive && contextMenuState && menuActions.length > 0 ? (
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

  const canvasInteractionClass = isResearchViewActive
    ? "cursor-default pointer-events-none"
    : isPaused
      ? "cursor-not-allowed pointer-events-none"
      : "cursor-pointer";

  const canvasOpacityClass = isResearchViewActive ? "opacity-0" : "opacity-100";

  const canvasClass = `h-auto w-full max-w-[600px] touch-manipulation transition-opacity duration-500 ease-out ${canvasInteractionClass} ${canvasOpacityClass}`;

  const researchOverlay = (
    <div
      className={`absolute inset-0 flex items-center justify-center bg-panel transition-opacity duration-500 ease-out ${
        isResearchViewActive ? "opacity-100 pointer-events-auto" : "opacity-0 pointer-events-none"
      }`}
      aria-hidden={!isResearchViewActive}
    />
  );

  return (
    <div className="relative w-[min(80vw,540px)] max-w-[600px]">
      <canvas
        ref={canvasRef}
        width={600}
        height={480}
        className={canvasClass}
        onClick={isResearchViewActive ? undefined : onClick}
        onContextMenu={isResearchViewActive ? undefined : onContextMenu}
        onTouchStart={isResearchViewActive ? undefined : handleTouchStart}
        onTouchMove={isResearchViewActive ? undefined : handleTouchMove}
        onTouchEnd={isResearchViewActive ? undefined : handleTouchEnd}
        onTouchCancel={isResearchViewActive ? undefined : handleTouchEnd}
      >
        Your browser does not support HTML canvas.
      </canvas>
      {researchOverlay}
      {pausedOverlay}
      {contextMenu}
    </div>
  );
}
