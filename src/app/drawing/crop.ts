import { ORBIT_01, ORBIT_02, ORBIT_03, ORBIT_05 } from "../../constants";
import { CropState } from "../../types";
import { easeOutQuad } from "../helpers";

const CROP_POP_DURATION_MS = 360;
const CROP_HEIGHT = 26;
const CROP_WIDTH = 18;

export function drawCrop(context: CanvasRenderingContext2D, crop: CropState, nowMs: number): void {
  const elapsed = Math.max(0, nowMs - crop.createdMs);
  const progress = Math.min(1, elapsed / CROP_POP_DURATION_MS);
  const eased = easeOutQuad(progress);
  const scale = 0.35 + 0.65 * eased;

  context.save();
  context.translate(crop.x, crop.y);
  context.scale(scale, scale);

  context.fillStyle = ORBIT_02;
  context.beginPath();
  context.moveTo(-CROP_WIDTH * 0.5, 0);
  context.quadraticCurveTo(-CROP_WIDTH * 0.45, -CROP_HEIGHT * 0.35, -CROP_WIDTH * 0.35, -CROP_HEIGHT);
  context.quadraticCurveTo(-CROP_WIDTH * 0.6, -CROP_HEIGHT * 0.35, -CROP_WIDTH * 0.45, 0);
  context.closePath();
  context.fill();

  context.fillStyle = ORBIT_01;
  context.beginPath();
  context.moveTo(0, 0);
  context.quadraticCurveTo(CROP_WIDTH * 0.05, -CROP_HEIGHT * 0.35, 0, -CROP_HEIGHT * 1.05);
  context.quadraticCurveTo(-CROP_WIDTH * 0.15, -CROP_HEIGHT * 0.3, -CROP_WIDTH * 0.05, 0);
  context.closePath();
  context.fill();

  context.fillStyle = ORBIT_03;
  context.beginPath();
  context.moveTo(CROP_WIDTH * 0.45, 0);
  context.quadraticCurveTo(CROP_WIDTH * 0.35, -CROP_HEIGHT * 0.4, CROP_WIDTH * 0.25, -CROP_HEIGHT * 0.95);
  context.quadraticCurveTo(CROP_WIDTH * 0.55, -CROP_HEIGHT * 0.45, CROP_WIDTH * 0.5, 0);
  context.closePath();
  context.fill();

  context.fillStyle = ORBIT_05;
  context.globalAlpha = 0.65;
  context.beginPath();
  context.moveTo(CROP_WIDTH * 0.1, -CROP_HEIGHT * 0.25);
  context.quadraticCurveTo(CROP_WIDTH * 0.05, -CROP_HEIGHT * 0.55, CROP_WIDTH * 0.2, -CROP_HEIGHT);
  context.quadraticCurveTo(CROP_WIDTH * 0.25, -CROP_HEIGHT * 0.35, CROP_WIDTH * 0.2, -CROP_HEIGHT * 0.1);
  context.closePath();
  context.fill();
  context.globalAlpha = 1;

  context.restore();
}
