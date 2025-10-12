import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { AppView } from "./app/view/AppView";
import { ContextMenuState, InfoEntry } from "./app/types";
import {
  useBuildFarmMenuHandler,
  useBuildHarvesterMenuHandler,
  useBuildHouseMenuHandler,
  useBuildMarketMenuHandler,
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
type PromptKey = "explore" | "build" | "farm" | "harvester" | "market";

const PROMPT_MESSAGES: Record<PromptKey, string> = {
  explore: "Click around and find out",
  build: "Right click the planet to build a house",
  farm: "Right click the planet to build a farm",
  harvester: "You can build a harvester",
  market: "You can build a market",
};

const PROMPT_INFORMATION: Record<PromptKey, InfoEntry> = {
  explore: {
    id: "explore",
    title: "Scout the Surface",
    description: "Clicking the planet produces a settler.",
  },
  build: {
    id: "build",
    title: "Establish Housing",
    description: "Having a settler allows you to build a house. Houses expand your population capacity.",
  },
  farm: {
    id: "farm",
    title: "Cultivate Farms",
    description: "Having at least ten settlers allows you to build a farm. Farms produce crops that can be processed into grains.",
  },
  harvester: {
    id: "harvester",
    title: "Deploy a Harvester",
    description: "Gather at least five crop bundles to assemble a harvester. It automates grain collection to keep supplies flowing.",
  },
  market: {
    id: "market",
    title: "Open the Market",
    description: "Stockpile thirty grains to build a market. Markets convert grains into coins.",
  },
};

export function App() {
  const gameStateRef = useRef<GameState>(createInitialGameState());
  const [aliveCount, setAliveCount] = useState(0);
  const [planetName, setPlanetName] = useState(() => gameStateRef.current.planetName);
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const fileInputRef = useRef<HTMLInputElement | null>(null);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [isInfoModalOpen, setIsInfoModalOpen] = useState(false);
  const [isPaused, setIsPaused] = useState(false);
  const pauseTimeRef = useRef<number | null>(null);
  const [contextMenuState, setContextMenuState] = useState<ContextMenuState | null>(null);
  const [forcedPromptKey, setForcedPromptKey] = useState<PromptKey | null>(null);
  const [hasShownHarvesterPrompt, setHasShownHarvesterPrompt] = useState(false);
  const [hasShownMarketPrompt, setHasShownMarketPrompt] = useState(false);
  const [infoEntryIds, setInfoEntryIds] = useState<string[]>(() => [
    ...gameStateRef.current.infoEntryIds,
  ]);
  const infoEntries = useMemo(() => {
    return infoEntryIds
      .map((id) => PROMPT_INFORMATION[id as PromptKey])
      .filter((entry): entry is InfoEntry => Boolean(entry));
  }, [infoEntryIds]);

  const handleStateRestore = useCallback(
    (state: GameState) => {
      setPlanetName(state.planetName);
      setInfoEntryIds(() => [...state.infoEntryIds]);
    },
    [setInfoEntryIds],
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

  const openSettings = useModalOpenHandler(setIsModalOpen);
  const closeModal = useModalCloseHandler(setIsModalOpen);
  const openInfoModal = useModalOpenHandler(setIsInfoModalOpen);
  const closeInfoModal = useModalCloseHandler(setIsInfoModalOpen);

  const restartGameHandler = useRestartGameHandler({
    gameStateRef,
    setAliveCount,
    setIsModalOpen,
    setIsPaused,
    pauseTimeRef,
    setPlanetName,
  });

  const restartGame = useCallback(() => {
    restartGameHandler();
    setInfoEntryIds(() => {
      gameStateRef.current.infoEntryIds = [];
      return [];
    });
  }, [gameStateRef, restartGameHandler]);

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

  const buildHarvesterFromMenu = useBuildHarvesterMenuHandler({
    gameStateRef,
    contextMenuState,
    setContextMenuState,
  });

  const buildMarketFromMenu = useBuildMarketMenuHandler({
    gameStateRef,
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
    setInfoEntryIds,
  });

  const handlePlanetNameChange = useCallback(
    (name: string) => {
      gameStateRef.current.planetName = name;
      setPlanetName(name);
    },
    [gameStateRef],
  );

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
  const hasHarvester = Boolean(state.harvester);
  const totalCrops = state.crops.length;
  const canBuildHarvester = !hasHarvester && totalCrops >= 5;
  const grainPile = state.grainPile;
  const grainCount = grainPile?.grains ?? 0;
  const grainsInFlight =
    state.grainProjectiles.length + state.cropProjectiles.length + state.marketGrainProjectiles.length;
  const grainCapacity = state.grainPileCapacity;
  const hasMarket = Boolean(state.market);
  const canBuildMarket = Boolean(grainPile && grainPile.grains >= 30 && !hasMarket);
  const coinCount = state.coins ?? 0;
  const hasContextMenuActions =
    canBuildHouse || canBuildFarm || canBuildHarvester || canBuildMarket;

  const getHasContextMenuActions = useCallback(
    () => hasContextMenuActions,
    [hasContextMenuActions],
  );

  const handleContextMenu = useContextMenuHandler({
    canvasRef,
    isPaused,
    setContextMenuState,
    hasContextMenuActions: getHasContextMenuActions,
  });

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

  useEffect(() => {
    if (canBuildHarvester) {
      if (!hasShownHarvesterPrompt) {
        setForcedPromptKey("harvester");
        setHasShownHarvesterPrompt(true);
      }
      return;
    }

    setHasShownHarvesterPrompt(false);
    setForcedPromptKey((current) => (current === "harvester" ? null : current));
  }, [canBuildHarvester, hasShownHarvesterPrompt]);

  useEffect(() => {
    if (canBuildMarket) {
      if (!hasShownMarketPrompt) {
        setForcedPromptKey("market");
        setHasShownMarketPrompt(true);
      }
      return;
    }

    setHasShownMarketPrompt(false);
    setForcedPromptKey((current) => (current === "market" ? null : current));
  }, [canBuildMarket, hasShownMarketPrompt]);

  useEffect(() => {
    if (forcedPromptKey !== "harvester" && forcedPromptKey !== "market") {
      return;
    }

    if (typeof window === "undefined") {
      return;
    }

    const timeoutId = window.setTimeout(() => {
      setForcedPromptKey((current) =>
        current === "harvester" || current === "market" ? null : current,
      );
    }, 4_500);

    return () => {
      window.clearTimeout(timeoutId);
    };
  }, [forcedPromptKey]);

  const promptKey = forcedPromptKey ?? activePromptKey;
  const promptMessage = promptKey ? PROMPT_MESSAGES[promptKey] : null;

  useEffect(() => {
    if (!promptKey) {
      return;
    }

    const entry = PROMPT_INFORMATION[promptKey];
    if (!entry) {
      return;
    }

    setInfoEntryIds((current) => {
      if (current.includes(entry.id)) {
        gameStateRef.current.infoEntryIds = current;
        return current;
      }

      const updated = [...current, entry.id];
      gameStateRef.current.infoEntryIds = updated;
      return updated;
    });
  }, [gameStateRef, promptKey]);

  return (
    <AppView
      aliveNow={aliveCount}
      planetName={planetName}
      canBuildHouse={canBuildHouse}
      canBuildFarm={canBuildFarm}
      canBuildHarvester={canBuildHarvester}
      canBuildMarket={canBuildMarket}
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
      onOpenInfo={openInfoModal}
      onRestartGame={restartGame}
      onSaveGame={saveGame}
      settlersCapacityLimit={settlersCapacityLimit}
      promptMessage={promptMessage}
      onBuildHouseFromMenu={buildHouseFromMenu}
      onBuildFarmFromMenu={buildFarmFromMenu}
      onBuildHarvesterFromMenu={buildHarvesterFromMenu}
      onBuildMarketFromMenu={buildMarketFromMenu}
      settlerMinLifespanMs={settlerMinLifespanMs}
      settlerMaxLifespanMs={settlerMaxLifespanMs}
      farmLifespanBonusMs={farmLifespanBonusMs}
      farmCapacityLimit={farmCapacityLimit}
      houseSpawnIntervalMs={houseSpawnIntervalMs}
      houseSpawnAmount={houseSpawnAmount}
      onPlanetNameChange={handlePlanetNameChange}
      grainCount={grainCount}
      grainCapacity={grainCapacity}
      grainsInFlight={grainsInFlight}
      hasHarvester={hasHarvester}
      hasMarket={hasMarket}
      coinCount={coinCount}
      infoEntries={infoEntries}
      isInfoModalActive={isInfoModalOpen}
      onCloseInfo={closeInfoModal}
    />
  );
}
