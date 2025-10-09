import { Dispatch, MutableRefObject, SetStateAction, useCallback } from "react";
import { STORAGE_KEY } from "../../constants";
import { GameState, createFarmState, createHouseState } from "../../types";
import { ContextMenuState } from "../types";
import { pointWithinPlanet, currentTimeMs } from "../helpers";
import { serializeGameState } from "../../persistence";

type SetContextMenuState = Dispatch<SetStateAction<ContextMenuState | null>>;

interface BuildHouseOptions {
  gameStateRef: MutableRefObject<GameState>;
  aliveCount: number;
  contextMenuState: ContextMenuState | null;
  setContextMenuState: SetContextMenuState;
}

export function useBuildHouseMenuHandler({
  gameStateRef,
  aliveCount,
  contextMenuState,
  setContextMenuState,
}: BuildHouseOptions) {
  return useCallback(
    () => {
      const menuState = contextMenuState;
      if (!menuState) {
        return;
      }

      if (aliveCount < 1) {
        setContextMenuState(null);
        return;
      }

      if (!pointWithinPlanet(menuState.canvasX, menuState.canvasY)) {
        setContextMenuState(null);
        return;
      }

      const state = gameStateRef.current;
      const houseLimit = state.housesBaseCapacity;
      if (houseLimit > 0 && state.houses.length >= houseLimit) {
        setContextMenuState(null);
        return;
      }

      const houseId = state.nextHouseId;
      state.nextHouseId = houseId + 1;
      const builtAt = currentTimeMs();
      state.houses.push(createHouseState(houseId, menuState.canvasX, menuState.canvasY, builtAt));

      try {
        if (typeof localStorage !== "undefined") {
          localStorage.setItem(STORAGE_KEY, serializeGameState(state));
        }
      } catch (error) {
        console.warn("Failed to persist game state after building house", error);
      }

      setContextMenuState(null);
    },
    [aliveCount, contextMenuState, gameStateRef, setContextMenuState],
  );
}

interface BuildFarmOptions {
  gameStateRef: MutableRefObject<GameState>;
  aliveCount: number;
  contextMenuState: ContextMenuState | null;
  setContextMenuState: SetContextMenuState;
}

export function useBuildFarmMenuHandler({
  gameStateRef,
  aliveCount,
  contextMenuState,
  setContextMenuState,
}: BuildFarmOptions) {
  return useCallback(
    () => {
      const menuState = contextMenuState;
      if (!menuState) {
        return;
      }

      if (aliveCount < 10) {
        setContextMenuState(null);
        return;
      }

      if (!pointWithinPlanet(menuState.canvasX, menuState.canvasY)) {
        setContextMenuState(null);
        return;
      }

      const state = gameStateRef.current;
      const farmLimit = state.farmsBaseCapacity;
      if (farmLimit > 0 && state.farms.length >= farmLimit) {
        setContextMenuState(null);
        return;
      }
      const farmId = state.nextFarmId;
      state.nextFarmId = farmId + 1;
      const builtAt = currentTimeMs();
      const lastProducedMs = builtAt - state.farmCropSpawnIntervalMs;
      state.farms.push(
        createFarmState(farmId, menuState.canvasX, menuState.canvasY, builtAt, lastProducedMs),
      );

      const farmBonus = state.farmLifespanBonusPerFarmMs;
      state.settlerMinLifespanMs += farmBonus;
      state.settlerMaxLifespanMs += farmBonus;
      state.settlers.forEach((settler) => {
        if (settler.phase.kind === "Alive") {
          settler.lifespanMs += farmBonus;
        }
      });

      try {
        if (typeof localStorage !== "undefined") {
          localStorage.setItem(STORAGE_KEY, serializeGameState(state));
        }
      } catch (error) {
        console.warn("Failed to persist game state after building farm", error);
      }

      setContextMenuState(null);
    },
    [aliveCount, contextMenuState, gameStateRef, setContextMenuState],
  );
}
