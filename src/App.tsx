import { useCallback, useEffect, useRef, useState } from "react";
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
import { createInitialGameState, GameState } from "./types";

type PromptKey = "explore" | "build" | "farm";

const PROMPT_MESSAGES: Record<PromptKey, string> = {
  explore: "Click around and find out",
  build: "Right click the planet to build a house",
  farm: "Right click the planet to build a farm",
};

export function App() {
  const gameStateRef = useRef<GameState>(createInitialGameState());
  const [aliveCount, setAliveCount] = useState(0);
  const [planetName, setPlanetName] = useState(() => gameStateRef.current.planetName);
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const fileInputRef = useRef<HTMLInputElement | null>(null);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [isPaused, setIsPaused] = useState(false);
  const pauseTimeRef = useRef<number | null>(null);
  const [contextMenuState, setContextMenuState] = useState<ContextMenuState | null>(null);

  const handleStateRestore = useCallback(
    (state: GameState) => {
      setPlanetName(state.planetName);
    },
    [],
  );

  useRestoreState(gameStateRef, handleStateRestore);
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
    setPlanetName,
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
    setPlanetName,
  });

  const handlePlanetNameChange = useCallback(
    (name: string) => {
      gameStateRef.current.planetName = name;
      setPlanetName(name);
    },
    [gameStateRef],
  );

  const pauseStatusText = isPaused ? "Time is currently paused." : "Time is currently running.";
  const state = gameStateRef.current;
  const housesBuilt = state.houses.length;
  const farmsBuilt = state.farms.length;
  const settlersBaseCapacity = state.settlersBaseCapacity;
  const housesCapacityLimit = state.housesBaseCapacity;
  const farmCapacityLimit = state.farmsBaseCapacity;
  const settlersPerHouse = state.settlersPerHouse;
  const farmLifespanBonusPerFarmMs = state.farmLifespanBonusPerFarmMs;
  const houseSpawnIntervalMs = state.houseSpawnIntervalMs;
  const houseSpawnAmount = state.houseSpawnAmount;
  const settlerMinLifespanMs = state.settlerMinLifespanMs;
  const settlerMaxLifespanMs = state.settlerMaxLifespanMs;
  const farmLifespanBonusMs = farmsBuilt * farmLifespanBonusPerFarmMs;

  const [activePromptKey, setActivePromptKey] = useState<PromptKey | null>(null);

  const hasHouseCapacity = housesCapacityLimit === 0 || housesBuilt < housesCapacityLimit;
  const canBuildHouse = aliveCount >= 1 && hasHouseCapacity;
  const hasFarmCapacity = farmCapacityLimit === 0 || farmsBuilt < farmCapacityLimit;
  const canBuildFarm = aliveCount >= 10 && hasFarmCapacity;
  const farmBuildDisabledReason = canBuildFarm
    ? undefined
    : aliveCount < 10
    ? "Requires at least 10 settlers"
    : hasFarmCapacity
    ? undefined
    : `Farm limit reached (${farmCapacityLimit})`;

  const settlersCapacityLimit = settlersBaseCapacity + housesBuilt * settlersPerHouse;
  const shouldShowBuildPrompt = aliveCount >= 1 && housesBuilt === 0;
  const shouldShowFarmPrompt = aliveCount >= 10 && farmsBuilt === 0;
  const shouldShowExplorePrompt = aliveCount === 0;

  useEffect(() => {
    setActivePromptKey((current) => {
      const isCurrentVisible =
        (current === "explore" && shouldShowExplorePrompt) ||
        (current === "build" && shouldShowBuildPrompt) ||
        (current === "farm" && shouldShowFarmPrompt);

      if (isCurrentVisible) {
        return current;
      }

      if (shouldShowExplorePrompt) {
        return "explore";
      }

      if (shouldShowBuildPrompt) {
        return "build";
      }

      if (shouldShowFarmPrompt) {
        return "farm";
      }

      return null;
    });
  }, [shouldShowExplorePrompt, shouldShowBuildPrompt, shouldShowFarmPrompt]);

  const promptMessage = activePromptKey ? PROMPT_MESSAGES[activePromptKey] : null;

  return (
    <AppView
      aliveNow={aliveCount}
      planetName={planetName}
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
      promptMessage={promptMessage}
      onBuildHouseFromMenu={buildHouseFromMenu}
      onBuildFarmFromMenu={buildFarmFromMenu}
      settlerMinLifespanMs={settlerMinLifespanMs}
      settlerMaxLifespanMs={settlerMaxLifespanMs}
      farmLifespanBonusMs={farmLifespanBonusMs}
      farmCapacityLimit={farmCapacityLimit}
      houseSpawnIntervalMs={houseSpawnIntervalMs}
      houseSpawnAmount={houseSpawnAmount}
      farmBuildDisabledReason={farmBuildDisabledReason}
      onPlanetNameChange={handlePlanetNameChange}
    />
  );
}
