import { MutableRefObject, useEffect } from "react";
import { STORAGE_KEY } from "../../constants";
import { serializeGameState } from "../../persistence";
import { GameState } from "../../types";

export function usePeriodicSave(gameStateRef: MutableRefObject<GameState>) {
  useEffect(() => {
    if (typeof window === "undefined" || typeof localStorage === "undefined") {
      return;
    }

    const interval = window.setInterval(() => {
      void (async () => {
        try {
          const serialized = await serializeGameState(gameStateRef.current);
          localStorage.setItem(STORAGE_KEY, serialized);
        } catch (error) {
          console.warn("Failed to persist game state", error);
        }
      })();
    }, 1_000);

    return () => {
      window.clearInterval(interval);
    };
  }, [gameStateRef]);
}
