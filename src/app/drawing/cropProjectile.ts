import { ORBIT_01, ORBIT_02, ORBIT_05 } from "../../constants";
import { CropProjectileState } from "../../types";
import { easeOutQuad } from "../helpers";

export function drawCropProjectile(
  context: CanvasRenderingContext2D,
  projectile: CropProjectileState,
  nowMs: number,
): void {
  const elapsed = nowMs - projectile.launchedMs;
  const duration = Math.max(1, projectile.durationMs);
  const progress = Math.max(0, Math.min(1, elapsed / duration));
  const eased = easeOutQuad(progress);

  const x = projectile.startX + (projectile.endX - projectile.startX) * eased;
  const baseY = projectile.startY + (projectile.endY - projectile.startY) * eased;
  const lift = Math.sin(progress * Math.PI) * 18;
  const y = baseY - lift;

  context.save();
  context.globalAlpha = 0.82;
  context.fillStyle = ORBIT_02;
  context.beginPath();
  context.ellipse(x, y, 6, 10, Math.PI / 8, 0, Math.PI * 2);
  context.fill();

  context.strokeStyle = ORBIT_05;
  context.lineWidth = 2.2;
  context.beginPath();
  context.ellipse(x, y, 4, 8, Math.PI / 8, 0, Math.PI * 2);
  context.stroke();

  context.fillStyle = ORBIT_01;
  context.globalAlpha = 0.35;
  context.beginPath();
  context.ellipse(x - 2, y - 4, 3, 4.5, Math.PI / 6, 0, Math.PI * 2);
  context.fill();
  context.globalAlpha = 1;
  context.restore();
}
