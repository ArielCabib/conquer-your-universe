import {
  HARVESTER_SPAWN_ANIMATION_MS,
  ORBIT_01,
  ORBIT_02,
  ORBIT_03,
  ORBIT_04,
  ORBIT_05,
} from "../../constants";
import { HarvesterState } from "../../types";
import { easeOutQuad } from "../helpers";

export function drawHarvester(
  context: CanvasRenderingContext2D,
  harvester: HarvesterState,
  nowMs: number,
): void {
  const baseRadius = 18;
  const elapsed = Math.max(0, nowMs - harvester.builtMs);
  const spawnProgress = Math.min(1, elapsed / HARVESTER_SPAWN_ANIMATION_MS);
  const easedSpawn = easeOutQuad(spawnProgress);
  const rotationAngle = harvester.rotationAngle;

  let scale = 0.2 + 0.8 * easedSpawn;
  if (spawnProgress < 1) {
    scale += 0.12 * Math.sin(spawnProgress * Math.PI) * (1 - spawnProgress);
  }

  const alpha = 0.2 + 0.8 * easedSpawn;
  const haloStrength = 1 - easedSpawn;

  const outerRadius = baseRadius * scale;
  const innerRadius = baseRadius * 0.62 * scale;
  const coreRadius = baseRadius * 0.32 * scale;
  const legInnerRadius = baseRadius * 0.8 * scale;
  const legOuterRadius = baseRadius * 1.5 * scale;
  const crossRadius = baseRadius * 0.9 * scale;

  context.save();
  context.translate(harvester.x, harvester.y);

  if (haloStrength > 0) {
    context.globalAlpha = 0.18 + 0.4 * haloStrength;
    context.fillStyle = ORBIT_04;
    context.beginPath();
    context.arc(0, 0, baseRadius * (1.6 + 0.4 * haloStrength), 0, Math.PI * 2);
    context.fill();
    context.globalAlpha = 1;
  }

  context.rotate(rotationAngle);
  context.globalAlpha = alpha * (0.7 + 0.3 * harvester.spinLevel);

  context.fillStyle = ORBIT_02;
  context.beginPath();
  context.arc(0, 0, outerRadius, 0, Math.PI * 2);
  context.fill();

  context.fillStyle = ORBIT_03;
  context.beginPath();
  context.arc(0, 0, innerRadius, 0, Math.PI * 2);
  context.fill();

  context.strokeStyle = ORBIT_04;
  context.lineWidth = 3 * scale;
  context.beginPath();
  context.moveTo(-crossRadius, 0);
  context.lineTo(crossRadius, 0);
  context.moveTo(0, -crossRadius);
  context.lineTo(0, crossRadius);
  context.stroke();

  context.fillStyle = ORBIT_05;
  context.beginPath();
  context.arc(0, 0, coreRadius, 0, Math.PI * 2);
  context.fill();

  context.strokeStyle = ORBIT_01;
  context.lineWidth = 2 * scale;
  context.beginPath();
  for (let index = 0; index < 6; index += 1) {
    const angle = index * (Math.PI / 3) + Math.PI / 6;
    const innerX = Math.cos(angle) * legInnerRadius;
    const innerY = Math.sin(angle) * legInnerRadius;
    const outerX = Math.cos(angle) * legOuterRadius;
    const outerY = Math.sin(angle) * legOuterRadius;
    context.moveTo(innerX, innerY);
    context.lineTo(outerX, outerY);
  }
  context.stroke();

  context.globalAlpha = 1;
  context.restore();
}
