import { ORBIT_01, ORBIT_02, ORBIT_03, ORBIT_04, ORBIT_05 } from "../../constants";
import { HarvesterState } from "../../types";

export function drawHarvester(
  context: CanvasRenderingContext2D,
  harvester: HarvesterState,
  _nowMs: number,
): void {
  const baseRadius = 18;

  context.save();
  context.translate(harvester.x, harvester.y);

  context.fillStyle = ORBIT_02;
  context.beginPath();
  context.arc(0, 0, baseRadius, 0, Math.PI * 2);
  context.fill();

  context.fillStyle = ORBIT_03;
  context.beginPath();
  context.arc(0, 0, baseRadius * 0.62, 0, Math.PI * 2);
  context.fill();

  context.strokeStyle = ORBIT_04;
  context.lineWidth = 3;
  context.beginPath();
  context.moveTo(-baseRadius * 0.9, 0);
  context.lineTo(baseRadius * 0.9, 0);
  context.moveTo(0, -baseRadius * 0.9);
  context.lineTo(0, baseRadius * 0.9);
  context.stroke();

  context.fillStyle = ORBIT_05;
  context.beginPath();
  context.arc(0, 0, baseRadius * 0.32, 0, Math.PI * 2);
  context.fill();

  const legInnerRadius = baseRadius * 0.8;
  const legOuterRadius = baseRadius * 1.5;
  context.strokeStyle = ORBIT_01;
  context.lineWidth = 2;
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

  context.restore();
}
