import { ORBIT_01, ORBIT_02, ORBIT_03, ORBIT_04, ORBIT_05 } from "../../constants";
import { MarketState } from "../../types";

export function drawMarket(
  context: CanvasRenderingContext2D,
  market: MarketState,
  nowMs: number,
): void {
  void nowMs;

  const baseWidth = 38;
  const baseHeight = 22;
  const roofHeight = 16;

  const baseX = market.x - baseWidth / 2;
  const baseY = market.y - baseHeight;

  context.save();

  context.globalAlpha = 0.28;
  context.fillStyle = ORBIT_01;
  context.beginPath();
  context.ellipse(market.x, market.y + 10, baseWidth * 0.9, 12, 0, 0, Math.PI * 2);
  context.fill();
  context.globalAlpha = 1;

  context.fillStyle = ORBIT_02;
  context.fillRect(baseX, baseY, baseWidth, baseHeight);

  context.fillStyle = ORBIT_04;
  const awningHeight = baseHeight * 0.45;
  context.fillRect(baseX, baseY, baseWidth, awningHeight);

  const stripeWidth = baseWidth / 4;
  context.fillStyle = ORBIT_03;
  for (let index = 0; index < 4; index += 1) {
    if (index % 2 === 0) {
      continue;
    }
    const stripeX = baseX + index * stripeWidth;
    context.fillRect(stripeX, baseY, stripeWidth, awningHeight);
  }

  context.fillStyle = ORBIT_05;
  context.beginPath();
  context.moveTo(baseX - 4, baseY);
  context.lineTo(market.x, baseY - roofHeight);
  context.lineTo(baseX + baseWidth + 4, baseY);
  context.closePath();
  context.fill();

  context.fillStyle = ORBIT_03;
  const counterHeight = baseHeight * 0.4;
  context.fillRect(baseX + 6, baseY + baseHeight - counterHeight, baseWidth - 12, counterHeight);

  context.fillStyle = ORBIT_01;
  const stallWidth = 10;
  const stallHeight = counterHeight * 0.7;
  context.fillRect(market.x - stallWidth / 2, baseY + baseHeight - stallHeight, stallWidth, stallHeight);

  context.restore();
}
