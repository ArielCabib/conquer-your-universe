import { ORBIT_03, ORBIT_04 } from "../../constants";
import { CoinProjectileState } from "../../types";
import { easeOutQuad } from "../helpers";

export function drawCoinProjectile(
  context: CanvasRenderingContext2D,
  projectile: CoinProjectileState,
  nowMs: number,
): void {
  const elapsed = nowMs - projectile.launchedMs;
  const duration = Math.max(1, projectile.durationMs);
  const progress = Math.max(0, Math.min(1, elapsed / duration));
  const eased = easeOutQuad(progress);

  const x = projectile.x;
  const y = projectile.startY + (projectile.endY - projectile.startY) * eased;
  const scale = 1 + 0.2 * (1 - eased);
  const opacity = 0.85 - eased * 0.35;

  context.save();
  context.globalAlpha = Math.max(0, Math.min(1, opacity));
  context.fillStyle = ORBIT_03;
  context.beginPath();
  context.ellipse(x, y, 6 * scale, 6 * scale * 0.72, 0, 0, Math.PI * 2);
  context.fill();

  context.fillStyle = ORBIT_04;
  context.beginPath();
  context.ellipse(x - 1, y - 2, 2.6 * scale, 2.6 * scale * 0.72, 0, 0, Math.PI * 2);
  context.fill();
  context.restore();
}
