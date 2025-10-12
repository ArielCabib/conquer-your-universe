export interface ContextMenuState {
  canvasX: number;
  canvasY: number;
  offsetX: number;
  offsetY: number;
}

export interface InfoEntry {
  id: string;
  title: string;
  description: string;
}

export interface SimulationSnapshot {
  cropCount: number;
  grainCount: number;
  grainsInFlight: number;
  hasGrainPile: boolean;
  hasHarvester: boolean;
  hasMarket: boolean;
  coinCount: number;
}
