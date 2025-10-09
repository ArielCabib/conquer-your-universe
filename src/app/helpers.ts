import {
  MOVE_DISTANCE_MAX,
  MOVE_DISTANCE_MIN,
  PLANET_CENTER_X,
  PLANET_CENTER_Y,
  PLANET_RADIUS,
  SETTLER_RADIUS,
} from "../constants";
import { CropState, FarmState, GameState } from "../types";
import { easeOutQuad } from "../utils/easing";

export { easeOutQuad };

export function currentTimeMs(): number {
  return typeof performance !== "undefined" ? performance.now() : Date.now();
}

export function pointWithinPlanet(x: number, y: number): boolean {
  const dx = x - PLANET_CENTER_X;
  const dy = y - PLANET_CENTER_Y;
  return Math.sqrt(dx * dx + dy * dy) <= PLANET_RADIUS - SETTLER_RADIUS;
}

export function randomRange(min: number, max: number): number {
  const normalized = Math.random();
  return min + normalized * (max - min);
}

function randomAngle(): number {
  return randomRange(0, Math.PI * 2);
}

export function randomTargetNear(x: number, y: number): { x: number; y: number } {
  const ATTEMPTS = 8;
  for (let attempt = 0; attempt < ATTEMPTS; attempt += 1) {
    const angle = randomAngle();
    const distance = randomRange(MOVE_DISTANCE_MIN, MOVE_DISTANCE_MAX);
    const candidateX = x + distance * Math.cos(angle);
    const candidateY = y + distance * Math.sin(angle);
    if (pointWithinPlanet(candidateX, candidateY)) {
      return { x: candidateX, y: candidateY };
    }
  }

  return { x: PLANET_CENTER_X, y: PLANET_CENTER_Y };
}

export function ensureSettlerLifespans(state: GameState): void {
  const minLifespan = state.settlerMinLifespanMs;
  const maxLifespan = state.settlerMaxLifespanMs;

  state.settlers.forEach((settler) => {
    if (settler.lifespanMs <= 0) {
      settler.lifespanMs = randomRange(minLifespan, maxLifespan);
    }
  });
}

export function ensureHouseRegistry(state: GameState): void {
  const highestId = state.houses.reduce<number | undefined>((maxId, house) => {
    if (maxId === undefined || house.id > maxId) {
      return house.id;
    }
    return maxId;
  }, undefined);

  if (typeof highestId === "number") {
    const nextId = highestId + 1;
    if (state.nextHouseId <= highestId) {
      state.nextHouseId = nextId;
    }
  }
}

export function ensureFarmRegistry(state: GameState): void {
  const highestId = state.farms.reduce<number | undefined>((maxId, farm) => {
    if (maxId === undefined || farm.id > maxId) {
      return farm.id;
    }
    return maxId;
  }, undefined);

  if (typeof highestId === "number") {
    const nextId = highestId + 1;
    if (state.nextFarmId <= highestId) {
      state.nextFarmId = nextId;
    }
  }

  state.farms.forEach((farm) => {
    if (typeof farm.lastProducedMs !== "number" || !Number.isFinite(farm.lastProducedMs)) {
      farm.lastProducedMs = farm.builtMs;
    }
  });
}

export function ensureCropRegistry(state: GameState): void {
  const highestId = state.crops.reduce<number | undefined>((maxId, crop) => {
    if (maxId === undefined || crop.id > maxId) {
      return crop.id;
    }
    return maxId;
  }, undefined);

  if (typeof highestId === "number") {
    const nextId = highestId + 1;
    if (state.nextCropId <= highestId) {
      state.nextCropId = nextId;
    }
  }

  state.crops.forEach((crop) => {
    if (typeof crop.createdMs !== "number" || !Number.isFinite(crop.createdMs)) {
      crop.createdMs = 0;
    }
  });
}

const CROP_COLLISION_DISTANCE = 14;
const FARM_CROP_DISTANCE_MIN = 14;
const FARM_CROP_DISTANCE_MAX = 52;
const FARM_CROP_VERTICAL_SQUASH = 0.6;

export function randomCropPositionNearFarm(
  farm: FarmState,
  existingCrops: CropState[],
): { x: number; y: number } {
  const ATTEMPTS = 12;

  for (let attempt = 0; attempt < ATTEMPTS; attempt += 1) {
    const angle = randomAngle();
    const distance = randomRange(FARM_CROP_DISTANCE_MIN, FARM_CROP_DISTANCE_MAX);
    const x = farm.x + Math.cos(angle) * distance;
    const y = farm.y + Math.sin(angle) * distance * FARM_CROP_VERTICAL_SQUASH;

    if (!pointWithinPlanet(x, y)) {
      continue;
    }

    const collides = existingCrops.some((crop) => {
      const dx = crop.x - x;
      const dy = crop.y - y;
      return Math.hypot(dx, dy) < CROP_COLLISION_DISTANCE;
    });

    if (!collides) {
      return { x, y };
    }
  }

  return {
    x: farm.x + randomRange(-FARM_CROP_DISTANCE_MIN, FARM_CROP_DISTANCE_MIN),
    y: farm.y + randomRange(-FARM_CROP_DISTANCE_MIN * FARM_CROP_VERTICAL_SQUASH, FARM_CROP_DISTANCE_MIN * FARM_CROP_VERTICAL_SQUASH),
  };
}

export function randomTargetForSettler(x: number, y: number): { x: number; y: number } {
  return randomTargetNear(x, y);
}
