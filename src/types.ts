import {
  BIRTH_ANIMATION_MS,
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

export interface GameState {
  settlers: SettlerState[];
  nextSettlerId: number;
  settlerMinLifespanMs: number;
  settlerMaxLifespanMs: number;
  houses: HouseState[];
  nextHouseId: number;
  settlersBaseCapacity: number;
  housesBaseCapacity: number;
  settlersPerHouse: number;
}

export function createInitialGameState(): GameState {
  return {
    settlers: [],
    nextSettlerId: 0,
    settlerMinLifespanMs: 5_000,
    settlerMaxLifespanMs: 10_000,
    houses: [],
    nextHouseId: 0,
    settlersBaseCapacity: 10,
    housesBaseCapacity: 5,
    settlersPerHouse: 10,
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
