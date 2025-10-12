import {
  ChangeEvent,
  Dispatch,
  MutableRefObject,
  RefObject,
  SetStateAction,
  useCallback,
} from "react";
import { STORAGE_KEY } from "../../constants";
import { serializeGameState, deserializeGameState } from "../../persistence";
import { GameState } from "../../types";
import {
  ensureCropRegistry,
  ensureFarmRegistry,
  ensureHouseRegistry,
  ensureSettlerLifespans,
} from "../helpers";
import type { SimulationSnapshot } from "../types";
import { computeSimulationSnapshot } from "../state/simulationSnapshot";

export function useOpenFileDialogHandler(fileInputRef: RefObject<HTMLInputElement>) {
  return useCallback(() => {
    const input = fileInputRef.current;
    if (input) {
      input.click();
    }
  }, [fileInputRef]);
}

export function useSaveGameHandler(gameStateRef: MutableRefObject<GameState>) {
  return useCallback(() => {
    const state = gameStateRef.current;
    try {
      const serialized = serializeGameState(state);
      const blob = new Blob([serialized], { type: "application/json" });
      const url = URL.createObjectURL(blob);

      const anchor = document.createElement("a");
      anchor.href = url;
      anchor.download = "conquer-your-universe-save.json";
      document.body?.appendChild(anchor);
      anchor.click();
      anchor.remove();
      URL.revokeObjectURL(url);
    } catch (error) {
      console.warn("Failed to create save file", error);
    }
  }, [gameStateRef]);
}

interface FileChangeOptions {
  gameStateRef: MutableRefObject<GameState>;
  setAliveCount: Dispatch<SetStateAction<number>>;
  setIsModalOpen: Dispatch<SetStateAction<boolean>>;
  setIsPaused: Dispatch<SetStateAction<boolean>>;
  pauseTimeRef: MutableRefObject<number | null>;
  setPlanetName: Dispatch<SetStateAction<string>>;
  setInfoEntryIds: Dispatch<SetStateAction<string[]>>;
  setSimulationSnapshot: Dispatch<SetStateAction<SimulationSnapshot>>;
}

export function useFileChangeHandler({
  gameStateRef,
  setAliveCount,
  setIsModalOpen,
  setIsPaused,
  pauseTimeRef,
  setPlanetName,
  setInfoEntryIds,
  setSimulationSnapshot,
}: FileChangeOptions) {
  return useCallback(
    (event: ChangeEvent<HTMLInputElement>) => {
      const input = event.target;
      if (!input) {
        return;
      }

      const file = input.files?.[0];
      if (!file) {
        input.value = "";
        return;
      }

      const reader = new FileReader();
      reader.onload = () => {
        const text = typeof reader.result === "string" ? reader.result : null;
        if (!text) {
          return;
        }

        const loadedState = deserializeGameState(text);
        if (!loadedState) {
          console.warn("Unable to parse game file");
          return;
        }

        ensureSettlerLifespans(loadedState);
        ensureHouseRegistry(loadedState);
        ensureFarmRegistry(loadedState);
        ensureCropRegistry(loadedState);

        gameStateRef.current = loadedState;
        setPlanetName(loadedState.planetName);
        setInfoEntryIds(() => {
          const ids = [...loadedState.infoEntryIds];
          gameStateRef.current.infoEntryIds = ids;
          return ids;
        });
        setSimulationSnapshot(computeSimulationSnapshot(loadedState));

        try {
          if (typeof localStorage !== "undefined") {
            localStorage.setItem(STORAGE_KEY, serializeGameState(loadedState));
          }
        } catch (error) {
          console.warn("Failed to persist loaded game state", error);
        }

        const aliveTotal = loadedState.settlers.filter((settler) => settler.phase.kind === "Alive").length;

        setAliveCount(aliveTotal);
        pauseTimeRef.current = null;
        setIsPaused(false);
        setIsModalOpen(false);
      };

      reader.onerror = () => {
        console.warn("An error occurred while reading the game file.");
      };

      try {
        reader.readAsText(file);
      } catch (error) {
        console.warn("Failed to initiate game file read", error);
      }

      input.value = "";
    },
    [
      gameStateRef,
      pauseTimeRef,
      setAliveCount,
      setIsModalOpen,
      setIsPaused,
      setPlanetName,
      setInfoEntryIds,
      setSimulationSnapshot,
    ],
  );
}
