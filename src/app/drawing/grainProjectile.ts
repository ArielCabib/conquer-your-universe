import { ORBIT_03, ORBIT_04 } from "../../constants";
import { GrainProjectileState } from "../../types";
import { easeOutQuad } from "../helpers";

export function drawGrainProjectile(
  context: CanvasRenderingContext2D,
  projectile: GrainProjectileState,
  nowMs: number,
): void {
  const elapsed = nowMs - projectile.launchedMs;
  const duration = Math.max(1, projectile.durationMs);
  const progress = Math.max(0, Math.min(1, elapsed / duration));
  const eased = easeOutQuad(progress);

  const x = projectile.startX + (projectile.endX - projectile.startX) * eased;
  const baseY = projectile.startY + (projectile.endY - projectile.startY) * eased;
  const arcHeight = Math.sin(progress * Math.PI) * 22;
  const y = baseY - arcHeight;

  context.save();
  context.globalAlpha = 0.9;
  context.fillStyle = ORBIT_03;
  context.beginPath();
  context.arc(x, y, 5, 0, Math.PI * 2);
  context.fill();

  context.fillStyle = ORBIT_04;
  context.beginPath();
  context.arc(x - 2, y - 2, 2.4, 0, Math.PI * 2);
  context.fill();
  context.globalAlpha = 1;
  context.restore();
}
