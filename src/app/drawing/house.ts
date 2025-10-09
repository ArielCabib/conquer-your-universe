import { ORBIT_01, ORBIT_02, ORBIT_03, ORBIT_04, ORBIT_05 } from "../../constants";
import { HouseState, houseSpawnHighlight } from "../../types";

export function drawHouse(
  context: CanvasRenderingContext2D,
  house: HouseState,
  nowMs: number,
): void {
  const highlightFactor = houseSpawnHighlight(nowMs, house);

  if (highlightFactor > 0) {
    const haloRadius = 36 + 18 * highlightFactor;
    context.globalAlpha = 0.45 * highlightFactor;
    context.fillStyle = ORBIT_04;
    context.beginPath();
    context.arc(house.x, house.y + 4, haloRadius, 0, Math.PI * 2);
    context.fill();
    context.globalAlpha = 1;
  }

  const baseWidth = 28;
  const baseHeight = 18;
  const roofHeight = 14;

  const baseX = house.x - baseWidth / 2;
  const baseY = house.y - baseHeight / 2;

  context.save();
  context.fillStyle = ORBIT_01;
  context.fillRect(baseX, baseY, baseWidth, baseHeight);

  if (highlightFactor > 0) {
    context.globalAlpha = 0.35 * highlightFactor + 0.2;
    context.fillStyle = ORBIT_05;
    context.fillRect(baseX, baseY, baseWidth, baseHeight);
    context.globalAlpha = 1;
  }

  context.fillStyle = ORBIT_02;
  context.beginPath();
  context.moveTo(baseX - 2, baseY);
  context.lineTo(house.x, baseY - roofHeight);
  context.lineTo(baseX + baseWidth + 2, baseY);
  context.closePath();
  context.fill();

  if (highlightFactor > 0) {
    context.globalAlpha = 0.45 * highlightFactor + 0.15;
    context.beginPath();
    context.moveTo(baseX - 2, baseY);
    context.lineTo(house.x, baseY - roofHeight - 4 * highlightFactor);
    context.lineTo(baseX + baseWidth + 2, baseY);
    context.closePath();
    context.fillStyle = ORBIT_03;
    context.fill();
    context.globalAlpha = 1;
  }

  context.fillStyle = ORBIT_05;
  const windowSize = baseWidth * 0.22;
  const windowY = baseY + baseHeight * 0.28;
  context.fillRect(baseX + baseWidth * 0.16, windowY, windowSize, windowSize);
  context.fillRect(
    baseX + baseWidth - windowSize - baseWidth * 0.16,
    windowY,
    windowSize,
    windowSize,
  );

  context.fillStyle = ORBIT_04;
  const doorWidth = baseWidth * 0.28;
  const doorHeight = baseHeight * 0.62;
  context.fillRect(house.x - doorWidth / 2, baseY + baseHeight - doorHeight, doorWidth, doorHeight);

  context.restore();
}
