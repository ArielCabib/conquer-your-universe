import { GameState } from "../../types";

const HABITATION_EFFICIENCY_INTERVAL_MS = 2_000;

export function applyResearchNodeEffect(state: GameState, nodeId: string): void {
  switch (nodeId) {
    case "habitation-efficiency": {
      if (state.houseSpawnIntervalMs > HABITATION_EFFICIENCY_INTERVAL_MS) {
        state.houseSpawnIntervalMs = HABITATION_EFFICIENCY_INTERVAL_MS;
      }
      break;
    }
    default: {
      break;
    }
  }
}

export function applyCompletedResearchNodeEffects(state: GameState): void {
  for (const nodeId of state.completedResearchNodeIds) {
    applyResearchNodeEffect(state, nodeId);
  }
}
