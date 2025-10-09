import { useRef, useState } from "react";
import { AppView } from "./app/view/AppView";
import { ContextMenuState } from "./app/types";
import {
  useBuildFarmMenuHandler,
  useBuildHouseMenuHandler,
  useCanvasClickHandler,
  useContextMenuHandler,
  useFileChangeHandler,
  useModalCloseHandler,
  useModalOpenHandler,
  useOpenFileDialogHandler,
  useRestartGameHandler,
  useSaveGameHandler,
} from "./app/handlers";
import { useCanvasRenderer } from "./app/effects/render";
import { usePeriodicSave } from "./app/effects/usePeriodicSave";
import { useRestoreState } from "./app/effects/useRestoreState";
import { FARM_LIFESPAN_BONUS_MS } from "./constants";
import { createInitialGameState, GameState } from "./types";

export function App() {
  const gameStateRef = useRef<GameState>(createInitialGameState());
  const [aliveCount, setAliveCount] = useState(0);
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const fileInputRef = useRef<HTMLInputElement | null>(null);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [isPaused, setIsPaused] = useState(false);
  const pauseTimeRef = useRef<number | null>(null);
  const [contextMenuState, setContextMenuState] = useState<ContextMenuState | null>(null);

  useRestoreState(gameStateRef);
  useCanvasRenderer(canvasRef, gameStateRef, setAliveCount, pauseTimeRef);
  usePeriodicSave(gameStateRef);

  const handleClick = useCanvasClickHandler({
    gameStateRef,
    canvasRef,
    isPaused,
    setContextMenuState,
    aliveCount,
    setAliveCount,
  });

  const handleContextMenu = useContextMenuHandler({
    canvasRef,
    isPaused,
    setContextMenuState,
  });

  const openSettings = useModalOpenHandler(setIsModalOpen);
  const closeModal = useModalCloseHandler(setIsModalOpen);

  const restartGame = useRestartGameHandler({
    gameStateRef,
    setAliveCount,
    setIsModalOpen,
    setIsPaused,
    pauseTimeRef,
  });

  const openFileDialog = useOpenFileDialogHandler(fileInputRef);
  const saveGame = useSaveGameHandler(gameStateRef);

  const buildHouseFromMenu = useBuildHouseMenuHandler({
    gameStateRef,
    aliveCount,
    contextMenuState,
    setContextMenuState,
  });

  const buildFarmFromMenu = useBuildFarmMenuHandler({
    gameStateRef,
    aliveCount,
    contextMenuState,
    setContextMenuState,
  });

  const onFileChange = useFileChangeHandler({
    gameStateRef,
    setAliveCount,
    setIsModalOpen,
    setIsPaused,
    pauseTimeRef,
  });

  const pauseStatusText = isPaused ? "Time is currently paused." : "Time is currently running.";
  const state = gameStateRef.current;
  const housesBuilt = state.houses.length;
  const farmsBuilt = state.farms.length;
  const settlersBaseCapacity = state.settlersBaseCapacity;
  const housesCapacityLimit = state.housesBaseCapacity;
  const settlersPerHouse = state.settlersPerHouse;
  const settlerMinLifespanMs = state.settlerMinLifespanMs;
  const settlerMaxLifespanMs = state.settlerMaxLifespanMs;
  const farmLifespanBonusMs = farmsBuilt * FARM_LIFESPAN_BONUS_MS;

  const hasHouseCapacity = housesCapacityLimit === 0 || housesBuilt < housesCapacityLimit;
  const canBuildHouse = aliveCount >= 1 && hasHouseCapacity;
  const canBuildFarm = aliveCount >= 10;

  const settlersCapacityLimit = settlersBaseCapacity + housesBuilt * settlersPerHouse;
  const shouldShowBuildPrompt = aliveCount >= 1 && housesBuilt === 0;

  return (
    <AppView
      aliveNow={aliveCount}
      canBuildHouse={canBuildHouse}
      canBuildFarm={canBuildFarm}
      canvasRef={canvasRef}
      onCloseModal={closeModal}
      contextMenuState={contextMenuState}
      fileInputRef={fileInputRef}
      onClickCanvas={handleClick}
      onContextMenuCanvas={handleContextMenu}
      housesBuilt={housesBuilt}
      housesCapacityLimit={housesCapacityLimit}
      farmsBuilt={farmsBuilt}
      isModalActive={isModalOpen}
      isPaused={isPaused}
      onFileChange={onFileChange}
      onOpenFileDialog={openFileDialog}
      onOpenSettings={openSettings}
      pauseStatusText={pauseStatusText}
      onRestartGame={restartGame}
      onSaveGame={saveGame}
      settlersCapacityLimit={settlersCapacityLimit}
      shouldShowBuildPrompt={shouldShowBuildPrompt}
      onBuildHouseFromMenu={buildHouseFromMenu}
      onBuildFarmFromMenu={buildFarmFromMenu}
      settlerMinLifespanMs={settlerMinLifespanMs}
      settlerMaxLifespanMs={settlerMaxLifespanMs}
      farmLifespanBonusMs={farmLifespanBonusMs}
    />
  );
}
