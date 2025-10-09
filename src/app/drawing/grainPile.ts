import { GRAIN_DRAW_LIMIT, ORBIT_02, ORBIT_03, ORBIT_04, ORBIT_05 } from "../../constants";
import { GrainPileState } from "../../types";

const GRAIN_OFFSETS = [
  { x: -10, y: -12 },
  { x: 2, y: -18 },
  { x: 12, y: -14 },
  { x: -4, y: -4 },
  { x: 8, y: -6 },
];

export function drawGrainPile(context: CanvasRenderingContext2D, pile: GrainPileState): void {
  const baseRadiusX = 28;
  const baseRadiusY = 16;

  context.save();
  context.translate(pile.x, pile.y);

  context.globalAlpha = 0.4;
  context.fillStyle = ORBIT_02;
  context.beginPath();
  context.ellipse(0, baseRadiusY * 0.55, baseRadiusX * 0.95, baseRadiusY * 0.55, 0, 0, Math.PI * 2);
  context.fill();
  context.globalAlpha = 1;

  context.fillStyle = ORBIT_02;
  context.beginPath();
  context.ellipse(0, 0, baseRadiusX, baseRadiusY, 0, 0, Math.PI * 2);
  context.fill();

  const grainsToDraw = Math.min(pile.grains, GRAIN_DRAW_LIMIT);
  context.globalAlpha = 0.9;
  context.fillStyle = ORBIT_03;

  for (let index = 0; index < grainsToDraw; index += 1) {
    const offset = GRAIN_OFFSETS[index] ?? GRAIN_OFFSETS[GRAIN_OFFSETS.length - 1];
    context.beginPath();
    context.arc(offset.x, offset.y, 6, 0, Math.PI * 2);
    context.fill();

    context.fillStyle = ORBIT_05;
    context.beginPath();
    context.arc(offset.x - 2, offset.y - 2, 2.5, 0, Math.PI * 2);
    context.fill();
    context.fillStyle = ORBIT_03;
  }

  context.globalAlpha = 1;
  context.fillStyle = ORBIT_04;
  context.font = '1rem "Trebuchet MS", sans-serif';
  context.textAlign = "left";
  context.textBaseline = "bottom";
  context.fillText(`${pile.grains}`, baseRadiusX + 6, baseRadiusY + 4);

  context.restore();
}
