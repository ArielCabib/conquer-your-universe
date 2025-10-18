import type { Dispatch, SetStateAction } from "react";

import type { GameState } from "../../types";
import type { SimulationSnapshot } from "../types";

function snapshotsEqual(a: SimulationSnapshot, b: SimulationSnapshot): boolean {
  return (
    a.cropCount === b.cropCount &&
    a.grainCount === b.grainCount &&
    a.grainsInFlight === b.grainsInFlight &&
    a.hasGrainPile === b.hasGrainPile &&
    a.hasHarvester === b.hasHarvester &&
    a.hasResearcher === b.hasResearcher &&
    a.hasMarket === b.hasMarket &&
    a.coinCount === b.coinCount
  );
}

export function computeSimulationSnapshot(state: GameState): SimulationSnapshot {
  return {
    cropCount: state.crops.length,
    grainCount: state.grainPile?.grains ?? 0,
    grainsInFlight:
      state.grainProjectiles.length +
      state.cropProjectiles.length +
      state.marketGrainProjectiles.length,
    hasGrainPile: Boolean(state.grainPile),
    hasHarvester: Boolean(state.harvester),
    hasResearcher: Boolean(state.researcher),
    hasMarket: Boolean(state.market),
    coinCount: state.coins ?? 0,
  };
}

export function updateSimulationSnapshot(
  state: GameState,
  setSimulationSnapshot: Dispatch<SetStateAction<SimulationSnapshot>>,
): void {
  const snapshot = computeSimulationSnapshot(state);

  setSimulationSnapshot((current) => {
    if (snapshotsEqual(current, snapshot)) {
      return current;
    }

    return snapshot;
  });
}
