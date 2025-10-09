import {
  Dispatch,
  MutableRefObject,
  SetStateAction,
  useEffect,
} from "react";
import {
  ORBIT_05,
  PLANET_CENTER_X,
  PLANET_CENTER_Y,
  PLANET_RADIUS,
  VIEWBOX_HEIGHT,
  VIEWBOX_WIDTH,
} from "../../../constants";
import { GameState } from "../../../types";
import { currentTimeMs } from "../../helpers";
import {
  drawCrop,
  drawFarm,
  drawGrainPile,
  drawGrainProjectile,
  drawHarvester,
  drawHouse,
  drawMarket,
} from "../../drawing";
import { handleActiveState } from "./active";
import { renderPausedState } from "./paused";

function renderPlanet(context: CanvasRenderingContext2D) {
  context.fillStyle = ORBIT_05;
  context.beginPath();
  context.arc(PLANET_CENTER_X, PLANET_CENTER_Y, PLANET_RADIUS, 0, Math.PI * 2);
  context.fill();
}

function renderStructures(
  context: CanvasRenderingContext2D,
  state: GameState,
  now: number,
) {
  for (const farm of state.farms) {
    drawFarm(context, farm, now);
  }
  for (const crop of state.crops) {
    drawCrop(context, crop, now);
  }
  if (state.grainPile) {
    drawGrainPile(context, state.grainPile);
  }
  if (state.harvester) {
    drawHarvester(context, state.harvester, now);
  }
  for (const projectile of state.grainProjectiles) {
    drawGrainProjectile(context, projectile, now);
  }
  for (const house of state.houses) {
    drawHouse(context, house, now);
  }
  if (state.market) {
    drawMarket(context, state.market, now);
  }
}

export function useCanvasRenderer(
  canvasRef: MutableRefObject<HTMLCanvasElement | null>,
  gameStateRef: MutableRefObject<GameState>,
  setAliveCount: Dispatch<SetStateAction<number>>,
  pauseTimeRef: MutableRefObject<number | null>,
) {
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas || typeof window === "undefined") {
      return;
    }

    canvas.width = VIEWBOX_WIDTH;
    canvas.height = VIEWBOX_HEIGHT;

    const context = canvas.getContext("2d");
    if (!context) {
      return;
    }

    let animationFrame = 0;

    const drawFrame = () => {
      const pausedTime = pauseTimeRef.current;
      const now = typeof pausedTime === "number" ? pausedTime : currentTimeMs();
      const isPaused = typeof pausedTime === "number";

      context.clearRect(0, 0, VIEWBOX_WIDTH, VIEWBOX_HEIGHT);
      renderPlanet(context);

      if (isPaused) {
        renderPausedState(context, gameStateRef.current, now, setAliveCount, renderStructures);
      } else {
        handleActiveState(context, gameStateRef.current, now, setAliveCount, renderStructures);
      }

      animationFrame = window.requestAnimationFrame(drawFrame);
    };

    drawFrame();

    return () => {
      window.cancelAnimationFrame(animationFrame);
    };
  }, [canvasRef, gameStateRef, pauseTimeRef, setAliveCount]);
}
