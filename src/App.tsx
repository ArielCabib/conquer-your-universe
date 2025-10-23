import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { AppView } from "./app/view/AppView";
import { ContextMenuState, InfoEntry, SimulationSnapshot } from "./app/types";
import { computeSimulationSnapshot, updateSimulationSnapshot } from "./app/state/simulationSnapshot";
import {
  useBuildFarmMenuHandler,
  useBuildHarvesterMenuHandler,
  useBuildHouseMenuHandler,
  useBuildMarketMenuHandler,
  useBuildResearcherMenuHandler,
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
import { getResearchNodeRequirements } from "./app/research/nodes";
import { STORAGE_KEY } from "./constants";
import { serializeGameState } from "./persistence";
import { createInitialGameState, GameState } from "./types";
type PromptKey = "explore" | "build" | "farm" | "harvester" | "market" | "researcher";

const PROMPT_MESSAGES: Record<PromptKey, string> = {
  explore: "Click around and find out",
  build: "Right click the planet to build a house",
  farm: "You can build a farm",
  harvester: "You can build a harvester",
  market: "You can build a market",
  researcher: "You can recruit a researcher",
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
    description: "Having a settler allows you to build a house. Houses expand your population capacity. What happens when you have more settlers?",
  },
  farm: {
    id: "farm",
    title: "Cultivate Farms",
    description: "Having at least ten settlers allows you to build a farm. Farms produce crops that can be processed into grains. What will you do with all those crops?",
  },
  harvester: {
    id: "harvester",
    title: "Deploy a Harvester",
    description: "Gather at least five crop bundles to assemble a harvester. It automates grain collection to keep supplies flowing. What will you do with all those grains?",
  },
  market: {
    id: "market",
    title: "Open the Market",
    description: "Stockpile thirty grains to build a market. Markets convert grains into coins. What will you do with all those coins?",
  },
  researcher: {
    id: "researcher",
    title: "Recruit a Researcher",
    description: "Accumulate fifty coins to recruit a researcher. You can unlock new technologies.",
  },
};

export function App() {
  const gameStateRef = useRef<GameState>(createInitialGameState());
  const [aliveCount, setAliveCount] = useState(0);
  const [simulationSnapshot, setSimulationSnapshot] = useState<SimulationSnapshot>(() =>
    computeSimulationSnapshot(gameStateRef.current),
  );
  const [planetName, setPlanetName] = useState(() => gameStateRef.current.planetName);
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const fileInputRef = useRef<HTMLInputElement | null>(null);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [isInfoModalOpen, setIsInfoModalOpen] = useState(false);
  const [isResearchViewActive, setIsResearchViewActive] = useState(false);
  const [isPaused, setIsPaused] = useState(false);
  const pauseTimeRef = useRef<number | null>(null);
  const [contextMenuState, setContextMenuState] = useState<ContextMenuState | null>(null);
  const [forcedPromptKey, setForcedPromptKey] = useState<PromptKey | null>(null);
  const [hasShownHarvesterPrompt, setHasShownHarvesterPrompt] = useState(false);
  const [hasShownMarketPrompt, setHasShownMarketPrompt] = useState(false);
  const [hasShownResearcherPrompt, setHasShownResearcherPrompt] = useState(false);
  const [infoEntryIds, setInfoEntryIds] = useState<string[]>(() => [
    ...gameStateRef.current.infoEntryIds,
  ]);
  const [researchProgressSnapshot, setResearchProgressSnapshot] = useState<Record<string, number>>(
    () => ({ ...gameStateRef.current.researchProgress }),
  );
  const infoEntries = useMemo(() => {
    return infoEntryIds
      .map((id) => PROMPT_INFORMATION[id as PromptKey])
      .filter((entry): entry is InfoEntry => Boolean(entry));
  }, [infoEntryIds]);

  const handleStateRestore = useCallback(
    (state: GameState) => {
      setPlanetName(state.planetName);
      setInfoEntryIds(() => [...state.infoEntryIds]);
      setSimulationSnapshot(computeSimulationSnapshot(state));
      setResearchProgressSnapshot({ ...state.researchProgress });
    },
    [setInfoEntryIds, setResearchProgressSnapshot, setSimulationSnapshot],
  );

  useRestoreState(gameStateRef, handleStateRestore);
  useCanvasRenderer(
    canvasRef,
    gameStateRef,
    setAliveCount,
    pauseTimeRef,
    setSimulationSnapshot,
    isResearchViewActive,
  );
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
    setSimulationSnapshot(computeSimulationSnapshot(gameStateRef.current));
    setResearchProgressSnapshot({ ...gameStateRef.current.researchProgress });
  }, [gameStateRef, restartGameHandler, setResearchProgressSnapshot, setSimulationSnapshot]);

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

  const buildResearcherFromMenu = useBuildResearcherMenuHandler({
    gameStateRef,
    contextMenuState,
    setContextMenuState,
  });

  const handleResearchNode = useCallback(
    (nodeId: string) => {
      const state = gameStateRef.current;
      if (state.completedResearchNodeIds.includes(nodeId)) {
        return;
      }

      const requirements = getResearchNodeRequirements(nodeId);
      const clickTarget = requirements?.clickCount ?? 0;
      const currentClicks = state.researchProgress[nodeId] ?? 0;
      const nextClicks =
        clickTarget > 0 ? Math.min(currentClicks + 1, clickTarget) : currentClicks + 1;

      if (state.researchProgress[nodeId] !== nextClicks) {
        state.researchProgress = {
          ...state.researchProgress,
          [nodeId]: nextClicks,
        };
      }

      const meetsClickRequirement = clickTarget === 0 || nextClicks >= clickTarget;
      const coinCost = requirements?.coinCost ?? 0;
      const hasRequiredCoins = state.coins >= coinCost;

      if (meetsClickRequirement && hasRequiredCoins) {
        const updatedCompleted = state.completedResearchNodeIds.includes(nodeId)
          ? state.completedResearchNodeIds
          : [...state.completedResearchNodeIds, nodeId];
        state.completedResearchNodeIds = updatedCompleted;

        if (coinCost > 0) {
          state.coins = Math.max(0, state.coins - coinCost);
        }
      }

      setResearchProgressSnapshot({ ...state.researchProgress });
      updateSimulationSnapshot(state, setSimulationSnapshot);

      try {
        if (typeof localStorage !== "undefined") {
          localStorage.setItem(STORAGE_KEY, serializeGameState(state));
        }
      } catch (error) {
        console.warn("Failed to persist game state after research attempt", error);
      }
    },
    [gameStateRef, setResearchProgressSnapshot, setSimulationSnapshot],
  );

  const toggleResearchView = useCallback(() => {
    setContextMenuState(null);
    setIsResearchViewActive((current) => !current);
  }, [setContextMenuState, setIsResearchViewActive]);

  const onFileChange = useFileChangeHandler({
    gameStateRef,
    setAliveCount,
    setIsModalOpen,
    setIsPaused,
    pauseTimeRef,
    setPlanetName,
    setInfoEntryIds,
    setSimulationSnapshot,
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
  const totalCrops = simulationSnapshot.cropCount;
  const hasHarvester = simulationSnapshot.hasHarvester;
  const canBuildHarvester = !hasHarvester && totalCrops >= 5;
  const grainCount = simulationSnapshot.grainCount;
  const grainsInFlight = simulationSnapshot.grainsInFlight;
  const grainCapacity = state.grainPileCapacity;
  const hasMarket = simulationSnapshot.hasMarket;
  const canBuildMarket = simulationSnapshot.hasGrainPile && grainCount >= 30 && !hasMarket;
  const hasResearcher = simulationSnapshot.hasResearcher;
  const coinCount = simulationSnapshot.coinCount;
  const coinCapacity = state.coinCapacity;
  const canBuildResearcher = coinCount >= 50 && !hasResearcher;
  const completedResearchNodeIds = state.completedResearchNodeIds;
  const hasContextMenuActions =
    canBuildHouse || canBuildFarm || canBuildHarvester || canBuildMarket || canBuildResearcher;

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
    if (canBuildResearcher) {
      if (!hasShownResearcherPrompt) {
        setForcedPromptKey("researcher");
        setHasShownResearcherPrompt(true);
      }
      return;
    }

    setHasShownResearcherPrompt(false);
    setForcedPromptKey((current) => (current === "researcher" ? null : current));
  }, [canBuildResearcher, hasShownResearcherPrompt]);

  useEffect(() => {
    if (!hasResearcher) {
      setIsResearchViewActive(false);
    }
  }, [hasResearcher]);

  useEffect(() => {
    if (!forcedPromptKey) {
      return;
    }

    if (
      forcedPromptKey !== "harvester" &&
      forcedPromptKey !== "market" &&
      forcedPromptKey !== "researcher"
    ) {
      return;
    }

    if (typeof window === "undefined") {
      return;
    }

    const timeoutId = window.setTimeout(() => {
      setForcedPromptKey((current) =>
        current === "harvester" || current === "market" || current === "researcher" ? null : current,
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
      canBuildResearcher={canBuildResearcher}
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
      onToggleResearchView={toggleResearchView}
      onResearchNode={handleResearchNode}
      onRestartGame={restartGame}
      onSaveGame={saveGame}
      settlersCapacityLimit={settlersCapacityLimit}
      promptMessage={promptMessage}
      onBuildHouseFromMenu={buildHouseFromMenu}
      onBuildFarmFromMenu={buildFarmFromMenu}
      onBuildHarvesterFromMenu={buildHarvesterFromMenu}
      onBuildMarketFromMenu={buildMarketFromMenu}
      onBuildResearcherFromMenu={buildResearcherFromMenu}
      settlerMinLifespanMs={settlerMinLifespanMs}
      settlerMaxLifespanMs={settlerMaxLifespanMs}
      farmLifespanBonusMs={farmLifespanBonusMs}
      farmCapacityLimit={farmCapacityLimit}
      houseSpawnIntervalMs={houseSpawnIntervalMs}
      houseSpawnAmount={houseSpawnAmount}
      onPlanetNameChange={handlePlanetNameChange}
      grainCount={grainCount}
      grainCapacity={grainCapacity}
      coinCapacity={coinCapacity}
      grainsInFlight={grainsInFlight}
      hasHarvester={hasHarvester}
      hasMarket={hasMarket}
      hasResearcher={hasResearcher}
      coinCount={coinCount}
      researchProgress={researchProgressSnapshot}
      infoEntries={infoEntries}
      isInfoModalActive={isInfoModalOpen}
      onCloseInfo={closeInfoModal}
      isResearchViewActive={isResearchViewActive}
      completedResearchNodeIds={completedResearchNodeIds}
    />
  );
}
