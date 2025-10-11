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
  canvasRef: RefObject<HTMLCanvasElement>;
  onCloseModal: MouseEventHandler<HTMLButtonElement>;
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
  pauseStatusText: string;
  onRestartGame: MouseEventHandler<HTMLButtonElement>;
  onSaveGame: MouseEventHandler<HTMLButtonElement>;
  settlersCapacityLimit: number;
  promptMessage: string | null;
  onBuildHouseFromMenu: MouseEventHandler<HTMLButtonElement>;
  onBuildFarmFromMenu: MouseEventHandler<HTMLButtonElement>;
  onBuildHarvesterFromMenu: MouseEventHandler<HTMLButtonElement>;
  onBuildMarketFromMenu: MouseEventHandler<HTMLButtonElement>;
  settlerMinLifespanMs: number;
  settlerMaxLifespanMs: number;
  farmLifespanBonusMs: number;
  houseSpawnIntervalMs: number;
  houseSpawnAmount: number;
  onPlanetNameChange: (name: string) => void;
  grainCount: number;
  grainCapacity: number;
  grainsInFlight: number;
  hasHarvester: boolean;
  hasMarket: boolean;
  infoEntries: InfoEntry[];
  isInfoModalActive: boolean;
  onCloseInfo: MouseEventHandler<HTMLButtonElement>;
}

export function AppView({
  aliveNow,
  planetName,
  canBuildHouse,
  canBuildFarm,
  canBuildHarvester,
  canBuildMarket,
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
  pauseStatusText,
  onRestartGame,
  onSaveGame,
  settlersCapacityLimit,
  promptMessage,
  onBuildHouseFromMenu,
  onBuildFarmFromMenu,
  onBuildHarvesterFromMenu,
  onBuildMarketFromMenu,
  settlerMinLifespanMs,
  settlerMaxLifespanMs,
  farmLifespanBonusMs,
  houseSpawnIntervalMs,
  houseSpawnAmount,
  onPlanetNameChange,
  grainCount,
  grainCapacity,
  grainsInFlight,
  hasHarvester,
  hasMarket,
  infoEntries,
  isInfoModalActive,
  onCloseInfo,
}: AppViewProps) {
  return (
    <main className="flex min-h-screen items-center justify-center bg-orbit-01">
      <section className="flex flex-col items-center gap-10 text-center">
        <HeaderSection
          onOpenSettings={onOpenSettings}
          onOpenInfo={onOpenInfo}
          planetName={planetName}
          onPlanetNameChange={onPlanetNameChange}
        />
        <BuildPrompt message={promptMessage} />
        <CanvasArea
          canvasRef={canvasRef}
          onClick={onClickCanvas}
          onContextMenu={onContextMenuCanvas}
          isPaused={isPaused}
          contextMenuState={contextMenuState}
          onBuildHouse={onBuildHouseFromMenu}
          canBuildHouse={canBuildHouse}
          onBuildFarm={onBuildFarmFromMenu}
          canBuildFarm={canBuildFarm}
          onBuildHarvester={onBuildHarvesterFromMenu}
          canBuildHarvester={canBuildHarvester}
          onBuildMarket={onBuildMarketFromMenu}
          canBuildMarket={canBuildMarket}
        />
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
          grainsInFlight={grainsInFlight}
          hasHarvester={hasHarvester}
          hasMarket={hasMarket}
        />
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
        pauseStatusText={pauseStatusText}
        onRestart={onRestartGame}
        onSave={onSaveGame}
        onOpenFile={onOpenFileDialog}
      />
      <InfoModal isActive={isInfoModalActive} onClose={onCloseInfo} entries={infoEntries} />
    </main>
  );
}
