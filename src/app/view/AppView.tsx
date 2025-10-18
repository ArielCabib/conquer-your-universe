import type { ChangeEventHandler, MouseEventHandler, RefObject } from "react";
import { ContextMenuState, InfoEntry } from "../types";
import { CanvasArea } from "./CanvasArea";
import { HeaderSection } from "./Header";
import { ControlModal, InfoModal } from "./Modal";
import { BuildPrompt } from "./Prompt";
import { StatsPanel } from "./Stats";

interface AppViewProps {
  aliveNow: number;
  planetName: string;
  canBuildHouse: boolean;
  canBuildFarm: boolean;
  canBuildHarvester: boolean;
  canBuildMarket: boolean;
  canBuildResearcher: boolean;
  canvasRef: RefObject<HTMLCanvasElement>;
  onCloseModal: () => void;
  contextMenuState: ContextMenuState | null;
  fileInputRef: RefObject<HTMLInputElement>;
  onClickCanvas: MouseEventHandler<HTMLCanvasElement>;
  onContextMenuCanvas: MouseEventHandler<HTMLCanvasElement>;
  housesBuilt: number;
  housesCapacityLimit: number;
  farmsBuilt: number;
  farmCapacityLimit: number;
  isModalActive: boolean;
  isPaused: boolean;
  onFileChange: ChangeEventHandler<HTMLInputElement>;
  onOpenFileDialog: MouseEventHandler<HTMLButtonElement>;
  onOpenSettings: MouseEventHandler<HTMLButtonElement>;
  onOpenInfo: MouseEventHandler<HTMLButtonElement>;
  onToggleResearchView: MouseEventHandler<HTMLButtonElement>;
  onRestartGame: MouseEventHandler<HTMLButtonElement>;
  onSaveGame: MouseEventHandler<HTMLButtonElement>;
  settlersCapacityLimit: number;
  promptMessage: string | null;
  onBuildHouseFromMenu: MouseEventHandler<HTMLButtonElement>;
  onBuildFarmFromMenu: MouseEventHandler<HTMLButtonElement>;
  onBuildHarvesterFromMenu: MouseEventHandler<HTMLButtonElement>;
  onBuildMarketFromMenu: MouseEventHandler<HTMLButtonElement>;
  onBuildResearcherFromMenu: MouseEventHandler<HTMLButtonElement>;
  settlerMinLifespanMs: number;
  settlerMaxLifespanMs: number;
  farmLifespanBonusMs: number;
  houseSpawnIntervalMs: number;
  houseSpawnAmount: number;
  onPlanetNameChange: (name: string) => void;
  grainCount: number;
  grainCapacity: number;
  coinCapacity: number;
  grainsInFlight: number;
  hasHarvester: boolean;
  hasMarket: boolean;
  hasResearcher: boolean;
  coinCount: number;
  infoEntries: InfoEntry[];
  isInfoModalActive: boolean;
  onCloseInfo: () => void;
  isResearchViewActive: boolean;
}

export function AppView({
  aliveNow,
  planetName,
  canBuildHouse,
  canBuildFarm,
  canBuildHarvester,
  canBuildMarket,
  canBuildResearcher,
  canvasRef,
  onCloseModal,
  contextMenuState,
  fileInputRef,
  onClickCanvas,
  onContextMenuCanvas,
  housesBuilt,
  housesCapacityLimit,
  farmsBuilt,
  farmCapacityLimit,
  isModalActive,
  isPaused,
  onFileChange,
  onOpenFileDialog,
  onOpenSettings,
  onOpenInfo,
  onToggleResearchView,
  onRestartGame,
  onSaveGame,
  settlersCapacityLimit,
  promptMessage,
  onBuildHouseFromMenu,
  onBuildFarmFromMenu,
  onBuildHarvesterFromMenu,
  onBuildMarketFromMenu,
  onBuildResearcherFromMenu,
  settlerMinLifespanMs,
  settlerMaxLifespanMs,
  farmLifespanBonusMs,
  houseSpawnIntervalMs,
  houseSpawnAmount,
  onPlanetNameChange,
  grainCount,
  grainCapacity,
  coinCapacity,
  grainsInFlight,
  hasHarvester,
  hasMarket,
  hasResearcher,
  coinCount,
  infoEntries,
  isInfoModalActive,
  onCloseInfo,
  isResearchViewActive,
}: AppViewProps) {
  return (
    <main className="flex min-h-screen items-center justify-center bg-orbit-01">
      <section className="flex flex-col items-center gap-10 text-center">
        <HeaderSection
          onOpenSettings={onOpenSettings}
          onOpenInfo={onOpenInfo}
          onToggleResearchView={onToggleResearchView}
          planetName={planetName}
          onPlanetNameChange={onPlanetNameChange}
          hasResearcher={hasResearcher}
          isResearchViewActive={isResearchViewActive}
        />
        <BuildPrompt message={promptMessage} />
        <CanvasArea
          canvasRef={canvasRef}
          onClick={onClickCanvas}
          onContextMenu={onContextMenuCanvas}
          isPaused={isPaused}
          isResearchViewActive={isResearchViewActive}
          contextMenuState={contextMenuState}
          onBuildHouse={onBuildHouseFromMenu}
          canBuildHouse={canBuildHouse}
          onBuildFarm={onBuildFarmFromMenu}
          canBuildFarm={canBuildFarm}
          onBuildHarvester={onBuildHarvesterFromMenu}
          canBuildHarvester={canBuildHarvester}
          onBuildMarket={onBuildMarketFromMenu}
          canBuildMarket={canBuildMarket}
          onBuildResearcher={onBuildResearcherFromMenu}
          canBuildResearcher={canBuildResearcher}
        />
        <div
          className={`w-full overflow-hidden transition-all duration-500 ease-out ${
            isResearchViewActive
              ? "max-h-0 opacity-0 pointer-events-none"
              : "max-h-[640px] opacity-100"
          }`}
          aria-hidden={isResearchViewActive}
        >
          <StatsPanel
            aliveNow={aliveNow}
            settlersCapacityLimit={settlersCapacityLimit}
            housesBuilt={housesBuilt}
            housesCapacityLimit={housesCapacityLimit}
            farmsBuilt={farmsBuilt}
            farmCapacityLimit={farmCapacityLimit}
            settlerMinLifespanMs={settlerMinLifespanMs}
            settlerMaxLifespanMs={settlerMaxLifespanMs}
            farmLifespanBonusMs={farmLifespanBonusMs}
            houseSpawnIntervalMs={houseSpawnIntervalMs}
            houseSpawnAmount={houseSpawnAmount}
            grainCount={grainCount}
            grainCapacity={grainCapacity}
            coinCapacity={coinCapacity}
            grainsInFlight={grainsInFlight}
            hasHarvester={hasHarvester}
            hasMarket={hasMarket}
            hasResearcher={hasResearcher}
            coinCount={coinCount}
          />
        </div>
      </section>
      <input
        ref={fileInputRef}
        type="file"
        accept="application/json"
        className="hidden"
        onChange={onFileChange}
      />
      <ControlModal
        isActive={isModalActive}
        onClose={onCloseModal}
        onRestart={onRestartGame}
        onSave={onSaveGame}
        onOpenFile={onOpenFileDialog}
      />
      <InfoModal isActive={isInfoModalActive} onClose={onCloseInfo} entries={infoEntries} />
    </main>
  );
}
