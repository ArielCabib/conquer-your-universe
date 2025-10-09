import { MutableRefObject, useEffect } from "react";
import { STORAGE_KEY } from "../../constants";
import { deserializeGameState, serializeGameState } from "../../persistence";
import { GameState } from "../../types";
import { ensureFarmRegistry, ensureHouseRegistry, ensureSettlerLifespans } from "../helpers";

export function useRestoreState(
  gameStateRef: MutableRefObject<GameState>,
  onRestore?: (state: GameState) => void,
) {
  useEffect(() => {
    if (typeof localStorage === "undefined") {
      return;
    }

    try {
      const stored = localStorage.getItem(STORAGE_KEY);
      if (stored) {
        const restored = deserializeGameState(stored);
        if (restored) {
          ensureSettlerLifespans(restored);
          ensureHouseRegistry(restored);
          ensureFarmRegistry(restored);
          gameStateRef.current = restored;
          onRestore?.(restored);
          return;
        }
      }

      localStorage.setItem(STORAGE_KEY, serializeGameState(gameStateRef.current));
    } catch (error) {
      console.warn("Failed to restore game state", error);
    }
  }, [gameStateRef, onRestore]);
}
