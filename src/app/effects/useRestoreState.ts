import { MutableRefObject, useEffect } from "react";
import { STORAGE_KEY } from "../../constants";
import { deserializeGameState, serializeGameState } from "../../persistence";
import { GameState } from "../../types";
import {
  ensureCropRegistry,
  ensureFarmRegistry,
  ensureHarvesterResources,
  ensureHouseRegistry,
  ensureSettlerLifespans,
} from "../helpers";

export function useRestoreState(
  gameStateRef: MutableRefObject<GameState>,
  onRestore?: (state: GameState) => void,
) {
  useEffect(() => {
    if (typeof localStorage === "undefined") {
      return;
    }

    let isCancelled = false;

    void (async () => {
      try {
        const stored = localStorage.getItem(STORAGE_KEY);
        if (stored) {
          const restored = await deserializeGameState(stored);
          if (restored) {
            ensureSettlerLifespans(restored);
            ensureHouseRegistry(restored);
            ensureFarmRegistry(restored);
            ensureCropRegistry(restored);
            ensureHarvesterResources(restored);
            if (isCancelled) {
              return;
            }
            gameStateRef.current = restored;
            onRestore?.(restored);
            return;
          }
        }

        const serialized = await serializeGameState(gameStateRef.current);
        if (!isCancelled) {
          localStorage.setItem(STORAGE_KEY, serialized);
        }
      } catch (error) {
        if (!isCancelled) {
          console.warn("Failed to restore game state", error);
        }
      }
    })();

    return () => {
      isCancelled = true;
    };
  }, [gameStateRef, onRestore]);
}
