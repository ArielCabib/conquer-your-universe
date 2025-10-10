import { useCallback, useEffect, useRef } from "react";
import type {
  MouseEvent as ReactMouseEvent,
  MouseEventHandler,
  PointerEvent as ReactPointerEvent,
  RefObject,
} from "react";
import { ContextMenuState } from "../types";

const LONG_PRESS_DURATION_MS = 500;
const LONG_PRESS_MOVE_THRESHOLD_PX = 10;
const LONG_PRESS_MOVE_THRESHOLD_SQUARED =
  LONG_PRESS_MOVE_THRESHOLD_PX * LONG_PRESS_MOVE_THRESHOLD_PX;

interface CanvasAreaProps {
  canvasRef: RefObject<HTMLCanvasElement>;
  onClick: MouseEventHandler<HTMLCanvasElement>;
  onContextMenu: MouseEventHandler<HTMLCanvasElement>;
  onLongPress: (clientX: number, clientY: number) => void;
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
  onLongPress,
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
  const longPressTimeoutRef = useRef<number | null>(null);
  const touchStartPositionRef = useRef<{ x: number; y: number } | null>(null);
  const hasLongPressTriggeredRef = useRef(false);
  const suppressClickRef = useRef(false);

  const clearLongPressTimeout = useCallback(() => {
    if (longPressTimeoutRef.current !== null) {
      window.clearTimeout(longPressTimeoutRef.current);
      longPressTimeoutRef.current = null;
    }
  }, []);

  const resetTouchTracking = useCallback(() => {
    touchStartPositionRef.current = null;
  }, []);

  useEffect(() => {
    return () => {
      clearLongPressTimeout();
      resetTouchTracking();
      hasLongPressTriggeredRef.current = false;
      suppressClickRef.current = false;
    };
  }, [clearLongPressTimeout, resetTouchTracking]);

  const handlePointerDown = useCallback(
    (event: ReactPointerEvent<HTMLCanvasElement>) => {
      if (event.pointerType === "mouse") {
        suppressClickRef.current = false;
        clearLongPressTimeout();
        resetTouchTracking();
        return;
      }

      touchStartPositionRef.current = { x: event.clientX, y: event.clientY };
      hasLongPressTriggeredRef.current = false;
      suppressClickRef.current = false;
      clearLongPressTimeout();

      longPressTimeoutRef.current = window.setTimeout(() => {
        if (!touchStartPositionRef.current) {
          return;
        }

        hasLongPressTriggeredRef.current = true;
        suppressClickRef.current = true;
        onLongPress(
          touchStartPositionRef.current.x,
          touchStartPositionRef.current.y,
        );
      }, LONG_PRESS_DURATION_MS);
    },
    [clearLongPressTimeout, onLongPress, resetTouchTracking],
  );

  const handlePointerMove = useCallback(
    (event: ReactPointerEvent<HTMLCanvasElement>) => {
      if (event.pointerType === "mouse") {
        return;
      }

      if (!touchStartPositionRef.current || hasLongPressTriggeredRef.current) {
        return;
      }

      const deltaX = event.clientX - touchStartPositionRef.current.x;
      const deltaY = event.clientY - touchStartPositionRef.current.y;
      const distanceSquared = deltaX * deltaX + deltaY * deltaY;

      if (distanceSquared > LONG_PRESS_MOVE_THRESHOLD_SQUARED) {
        clearLongPressTimeout();
        resetTouchTracking();
      }
    },
    [clearLongPressTimeout, resetTouchTracking],
  );

  const handlePointerEnd = useCallback(
    (event: ReactPointerEvent<HTMLCanvasElement>) => {
      if (event.pointerType === "mouse") {
        return;
      }

      clearLongPressTimeout();
      resetTouchTracking();
    },
    [clearLongPressTimeout, resetTouchTracking],
  );

  const handleCanvasClick = useCallback(
    (event: ReactMouseEvent<HTMLCanvasElement>) => {
      if (suppressClickRef.current) {
        event.preventDefault();
        event.stopPropagation();
        suppressClickRef.current = false;
        return;
      }

      onClick(event);
    },
    [onClick],
  );

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
    "w-full rounded-lg px-3 py-2 text-left font-trebuchet text-[0.95rem] tracking-[0.04em] transition-colors duration-150 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-orbit-04 cursor-pointer bg-transparent text-orbit-03 hover:bg-orbit-04 hover:text-orbit-01 disabled:cursor-not-allowed disabled:opacity-60";

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

  const quickActions = menuActions.length > 0 ? (
    <div className="mt-4 grid grid-cols-1 gap-2 sm:grid-cols-2">
      {menuActions.map((action) => (
        <button
          key={`${action.key}-quick`}
          type="button"
          onClick={action.onClick}
          disabled={isPaused}
          className={buttonBaseClass}
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
        onClick={handleCanvasClick}
        onContextMenu={onContextMenu}
        onPointerDown={handlePointerDown}
        onPointerMove={handlePointerMove}
        onPointerUp={handlePointerEnd}
        onPointerCancel={handlePointerEnd}
        onPointerLeave={handlePointerEnd}
      >
        Your browser does not support HTML canvas.
      </canvas>
      {pausedOverlay}
      {contextMenu}
      {quickActions}
    </div>
  );
}
