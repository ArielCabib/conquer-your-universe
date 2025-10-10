import { Dispatch, MutableRefObject, RefObject, SetStateAction, useCallback } from "react";
import { ContextMenuState } from "../types";
import { currentTimeMs, pointWithinPlanet, randomRange } from "../helpers";
import { GameState, createSettlerState } from "../../types";

interface CanvasClickHandlerOptions {
  gameStateRef: MutableRefObject<GameState>;
  canvasRef: RefObject<HTMLCanvasElement>;
  isPaused: boolean;
  setContextMenuState: Dispatch<SetStateAction<ContextMenuState | null>>;
  aliveCount: number;
  setAliveCount: Dispatch<SetStateAction<number>>;
}

export function useCanvasClickHandler({
  gameStateRef,
  canvasRef,
  isPaused,
  setContextMenuState,
  aliveCount,
  setAliveCount,
}: CanvasClickHandlerOptions) {
  return useCallback(
    (event: React.MouseEvent<HTMLCanvasElement>) => {
      if (isPaused) {
        return;
      }

      setContextMenuState(null);

      const canvas = canvasRef.current;
      if (!canvas) {
        return;
      }

      const rect = canvas.getBoundingClientRect();
      const width = rect.width;
      const height = rect.height;
      if (width === 0 || height === 0) {
        return;
      }

      const scaleX = canvas.width / width;
      const scaleY = canvas.height / height;
      const clientX = event.clientX;
      const clientY = event.clientY;
      const canvasX = (clientX - rect.left) * scaleX;
      const canvasY = (clientY - rect.top) * scaleY;

      if (!pointWithinPlanet(canvasX, canvasY)) {
        return;
      }

      const state = gameStateRef.current;
      const baseCapacity = state.settlersBaseCapacity;
      const settlersPerHouse = state.settlersPerHouse;
      const houseCapacity = state.houses.length * settlersPerHouse;
      const settlersCapacityLimit = baseCapacity + houseCapacity;

      if (settlersCapacityLimit > 0 && aliveCount >= settlersCapacityLimit) {
        return;
      }

      const id = state.nextSettlerId;
      state.nextSettlerId += 1;

      const now = currentTimeMs();
      const lifespan = randomRange(state.settlerMinLifespanMs, state.settlerMaxLifespanMs);
      state.settlers.push(createSettlerState(id, canvasX, canvasY, now, lifespan));

      setAliveCount((prev) => prev + 1);
    },
    [aliveCount, canvasRef, gameStateRef, isPaused, setAliveCount, setContextMenuState],
  );
}

interface ContextMenuHandlerOptions {
  canvasRef: RefObject<HTMLCanvasElement>;
  isPaused: boolean;
  setContextMenuState: Dispatch<SetStateAction<ContextMenuState | null>>;
  hasContextMenuActions: () => boolean;
}

interface ContextMenuHandlerResult {
  handleContextMenuEvent: (event: React.MouseEvent<HTMLCanvasElement>) => void;
  openContextMenuAtPoint: (clientX: number, clientY: number) => void;
}

export function useContextMenuHandler({
  canvasRef,
  isPaused,
  setContextMenuState,
  hasContextMenuActions,
}: ContextMenuHandlerOptions): ContextMenuHandlerResult {
  const openContextMenuAtPoint = useCallback(
    (clientX: number, clientY: number) => {
      if (isPaused) {
        setContextMenuState(null);
        return;
      }

      const canvas = canvasRef.current;
      if (!canvas) {
        setContextMenuState(null);
        return;
      }

      const rect = canvas.getBoundingClientRect();
      const width = rect.width;
      const height = rect.height;
      if (width === 0 || height === 0) {
        setContextMenuState(null);
        return;
      }

      const scaleX = canvas.width / width;
      const scaleY = canvas.height / height;
      const canvasX = (clientX - rect.left) * scaleX;
      const canvasY = (clientY - rect.top) * scaleY;

      if (!pointWithinPlanet(canvasX, canvasY)) {
        setContextMenuState(null);
        return;
      }

      if (!hasContextMenuActions()) {
        setContextMenuState(null);
        return;
      }

      const offsetX = clientX - rect.left;
      const offsetY = clientY - rect.top;

      setContextMenuState({
        canvasX,
        canvasY,
        offsetX,
        offsetY,
      });
    },
    [canvasRef, hasContextMenuActions, isPaused, setContextMenuState],
  );

  const handleContextMenuEvent = useCallback(
    (event: React.MouseEvent<HTMLCanvasElement>) => {
      event.preventDefault();
      openContextMenuAtPoint(event.clientX, event.clientY);
    },
    [openContextMenuAtPoint],
  );

  return {
    handleContextMenuEvent,
    openContextMenuAtPoint,
  };
}
