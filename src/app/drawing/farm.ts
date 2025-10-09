import { ORBIT_01, ORBIT_02, ORBIT_03, ORBIT_04, ORBIT_05 } from "../../constants";
import { FarmState } from "../../types";
import { easeOutQuad } from "../helpers";

const FARM_BUILD_HIGHLIGHT_MS = 1_200;
const FARM_PRODUCTION_PULSE_MS = 520;
const FARM_PRODUCTION_SCALE = 0.1;

function farmHighlightFactor(now: number, farm: FarmState): number {
  const buildElapsed = Math.max(0, now - farm.builtMs);
  let buildHighlight = 0;
  if (buildElapsed < FARM_BUILD_HIGHLIGHT_MS) {
    const buildProgress = Math.min(1, buildElapsed / FARM_BUILD_HIGHLIGHT_MS);
    buildHighlight = 1 - easeOutQuad(buildProgress);
  }

  let productionHighlight = 0;
  if (Number.isFinite(farm.lastProducedMs)) {
    const produceElapsed = Math.max(0, now - farm.lastProducedMs);
    if (produceElapsed < FARM_PRODUCTION_PULSE_MS) {
      const produceProgress = Math.min(1, produceElapsed / FARM_PRODUCTION_PULSE_MS);
      productionHighlight = 1 - easeOutQuad(produceProgress);
    }
  }

  return Math.max(buildHighlight, productionHighlight);
}

function farmProductionPulse(now: number, farm: FarmState): number {
  if (!Number.isFinite(farm.lastProducedMs)) {
    return 0;
  }

  const elapsed = Math.max(0, now - farm.lastProducedMs);
  if (elapsed >= FARM_PRODUCTION_PULSE_MS) {
    return 0;
  }

  const progress = Math.min(1, elapsed / FARM_PRODUCTION_PULSE_MS);
  return 1 - easeOutQuad(progress);
}

export function drawFarm(context: CanvasRenderingContext2D, farm: FarmState, nowMs: number): void {
  const highlight = farmHighlightFactor(nowMs, farm);
  const productionPulse = farmProductionPulse(nowMs, farm);
  const scale = 0.75 * (1 + FARM_PRODUCTION_SCALE * productionPulse);
  const baseOffsetY = 6 * scale;
  const highlightRadiusX = 44 * scale;
  const highlightRadiusY = 24 * scale;
  const highlightExpandX = 12 * scale;
  const highlightExpandY = 6 * scale;
  const soilRadiusX = 38 * scale;
  const soilRadiusY = 18 * scale;
  const rowTopY = -6 * scale;
  const rowMidY = 0;
  const rowBottomY = 6 * scale;
  const rowOuterX = 28 * scale;
  const rowMidX = 22 * scale;
  const rowInnerX = 18 * scale;

  context.save();
  context.translate(farm.x, farm.y);

  if (highlight > 0) {
    context.globalAlpha = 0.2 + 0.35 * highlight;
    context.fillStyle = ORBIT_04;
    context.beginPath();
    context.ellipse(
      0,
      baseOffsetY,
      highlightRadiusX + highlightExpandX * highlight,
      highlightRadiusY + highlightExpandY * highlight,
      0,
      0,
      Math.PI * 2,
    );
    context.fill();
    context.globalAlpha = 1;
  }

  context.fillStyle = ORBIT_02;
  context.beginPath();
  context.ellipse(0, baseOffsetY, highlightRadiusX, highlightRadiusY, 0, 0, Math.PI * 2);
  context.fill();

  context.fillStyle = ORBIT_01;
  context.beginPath();
  context.ellipse(0, 0, soilRadiusX, soilRadiusY, 0, 0, Math.PI * 2);
  context.fill();

  if (productionPulse > 0) {
    context.globalAlpha = 0.45 * productionPulse + 0.15;
    context.fillStyle = ORBIT_03;
    context.beginPath();
    context.ellipse(0, 0, soilRadiusX * 0.72, soilRadiusY * 0.72, 0, 0, Math.PI * 2);
    context.fill();
    context.globalAlpha = 1;
  }

  context.strokeStyle = ORBIT_05;
  context.lineWidth = 2 * scale;
  context.beginPath();
  context.moveTo(-rowOuterX, rowMidY);
  context.lineTo(rowOuterX, rowMidY);
  context.moveTo(-rowMidX, rowTopY);
  context.lineTo(rowMidX, rowTopY);
  context.moveTo(-rowInnerX, rowBottomY);
  context.lineTo(rowInnerX, rowBottomY);
  context.stroke();

  context.restore();
}
