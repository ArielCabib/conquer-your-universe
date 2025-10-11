import {
  BIRTH_ANIMATION_MS,
  CROP_FLIGHT_DURATION_MS,
  FADING_DURATION_MS,
  GRAIN_THROW_DURATION_MS,
  HARVESTER_PROCESS_INTERVAL_MS,
  MOVE_INTERVAL_MS,
  ORBIT_02,
  ORBIT_04,
  SETTLER_RADIUS,
} from "../../../constants";
import {
  CropProjectileState,
  CropState,
  GameState,
  GrainProjectileState,
  SettlerState,
  createCropProjectileState,
  createCropState,
  createGrainPileState,
  createGrainProjectileState,
  createSettlerState,
  settlerPositionAt,
} from "../../../types";
import {
  easeOutQuad,
  randomCropPositionNearFarm,
  randomPointOnPlanet,
  randomRange,
  randomTargetForSettler,
} from "../../helpers";
import { Dispatch, SetStateAction } from "react";

function drawBirthHalo(
  context: CanvasRenderingContext2D,
  x: number,
  y: number,
  progress: number,
  radius: number,
) {
  const haloRadius = radius * (1 + 0.6 * (1 - progress));
  context.globalAlpha = (1 - progress) * 0.45;
  context.fillStyle = ORBIT_02;
  context.beginPath();
  context.arc(x, y, haloRadius, 0, Math.PI * 2);
  context.fill();
  context.globalAlpha = 1;
}

function drawAliveSettler(
  context: CanvasRenderingContext2D,
  settler: SettlerState,
  x: number,
  y: number,
  now: number,
): number {
  const ageMs = now - settler.birthMs;
  let displayRadius = SETTLER_RADIUS;
  let baseAlpha = 0.92;

  if (ageMs < BIRTH_ANIMATION_MS) {
    const progress = easeOutQuad(Math.max(0, Math.min(1, ageMs / BIRTH_ANIMATION_MS)));
    displayRadius = Math.max(1.2, SETTLER_RADIUS * progress);
    drawBirthHalo(context, x, y, progress, displayRadius);
    baseAlpha = 0.74 + 0.18 * progress;
  }

  context.globalAlpha = baseAlpha;
  context.fillStyle = ORBIT_04;
  context.beginPath();
  context.arc(x, y, displayRadius, 0, Math.PI * 2);
  context.fill();
  context.globalAlpha = 1;

  return displayRadius;
}

function drawFadingSettler(
  context: CanvasRenderingContext2D,
  x: number,
  y: number,
  startedMs: number,
  now: number,
): boolean {
  const elapsed = now - startedMs;
  if (elapsed >= FADING_DURATION_MS) {
    return false;
  }

  const progress = Math.max(0, Math.min(1, elapsed / FADING_DURATION_MS));
  const opacity = 1 - progress;
  const radius = SETTLER_RADIUS * (1 + 0.6 * progress);

  context.globalAlpha = opacity;
  context.fillStyle = ORBIT_02;
  context.beginPath();
  context.arc(x, y, radius, 0, Math.PI * 2);
  context.fill();
  context.globalAlpha = 1;

  return true;
}

export function handleActiveState(
  context: CanvasRenderingContext2D,
  state: GameState,
  now: number,
  setAliveCount: Dispatch<SetStateAction<number>>,
  renderStructures: (ctx: CanvasRenderingContext2D, gameState: GameState, timestamp: number) => void,
) {
  let aliveTotal = 0;
  const survivors: SettlerState[] = [];
  const newCrops: CropState[] = [];

  for (const settler of state.settlers) {
    const { x, y } = settlerPositionAt(settler, now);

    if (
      settler.phase.kind === "Alive" &&
      now - settler.birthMs >= settler.lifespanMs &&
      settler.lifespanMs > 0
    ) {
      settler.anchorX = x;
      settler.anchorY = y;
      settler.targetX = x;
      settler.targetY = y;
      settler.moveStartMs = now;
      settler.phase = { kind: "Fading", startedMs: now };
    }

    if (settler.phase.kind === "Alive") {
      if (now - settler.lastDirectionChangeMs >= MOVE_INTERVAL_MS) {
        const target = randomTargetForSettler(x, y);
        settler.anchorX = x;
        settler.anchorY = y;
        settler.targetX = target.x;
        settler.targetY = target.y;
        settler.moveStartMs = now;
        settler.lastDirectionChangeMs = now;
      }

      drawAliveSettler(context, settler, x, y, now);
      aliveTotal += 1;
      survivors.push(settler);
    } else if (
      settler.phase.kind === "Fading" &&
      drawFadingSettler(context, x, y, settler.phase.startedMs, now)
    ) {
      survivors.push(settler);
    }
  }

  state.settlers = survivors;

  const baseCapacity = state.settlersBaseCapacity;
  const settlersPerHouse = state.settlersPerHouse;
  const housesLen = state.houses.length;
  const settlersCapacityLimit = baseCapacity + housesLen * settlersPerHouse;
  const capacityLimit = settlersCapacityLimit > 0 ? settlersCapacityLimit : null;
  const minLifespan = state.settlerMinLifespanMs;
  const maxLifespan = state.settlerMaxLifespanMs;
  const newSettlers: SettlerState[] = [];
  let nextSettlerId = state.nextSettlerId;
  const houseSpawnIntervalMs = state.houseSpawnIntervalMs;
  const houseSpawnAmount = state.houseSpawnAmount;
  let nextCropId = state.nextCropId;

  const cropsPerFarmLimit = state.farmCropCapacity <= 0 ? Number.POSITIVE_INFINITY : state.farmCropCapacity;
  const cropSpawnIntervalMs = state.farmCropSpawnIntervalMs;
  const cropsByFarm = new Map<number, CropState[]>();

  for (const crop of state.crops) {
    let list = cropsByFarm.get(crop.farmId);
    if (!list) {
      list = [];
      cropsByFarm.set(crop.farmId, list);
    }
    list.push(crop);
  }

  for (const farm of state.farms) {
    let farmCrops = cropsByFarm.get(farm.id);
    if (!farmCrops) {
      farmCrops = [];
      cropsByFarm.set(farm.id, farmCrops);
    }

    const capacityRemaining =
      cropsPerFarmLimit === Number.POSITIVE_INFINITY
        ? Number.POSITIVE_INFINITY
        : Math.max(0, cropsPerFarmLimit - farmCrops.length);

    if (capacityRemaining <= 0) {
      continue;
    }

    if (now - farm.lastProducedMs < cropSpawnIntervalMs) {
      continue;
    }

    const position = randomCropPositionNearFarm(farm, farmCrops);
    const crop = createCropState(nextCropId, farm.id, position.x, position.y, now);
    nextCropId += 1;
    newCrops.push(crop);
    farmCrops.push(crop);
    farm.lastProducedMs = now;
  }

  for (const house of state.houses) {
    if (capacityLimit !== null && aliveTotal >= capacityLimit) {
      break;
    }

    if (now - house.lastSpawnMs >= houseSpawnIntervalMs) {
      const capacityRemaining =
        capacityLimit !== null ? Math.max(0, capacityLimit - aliveTotal) : Number.POSITIVE_INFINITY;
      const spawnCount = Math.min(houseSpawnAmount, capacityRemaining);

      if (spawnCount > 0) {
        for (let index = 0; index < spawnCount; index += 1) {
          const id = nextSettlerId;
          nextSettlerId += 1;
          const lifespan = randomRange(minLifespan, maxLifespan);
          newSettlers.push(createSettlerState(id, house.x, house.y, now, lifespan));
          aliveTotal += 1;
        }
        house.lastSpawnMs = now;
      }
    }
  }

  if (newSettlers.length > 0) {
    state.settlers = state.settlers.concat(newSettlers);
  }

  state.nextSettlerId = nextSettlerId;
  if (newCrops.length > 0) {
    state.crops = state.crops.concat(newCrops);
  }
  state.nextCropId = nextCropId;

  const remainingProjectiles: GrainProjectileState[] = [];
  for (const projectile of state.grainProjectiles) {
    const elapsed = now - projectile.launchedMs;
    if (elapsed >= projectile.durationMs) {
      if (state.grainPile) {
        state.grainPile.grains = Math.min(
          state.grainPileCapacity,
          state.grainPile.grains + 1,
        );
      }
      continue;
    }
    remainingProjectiles.push(projectile);
  }
  state.grainProjectiles = remainingProjectiles;

  const harvester = state.harvester;
  if (harvester) {
    if (!Array.isArray(state.cropProjectiles)) {
      state.cropProjectiles = [];
    }

    if (!state.grainPile && (state.crops.length > 0 || state.cropProjectiles.length > 0)) {
      const position = randomPointOnPlanet();
      state.grainPile = createGrainPileState(position.x, position.y, now);
    }

    const pile = state.grainPile;
    const pendingDeliveries = state.grainProjectiles.length + state.cropProjectiles.length;
    const grainsInPile = pile?.grains ?? 0;
    const capacityRemaining = Math.max(0, state.grainPileCapacity - grainsInPile - pendingDeliveries);
    const lastHarvest = harvester.lastHarvestMs ?? harvester.builtMs;

    if (
      pile &&
      state.crops.length > 0 &&
      capacityRemaining > 0 &&
      now - lastHarvest >= HARVESTER_PROCESS_INTERVAL_MS
    ) {
      const cropIndex = Math.floor(Math.random() * state.crops.length);
      const [harvestedCrop] = state.crops.splice(cropIndex, 1);

      if (harvestedCrop) {
        const projectileId = state.nextCropProjectileId;
        state.nextCropProjectileId = projectileId + 1;

        const cropProjectile = createCropProjectileState(
          projectileId,
          harvestedCrop.x,
          harvestedCrop.y,
          harvester.x,
          harvester.y,
          now,
          CROP_FLIGHT_DURATION_MS,
        );

        state.cropProjectiles.push(cropProjectile);
        harvester.lastHarvestMs = now;
      }
    }

    const remainingCropProjectiles: CropProjectileState[] = [];
    if (pile) {
      for (const projectile of state.cropProjectiles) {
        const elapsed = now - projectile.launchedMs;
        if (elapsed >= projectile.durationMs) {
          const projectileId = state.nextGrainProjectileId;
          state.nextGrainProjectileId = projectileId + 1;

          const offsetX = randomRange(-6, 6);
          const offsetY = randomRange(-4, 4);
          const grainProjectile = createGrainProjectileState(
            projectileId,
            harvester.x,
            harvester.y,
            pile.x + offsetX,
            pile.y + offsetY,
            now,
            GRAIN_THROW_DURATION_MS,
          );

          state.grainProjectiles.push(grainProjectile);
        } else {
          remainingCropProjectiles.push(projectile);
        }
      }
    } else {
      remainingCropProjectiles.push(...state.cropProjectiles);
    }

    state.cropProjectiles = remainingCropProjectiles;

    const deltaMs = Math.max(0, now - harvester.lastSpinUpdateMs);
    harvester.lastSpinUpdateMs = now;

    const pendingAfterUpdate = state.cropProjectiles.length + state.grainProjectiles.length;
    const activeCapacity = state.grainPileCapacity - (state.grainPile?.grains ?? 0) - pendingAfterUpdate;
    const hasAvailableCapacity = activeCapacity > 0;
    const hasPendingWork = pendingAfterUpdate > 0;
    const hasIdleCrops = state.crops.length > 0;
    const targetSpinLevel = hasAvailableCapacity ? (hasIdleCrops || hasPendingWork ? 1 : 0) : hasPendingWork ? 1 : 0;

    const rampUp = deltaMs / 420;
    const rampDown = deltaMs / 320;
    if (targetSpinLevel > harvester.spinLevel) {
      harvester.spinLevel = Math.min(targetSpinLevel, harvester.spinLevel + rampUp);
    } else if (targetSpinLevel < harvester.spinLevel) {
      harvester.spinLevel = Math.max(targetSpinLevel, harvester.spinLevel - rampDown);
    }

    const rotationSpeed = (Math.PI / 2) / HARVESTER_PROCESS_INTERVAL_MS;
    harvester.rotationAngle += rotationSpeed * deltaMs * harvester.spinLevel;
    if (!Number.isFinite(harvester.rotationAngle)) {
      harvester.rotationAngle = 0;
    } else {
      const fullTurn = Math.PI * 2;
      harvester.rotationAngle %= fullTurn;
      if (harvester.rotationAngle < 0) {
        harvester.rotationAngle += fullTurn;
      }
    }
  }

  renderStructures(context, state, now);
  setAliveCount(aliveTotal);
}
