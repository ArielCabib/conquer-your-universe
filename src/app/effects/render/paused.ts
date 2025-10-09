import {
  BIRTH_ANIMATION_MS,
  FADING_DURATION_MS,
  ORBIT_02,
  ORBIT_04,
  SETTLER_RADIUS,
} from "../../../constants";
import { GameState, settlerPositionAt } from "../../../types";
import { easeOutQuad } from "../../helpers";
import { Dispatch, SetStateAction } from "react";

export function renderPausedState(
  context: CanvasRenderingContext2D,
  state: GameState,
  now: number,
  setAliveCount: Dispatch<SetStateAction<number>>,
  renderStructures: (ctx: CanvasRenderingContext2D, gameState: GameState, timestamp: number) => void,
) {
  let aliveTotal = 0;

  for (const settler of state.settlers) {
    const { x, y } = settlerPositionAt(settler, now);

    if (settler.phase.kind === "Alive") {
      const ageMs = now - settler.birthMs;
      let displayRadius = SETTLER_RADIUS;
      let baseAlpha = 0.92;

      if (ageMs < BIRTH_ANIMATION_MS) {
        const progress = easeOutQuad(Math.max(0, Math.min(1, ageMs / BIRTH_ANIMATION_MS)));
        displayRadius = Math.max(1.2, SETTLER_RADIUS * progress);
        const haloRadius = displayRadius * (1 + 0.6 * (1 - progress));

        context.globalAlpha = (1 - progress) * 0.45;
        context.fillStyle = ORBIT_02;
        context.beginPath();
        context.arc(x, y, haloRadius, 0, Math.PI * 2);
        context.fill();
        context.globalAlpha = 1;

        baseAlpha = 0.74 + 0.18 * progress;
      }

      context.globalAlpha = baseAlpha;
      context.fillStyle = ORBIT_04;
      context.beginPath();
      context.arc(x, y, displayRadius, 0, Math.PI * 2);
      context.fill();
      context.globalAlpha = 1;

      aliveTotal += 1;
    } else if (settler.phase.kind === "Fading") {
      const elapsed = now - settler.phase.startedMs;
      if (elapsed < FADING_DURATION_MS) {
        const progress = Math.max(0, Math.min(1, elapsed / FADING_DURATION_MS));
        const opacity = 1 - progress;
        const radius = SETTLER_RADIUS * (1 + 0.6 * progress);

        context.globalAlpha = opacity;
        context.fillStyle = ORBIT_02;
        context.beginPath();
        context.arc(x, y, radius, 0, Math.PI * 2);
        context.fill();
        context.globalAlpha = 1;
      }
    }
  }

  renderStructures(context, state, now);
  setAliveCount(aliveTotal);
}
