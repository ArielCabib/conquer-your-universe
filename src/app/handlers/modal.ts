import { Dispatch, MutableRefObject, SetStateAction, useCallback } from "react";
import { STORAGE_KEY } from "../../constants";
import { GameState, createInitialGameState } from "../../types";
import { serializeGameState } from "../../persistence";

export function useModalOpenHandler(setIsModalOpen: Dispatch<SetStateAction<boolean>>) {
  return useCallback(() => {
    setIsModalOpen(true);
  }, [setIsModalOpen]);
}

export function useModalCloseHandler(setIsModalOpen: Dispatch<SetStateAction<boolean>>) {
  return useCallback(() => {
    setIsModalOpen(false);
  }, [setIsModalOpen]);
}

interface RestartOptions {
  gameStateRef: MutableRefObject<GameState>;
  setAliveCount: Dispatch<SetStateAction<number>>;
  setIsModalOpen: Dispatch<SetStateAction<boolean>>;
  setIsPaused: Dispatch<SetStateAction<boolean>>;
  pauseTimeRef: MutableRefObject<number | null>;
  setPlanetName: Dispatch<SetStateAction<string>>;
}

export function useRestartGameHandler({
  gameStateRef,
  setAliveCount,
  setIsModalOpen,
  setIsPaused,
  pauseTimeRef,
  setPlanetName,
}: RestartOptions) {
  return useCallback(() => {
    gameStateRef.current = createInitialGameState();

    try {
      if (typeof localStorage !== "undefined") {
        localStorage.setItem(STORAGE_KEY, serializeGameState(gameStateRef.current));
      }
    } catch (error) {
      console.warn("Failed to persist game state after restart", error);
    }

    setAliveCount(0);
    setPlanetName(gameStateRef.current.planetName);
    pauseTimeRef.current = null;
    setIsPaused(false);
    setIsModalOpen(false);
  }, [gameStateRef, pauseTimeRef, setAliveCount, setIsModalOpen, setIsPaused, setPlanetName]);
}
