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
import { drawHouse, currentTimeMs } from "../../helpers";
import { handleActiveState } from "./active";
import { renderPausedState } from "./paused";

function renderPlanet(context: CanvasRenderingContext2D) {
  context.fillStyle = ORBIT_05;
  context.beginPath();
  context.arc(PLANET_CENTER_X, PLANET_CENTER_Y, PLANET_RADIUS, 0, Math.PI * 2);
  context.fill();
}

function renderHouses(
  context: CanvasRenderingContext2D,
  state: GameState,
  now: number,
) {
  for (const house of state.houses) {
    drawHouse(context, house, now);
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
        renderPausedState(context, gameStateRef.current, now, setAliveCount, renderHouses);
      } else {
        handleActiveState(context, gameStateRef.current, now, setAliveCount, renderHouses);
      }

      animationFrame = window.requestAnimationFrame(drawFrame);
    };

    drawFrame();

    return () => {
      window.cancelAnimationFrame(animationFrame);
    };
  }, [canvasRef, gameStateRef, pauseTimeRef, setAliveCount]);
}
