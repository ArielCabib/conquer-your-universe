import {
  BASE_SETTLER_MAX_LIFESPAN_MS,
  BASE_SETTLER_MIN_LIFESPAN_MS,
  BIRTH_ANIMATION_MS,
  GRAIN_PILE_CAPACITY,
  HOUSE_SPAWN_ANIMATION_MS,
  MOVE_INTERVAL_MS,
  PLANET_CENTER_X,
  PLANET_CENTER_Y,
  PLANET_RADIUS,
  SETTLER_RADIUS,
} from "./constants";
import { easeOutQuad } from "./utils/easing";

export type SettlerPhase =
  | { kind: "Alive" }
  | { kind: "Fading"; startedMs: number };

export interface SettlerState {
  id: number;
  anchorX: number;
  anchorY: number;
  targetX: number;
  targetY: number;
  moveStartMs: number;
  lastDirectionChangeMs: number;
  birthMs: number;
  phase: SettlerPhase;
  lifespanMs: number;
}

export interface HouseState {
  id: number;
  x: number;
  y: number;
  builtMs: number;
  lastSpawnMs: number;
}

export interface FarmState {
  id: number;
  x: number;
  y: number;
  builtMs: number;
  lastProducedMs: number;
}

export interface CropState {
  id: number;
  farmId: number;
  x: number;
  y: number;
  createdMs: number;
}

export interface GrainPileState {
  x: number;
  y: number;
  grains: number;
  createdMs: number;
}

export interface CropProjectileState {
  id: number;
  startX: number;
  startY: number;
  endX: number;
  endY: number;
  launchedMs: number;
  durationMs: number;
}

export interface GrainProjectileState {
  id: number;
  startX: number;
  startY: number;
  endX: number;
  endY: number;
  launchedMs: number;
  durationMs: number;
}

export interface HarvesterState {
  x: number;
  y: number;
  builtMs: number;
  lastHarvestMs: number;
  spinLevel: number;
  rotationAngle: number;
  lastSpinUpdateMs: number;
}

export interface MarketState {
  x: number;
  y: number;
  builtMs: number;
  lastSaleMs: number;
}

export interface CoinProjectileState {
  id: number;
  x: number;
  startY: number;
  endY: number;
  launchedMs: number;
  durationMs: number;
}

export interface GameState {
  planetName: string;
  settlers: SettlerState[];
  nextSettlerId: number;
  settlerMinLifespanMs: number;
  settlerMaxLifespanMs: number;
  houses: HouseState[];
  nextHouseId: number;
  farms: FarmState[];
  nextFarmId: number;
  crops: CropState[];
  nextCropId: number;
  harvester: HarvesterState | null;
  grainPile: GrainPileState | null;
  cropProjectiles: CropProjectileState[];
  grainProjectiles: GrainProjectileState[];
  marketGrainProjectiles: GrainProjectileState[];
  coinProjectiles: CoinProjectileState[];
  nextCropProjectileId: number;
  nextGrainProjectileId: number;
  nextCoinProjectileId: number;
  market: MarketState | null;
  grainPileCapacity: number;
  coins: number;
  settlersBaseCapacity: number;
  housesBaseCapacity: number;
  farmsBaseCapacity: number;
  settlersPerHouse: number;
  farmLifespanBonusPerFarmMs: number;
  farmCropCapacity: number;
  farmCropSpawnIntervalMs: number;
  houseSpawnIntervalMs: number;
  houseSpawnAmount: number;
}

export function createInitialGameState(): GameState {
  return {
    planetName: "Your Planet",
    settlers: [],
    nextSettlerId: 0,
    settlerMinLifespanMs: BASE_SETTLER_MIN_LIFESPAN_MS,
    settlerMaxLifespanMs: BASE_SETTLER_MAX_LIFESPAN_MS,
    houses: [],
    nextHouseId: 0,
    farms: [],
    nextFarmId: 0,
    crops: [],
    nextCropId: 0,
    harvester: null,
    grainPile: null,
    cropProjectiles: [],
    grainProjectiles: [],
    marketGrainProjectiles: [],
    coinProjectiles: [],
    nextCropProjectileId: 0,
    nextGrainProjectileId: 0,
    nextCoinProjectileId: 0,
    market: null,
    grainPileCapacity: GRAIN_PILE_CAPACITY,
    coins: 0,
    settlersBaseCapacity: 10,
    housesBaseCapacity: 5,
    farmsBaseCapacity: 5,
    settlersPerHouse: 10,
    farmLifespanBonusPerFarmMs: 1_000,
    farmCropCapacity: 5,
    farmCropSpawnIntervalMs: 4_500,
    houseSpawnIntervalMs: 5_000,
    houseSpawnAmount: 1,
  };
}

export function createSettlerState(
  id: number,
  x: number,
  y: number,
  nowMs: number,
  lifespanMs: number,
): SettlerState {
  return {
    id,
    anchorX: x,
    anchorY: y,
    targetX: x,
    targetY: y,
    moveStartMs: nowMs,
    lastDirectionChangeMs: nowMs - MOVE_INTERVAL_MS,
    birthMs: nowMs,
    phase: { kind: "Alive" },
    lifespanMs,
  };
}

const NEGATIVE_INFINITY = Number.NEGATIVE_INFINITY;

export function createHouseState(
  id: number,
  x: number,
  y: number,
  builtMs: number,
): HouseState {
  return {
    id,
    x,
    y,
    builtMs,
    lastSpawnMs: NEGATIVE_INFINITY,
  };
}

export function createFarmState(
  id: number,
  x: number,
  y: number,
  builtMs: number,
  lastProducedMs: number = builtMs,
): FarmState {
  return {
    id,
    x,
    y,
    builtMs,
    lastProducedMs,
  };
}

export function createCropState(
  id: number,
  farmId: number,
  x: number,
  y: number,
  createdMs: number,
): CropState {
  return {
    id,
    farmId,
    x,
    y,
    createdMs,
  };
}

export function createHarvesterState(x: number, y: number, builtMs: number): HarvesterState {
  return {
    x,
    y,
    builtMs,
    lastHarvestMs: builtMs,
    spinLevel: 0,
    rotationAngle: 0,
    lastSpinUpdateMs: builtMs,
  };
}

export function createCropProjectileState(
  id: number,
  startX: number,
  startY: number,
  endX: number,
  endY: number,
  launchedMs: number,
  durationMs: number,
): CropProjectileState {
  return {
    id,
    startX,
    startY,
    endX,
    endY,
    launchedMs,
    durationMs,
  };
}

export function createGrainPileState(
  x: number,
  y: number,
  createdMs: number,
  grains: number = 0,
): GrainPileState {
  return {
    x,
    y,
    createdMs,
    grains,
  };
}

export function createGrainProjectileState(
  id: number,
  startX: number,
  startY: number,
  endX: number,
  endY: number,
  launchedMs: number,
  durationMs: number,
): GrainProjectileState {
  return {
    id,
    startX,
    startY,
    endX,
    endY,
    launchedMs,
    durationMs,
  };
}

export function createMarketState(x: number, y: number, builtMs: number): MarketState {
  return {
    x,
    y,
    builtMs,
    lastSaleMs: builtMs,
  };
}

export function createCoinProjectileState(
  id: number,
  x: number,
  startY: number,
  endY: number,
  launchedMs: number,
  durationMs: number,
): CoinProjectileState {
  return {
    id,
    x,
    startY,
    endY,
    launchedMs,
    durationMs,
  };
}

export function settlerPositionAt(
  settler: SettlerState,
  nowMs: number,
): { x: number; y: number } {
  const elapsed = Math.max(0, nowMs - settler.moveStartMs);
  const progress = Math.max(0, Math.min(1, elapsed / MOVE_INTERVAL_MS));
  const eased = easeOutQuad(progress);
  const x = settler.anchorX + (settler.targetX - settler.anchorX) * eased;
  const y = settler.anchorY + (settler.targetY - settler.anchorY) * eased;
  return { x, y };
}

export function isSettlerWithinPlanet(x: number, y: number): boolean {
  const dx = x - PLANET_CENTER_X;
  const dy = y - PLANET_CENTER_Y;
  return Math.sqrt(dx * dx + dy * dy) <= PLANET_RADIUS - SETTLER_RADIUS;
}

export function birthAnimationProgress(now: number, settler: SettlerState): number {
  const age = now - settler.birthMs;
  if (age <= 0) {
    return 0;
  }
  if (age >= BIRTH_ANIMATION_MS) {
    return 1;
  }
  return Math.max(0, Math.min(1, age / BIRTH_ANIMATION_MS));
}

export function houseSpawnHighlight(now: number, house: HouseState): number {
  const elapsed = Math.max(0, now - house.lastSpawnMs);
  if (elapsed >= HOUSE_SPAWN_ANIMATION_MS) {
    return 0;
  }
  return 1 - easeOutQuad(Math.max(0, Math.min(1, elapsed / HOUSE_SPAWN_ANIMATION_MS)));
}
