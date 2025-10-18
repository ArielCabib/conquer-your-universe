import {
  HARVESTER_SPAWN_ANIMATION_MS,
  ORBIT_01,
  ORBIT_02,
  ORBIT_03,
  ORBIT_04,
  ORBIT_05,
} from "../../constants";
import { ResearcherState } from "../../types";

export function drawResearcher(
  context: CanvasRenderingContext2D,
  researcher: ResearcherState,
  nowMs: number,
): void {
  const spawnElapsed = Math.max(0, nowMs - researcher.builtMs);
  const spawnProgress = Math.max(
    0,
    Math.min(1, spawnElapsed / HARVESTER_SPAWN_ANIMATION_MS),
  );
  const pulseStrength = 1 - spawnProgress;

  context.save();
  context.translate(researcher.x, researcher.y);

  if (pulseStrength > 0) {
    const haloRadius = 34 + pulseStrength * 18;
    context.globalAlpha = 0.35 * pulseStrength;
    context.fillStyle = ORBIT_04;
    context.beginPath();
    context.arc(0, 6, haloRadius, 0, Math.PI * 2);
    context.fill();
    context.globalAlpha = 1;
  }

  // Base platform
  const baseRadius = 22;
  context.fillStyle = ORBIT_01;
  context.beginPath();
  context.ellipse(0, 10, baseRadius, baseRadius * 0.55, 0, 0, Math.PI * 2);
  context.fill();

  context.fillStyle = ORBIT_02;
  context.beginPath();
  context.ellipse(0, 6, baseRadius * 0.82, baseRadius * 0.38, 0, 0, Math.PI * 2);
  context.fill();

  // Central column
  context.fillStyle = ORBIT_05;
  context.fillRect(-5, -26, 10, 32);

  context.fillStyle = ORBIT_03;
  context.fillRect(-3, -26, 6, 30);

  // Observation dome
  const domeRadius = 14;
  context.fillStyle = ORBIT_04;
  context.beginPath();
  context.arc(0, -26, domeRadius, 0, Math.PI * 2);
  context.fill();

  context.fillStyle = ORBIT_02;
  context.beginPath();
  context.arc(0, -26, domeRadius * 0.6, 0, Math.PI * 2);
  context.fill();

  if (pulseStrength > 0) {
    context.globalAlpha = 0.4 * pulseStrength + 0.2;
    context.fillStyle = ORBIT_03;
    context.beginPath();
    context.arc(0, -26, domeRadius * 0.78, 0, Math.PI * 2);
    context.fill();
    context.globalAlpha = 1;
  }

  context.restore();
}
