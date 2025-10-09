import {
  BIRTH_ANIMATION_MS,
  FADING_DURATION_MS,
  MOVE_INTERVAL_MS,
  ORBIT_02,
  ORBIT_04,
  SETTLER_RADIUS,
} from "../../../constants";
import {
  GameState,
  SettlerState,
  createSettlerState,
  settlerPositionAt,
} from "../../../types";
import { easeOutQuad, randomRange, randomTargetForSettler } from "../../helpers";
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
  renderHouses: (ctx: CanvasRenderingContext2D, gameState: GameState, timestamp: number) => void,
) {
  let aliveTotal = 0;
  const survivors: SettlerState[] = [];

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

  for (const house of state.houses) {
    if (capacityLimit !== null && aliveTotal >= capacityLimit) {
      break;
    }

    if (now - house.lastSpawnMs >= 5_000) {
      const id = nextSettlerId;
      nextSettlerId += 1;
      const lifespan = randomRange(minLifespan, maxLifespan);
      newSettlers.push(createSettlerState(id, house.x, house.y, now, lifespan));
      house.lastSpawnMs = now;
      aliveTotal += 1;
    }
  }

  if (newSettlers.length > 0) {
    state.settlers = state.settlers.concat(newSettlers);
  }

  state.nextSettlerId = nextSettlerId;

  renderHouses(context, state, now);
  setAliveCount(aliveTotal);
}
