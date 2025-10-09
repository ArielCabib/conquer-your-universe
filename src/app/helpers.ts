import {
  MOVE_DISTANCE_MAX,
  MOVE_DISTANCE_MIN,
  PLANET_CENTER_X,
  PLANET_CENTER_Y,
  PLANET_RADIUS,
  SETTLER_RADIUS,
  ORBIT_01,
  ORBIT_02,
  ORBIT_03,
  ORBIT_04,
  ORBIT_05,
} from "../constants";
import { FarmState, GameState, HouseState, SettlerState, houseSpawnHighlight } from "../types";
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
}

export function drawHouse(
  context: CanvasRenderingContext2D,
  house: HouseState,
  nowMs: number,
): void {
  const highlightFactor = houseSpawnHighlight(nowMs, house);

  if (highlightFactor > 0) {
    const haloRadius = 36 + 18 * highlightFactor;
    context.globalAlpha = 0.45 * highlightFactor;
    context.fillStyle = ORBIT_04;
    context.beginPath();
    context.arc(house.x, house.y + 4, haloRadius, 0, Math.PI * 2);
    context.fill();
    context.globalAlpha = 1;
  }

  const baseWidth = 28;
  const baseHeight = 18;
  const roofHeight = 14;

  const baseX = house.x - baseWidth / 2;
  const baseY = house.y - baseHeight / 2;

  context.save();
  context.fillStyle = ORBIT_01;
  context.fillRect(baseX, baseY, baseWidth, baseHeight);

  if (highlightFactor > 0) {
    context.globalAlpha = 0.35 * highlightFactor + 0.2;
    context.fillStyle = ORBIT_05;
    context.fillRect(baseX, baseY, baseWidth, baseHeight);
    context.globalAlpha = 1;
  }

  context.fillStyle = ORBIT_02;
  context.beginPath();
  context.moveTo(baseX - 2, baseY);
  context.lineTo(house.x, baseY - roofHeight);
  context.lineTo(baseX + baseWidth + 2, baseY);
  context.closePath();
  context.fill();

  if (highlightFactor > 0) {
    context.globalAlpha = 0.45 * highlightFactor + 0.15;
    context.beginPath();
    context.moveTo(baseX - 2, baseY);
    context.lineTo(house.x, baseY - roofHeight - 4 * highlightFactor);
    context.lineTo(baseX + baseWidth + 2, baseY);
    context.closePath();
    context.fillStyle = ORBIT_03;
    context.fill();
    context.globalAlpha = 1;
  }

  context.fillStyle = ORBIT_05;
  const windowSize = baseWidth * 0.22;
  const windowY = baseY + baseHeight * 0.28;
  context.fillRect(baseX + baseWidth * 0.16, windowY, windowSize, windowSize);
  context.fillRect(
    baseX + baseWidth - windowSize - baseWidth * 0.16,
    windowY,
    windowSize,
    windowSize,
  );

  context.fillStyle = ORBIT_04;
  const doorWidth = baseWidth * 0.28;
  const doorHeight = baseHeight * 0.62;
  context.fillRect(house.x - doorWidth / 2, baseY + baseHeight - doorHeight, doorWidth, doorHeight);

  context.restore();
}

export function drawFarm(
  context: CanvasRenderingContext2D,
  farm: FarmState,
  nowMs: number,
): void {
  const elapsed = Math.max(0, nowMs - farm.builtMs);
  const highlight = Math.max(0, 1 - Math.min(1, elapsed / 1_200));
  const scale = 0.75;
  const baseOffsetY = 6 * scale;
  const highlightRadiusX = 44 * scale;
  const highlightRadiusY = 24 * scale;
  const highlightExpandX = 12 * scale;
  const highlightExpandY = 6 * scale;
  const soilRadiusX = 38 * scale;
  const soilRadiusY = 18 * scale;
  const rowTopY = -6 * scale;
  const rowMidY = 0;
  const rowBottomY = 6 * scale;
  const rowOuterX = 28 * scale;
  const rowMidX = 22 * scale;
  const rowInnerX = 18 * scale;

  context.save();
  context.translate(farm.x, farm.y);

  if (highlight > 0) {
    context.globalAlpha = 0.2 + 0.35 * highlight;
    context.fillStyle = ORBIT_04;
    context.beginPath();
    context.ellipse(
      0,
      baseOffsetY,
      highlightRadiusX + highlightExpandX * highlight,
      highlightRadiusY + highlightExpandY * highlight,
      0,
      0,
      Math.PI * 2,
    );
    context.fill();
    context.globalAlpha = 1;
  }

  context.fillStyle = ORBIT_02;
  context.beginPath();
  context.ellipse(0, baseOffsetY, highlightRadiusX, highlightRadiusY, 0, 0, Math.PI * 2);
  context.fill();

  context.fillStyle = ORBIT_01;
  context.beginPath();
  context.ellipse(0, 0, soilRadiusX, soilRadiusY, 0, 0, Math.PI * 2);
  context.fill();

  context.strokeStyle = ORBIT_05;
  context.lineWidth = 2 * scale;
  context.beginPath();
  context.moveTo(-rowOuterX, rowMidY);
  context.lineTo(rowOuterX, rowMidY);
  context.moveTo(-rowMidX, rowTopY);
  context.lineTo(rowMidX, rowTopY);
  context.moveTo(-rowInnerX, rowBottomY);
  context.lineTo(rowInnerX, rowBottomY);
  context.stroke();

  context.restore();
}

export function randomTargetForSettler(x: number, y: number): { x: number; y: number } {
  return randomTargetNear(x, y);
}
