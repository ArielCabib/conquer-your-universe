import type { CSSProperties, ChangeEventHandler, MouseEventHandler, RefObject } from "react";
import { ORBIT_01 } from "../../constants";
import { ContextMenuState } from "../types";
import { CanvasArea } from "./CanvasArea";
import { HeaderSection } from "./Header";
import { ControlModal } from "./Modal";
import { BuildPrompt } from "./Prompt";
import { StatsPanel } from "./Stats";

interface AppViewProps {
  aliveNow: number;
  canBuildHouse: boolean;
  canvasRef: RefObject<HTMLCanvasElement>;
  canvasStyle: CSSProperties;
  onCloseModal: MouseEventHandler<HTMLButtonElement>;
  contextMenuState: ContextMenuState | null;
  fileInputRef: RefObject<HTMLInputElement>;
  onClickCanvas: MouseEventHandler<HTMLCanvasElement>;
  onContextMenuCanvas: MouseEventHandler<HTMLCanvasElement>;
  housesBuilt: number;
  housesCapacityLimit: number;
  isModalActive: boolean;
  isPaused: boolean;
  onFileChange: ChangeEventHandler<HTMLInputElement>;
  onOpenFileDialog: MouseEventHandler<HTMLButtonElement>;
  onOpenSettings: MouseEventHandler<HTMLButtonElement>;
  pauseStatusText: string;
  onRestartGame: MouseEventHandler<HTMLButtonElement>;
  onSaveGame: MouseEventHandler<HTMLButtonElement>;
  settlersCapacityLimit: number;
  shouldShowBuildPrompt: boolean;
  onBuildHouseFromMenu: MouseEventHandler<HTMLButtonElement>;
  settlerMinLifespanMs: number;
  settlerMaxLifespanMs: number;
}

export function AppView({
  aliveNow,
  canBuildHouse,
  canvasRef,
  canvasStyle,
  onCloseModal,
  contextMenuState,
  fileInputRef,
  onClickCanvas,
  onContextMenuCanvas,
  housesBuilt,
  housesCapacityLimit,
  isModalActive,
  isPaused,
  onFileChange,
  onOpenFileDialog,
  onOpenSettings,
  pauseStatusText,
  onRestartGame,
  onSaveGame,
  settlersCapacityLimit,
  shouldShowBuildPrompt,
  onBuildHouseFromMenu,
  settlerMinLifespanMs,
  settlerMaxLifespanMs,
}: AppViewProps) {
  return (
    <main
      style={{
        backgroundColor: ORBIT_01,
        minHeight: "100vh",
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
      }}
    >
      <section
        style={{
          display: "flex",
          flexDirection: "column",
          alignItems: "center",
          gap: "2.5rem",
          textAlign: "center",
        }}
      >
        <HeaderSection onOpenSettings={onOpenSettings} />
        <BuildPrompt shouldShow={shouldShowBuildPrompt} />
        <CanvasArea
          canvasRef={canvasRef}
          canvasStyle={canvasStyle}
          onClick={onClickCanvas}
          onContextMenu={onContextMenuCanvas}
          isPaused={isPaused}
          contextMenuState={contextMenuState}
          onBuildHouse={onBuildHouseFromMenu}
          canBuildHouse={canBuildHouse}
        />
        <StatsPanel
          aliveNow={aliveNow}
          settlersCapacityLimit={settlersCapacityLimit}
          housesBuilt={housesBuilt}
          housesCapacityLimit={housesCapacityLimit}
          settlerMinLifespanMs={settlerMinLifespanMs}
          settlerMaxLifespanMs={settlerMaxLifespanMs}
        />
      </section>
      <input
        ref={fileInputRef}
        type="file"
        accept="application/json"
        style={{ display: "none" }}
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
    </main>
  );
}
