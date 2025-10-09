import { ORBIT_02, ORBIT_03, ORBIT_04, ORBIT_05 } from "../../constants";
import { CropState } from "../../types";
import { easeOutQuad } from "../helpers";

const CROP_POP_DURATION_MS = 360;
const CROP_BASE_RADIUS = 6.5;

export function drawCrop(context: CanvasRenderingContext2D, crop: CropState, nowMs: number): void {
  const elapsed = Math.max(0, nowMs - crop.createdMs);
  const progress = Math.min(1, elapsed / CROP_POP_DURATION_MS);
  const eased = easeOutQuad(progress);
  const radius = CROP_BASE_RADIUS * (0.35 + 0.65 * eased);
  const haloRadius = radius * (1.6 - 0.4 * eased);

  context.save();
  context.translate(crop.x, crop.y);

  if (progress < 1) {
    context.globalAlpha = 0.35 * (1 - eased) + 0.2;
    context.fillStyle = ORBIT_04;
    context.beginPath();
    context.arc(0, 0, haloRadius, 0, Math.PI * 2);
    context.fill();
    context.globalAlpha = 1;
  }

  context.fillStyle = ORBIT_03;
  context.beginPath();
  context.arc(0, 0, radius, 0, Math.PI * 2);
  context.fill();

  context.globalAlpha = 0.6;
  context.fillStyle = ORBIT_02;
  context.beginPath();
  context.arc(-radius * 0.2, -radius * 0.2, radius * 0.55, 0, Math.PI * 2);
  context.fill();
  context.globalAlpha = 1;

  context.fillStyle = ORBIT_05;
  context.beginPath();
  context.arc(radius * 0.35, radius * 0.25, radius * 0.35, 0, Math.PI * 2);
  context.fill();

  context.restore();
}
