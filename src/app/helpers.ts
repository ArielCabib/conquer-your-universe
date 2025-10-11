import {
  GRAIN_PILE_CAPACITY,
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

export function randomPointOnPlanet(): { x: number; y: number } {
  const radius = randomRange(SETTLER_RADIUS * 2, PLANET_RADIUS - SETTLER_RADIUS * 2);
  const angle = randomAngle();
  const x = PLANET_CENTER_X + Math.cos(angle) * radius;
  const y = PLANET_CENTER_Y + Math.sin(angle) * radius;
  return { x, y };
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

export function ensureHarvesterResources(state: GameState): void {
  if (state.harvester) {
    if (
      typeof state.harvester.lastHarvestMs !== "number" ||
      !Number.isFinite(state.harvester.lastHarvestMs)
    ) {
      state.harvester.lastHarvestMs = state.harvester.builtMs;
    }

    if (
      typeof state.harvester.spinLevel !== "number" ||
      !Number.isFinite(state.harvester.spinLevel) ||
      state.harvester.spinLevel < 0
    ) {
      state.harvester.spinLevel = 0;
    }

    if (
      typeof state.harvester.rotationAngle !== "number" ||
      !Number.isFinite(state.harvester.rotationAngle)
    ) {
      state.harvester.rotationAngle = 0;
    }

    if (
      typeof state.harvester.lastSpinUpdateMs !== "number" ||
      !Number.isFinite(state.harvester.lastSpinUpdateMs)
    ) {
      state.harvester.lastSpinUpdateMs = state.harvester.builtMs;
    }
  }

  if (!Array.isArray(state.grainProjectiles)) {
    state.grainProjectiles = [];
  }

  if (!Array.isArray(state.cropProjectiles)) {
    state.cropProjectiles = [];
  }

  if (
    typeof state.grainPileCapacity !== "number" ||
    !Number.isFinite(state.grainPileCapacity) ||
    state.grainPileCapacity < 0
  ) {
    state.grainPileCapacity = Math.max(0, GRAIN_PILE_CAPACITY);
  }

  if (
    typeof state.nextGrainProjectileId !== "number" ||
    !Number.isFinite(state.nextGrainProjectileId) ||
    state.nextGrainProjectileId < 0
  ) {
    state.nextGrainProjectileId = 0;
  }

  if (
    typeof state.nextCropProjectileId !== "number" ||
    !Number.isFinite(state.nextCropProjectileId) ||
    state.nextCropProjectileId < 0
  ) {
    state.nextCropProjectileId = 0;
  }

  if (state.grainPile) {
    if (typeof state.grainPile.grains !== "number" || !Number.isFinite(state.grainPile.grains)) {
      state.grainPile.grains = 0;
    }

    if (state.grainPile.grains < 0) {
      state.grainPile.grains = 0;
    }

    const capacity = state.grainPileCapacity;

    if (state.grainPile.grains > capacity) {
      state.grainPile.grains = capacity;
    }
  }
}

const CROP_COLLISION_DISTANCE = 22;
const FARM_CROP_DISTANCE_MIN = 28;
const FARM_CROP_DISTANCE_MAX = 78;
const FARM_CROP_VERTICAL_SQUASH = 0.7;

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
