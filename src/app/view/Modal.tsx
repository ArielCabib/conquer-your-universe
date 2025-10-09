import type { MouseEventHandler } from "react";

interface ControlModalProps {
  isActive: boolean;
  onClose: MouseEventHandler<HTMLButtonElement>;
  pauseStatusText: string;
  onRestart: MouseEventHandler<HTMLButtonElement>;
  onSave: MouseEventHandler<HTMLButtonElement>;
  onOpenFile: MouseEventHandler<HTMLButtonElement>;
}

export function ControlModal({
  isActive,
  onClose,
  pauseStatusText,
  onRestart,
  onSave,
  onOpenFile,
}: ControlModalProps) {
  if (!isActive) {
    return null;
  }

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-backdrop p-6">
      <div className="flex w-[min(90vw,420px)] flex-col gap-5 rounded-3xl border border-orbit-03/35 bg-panel p-6 text-orbit-03">
        <div className="flex items-center justify-between gap-3">
          <h2 className="m-0 font-orbitron text-[1.35rem] uppercase tracking-[0.08em]">Command Center</h2>
          <button
            type="button"
            onClick={onClose}
            className="rounded-lg bg-panel-soft px-3 py-1 font-trebuchet text-[0.9rem] tracking-[0.06em] text-orbit-03 transition-colors duration-150 hover:bg-orbit-04/20 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-orbit-04"
          >
            Close
          </button>
        </div>
        <p className="m-0 text-left font-trebuchet text-[0.95rem] tracking-[0.04em] text-orbit-03/85">{pauseStatusText}</p>
        <div className="flex flex-col gap-3">
          <button
            type="button"
            onClick={onRestart}
            className="rounded-xl border border-orbit-03/35 bg-panel-soft px-4 py-3 font-trebuchet text-[1rem] tracking-[0.06em] text-orbit-03 transition-colors duration-150 hover:bg-orbit-04/20 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-orbit-04"
          >
            Restart Game
          </button>
          <button
            type="button"
            onClick={onSave}
            className="rounded-xl border border-orbit-03/35 bg-panel-soft px-4 py-3 font-trebuchet text-[1rem] tracking-[0.06em] text-orbit-03 transition-colors duration-150 hover:bg-orbit-04/20 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-orbit-04"
          >
            Save Game
          </button>
          <button
            type="button"
            onClick={onOpenFile}
            className="rounded-xl border border-orbit-03/35 bg-panel-soft px-4 py-3 font-trebuchet text-[1rem] tracking-[0.06em] text-orbit-03 transition-colors duration-150 hover:bg-orbit-04/20 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-orbit-04"
          >
            Load Game
          </button>
        </div>
      </div>
    </div>
  );
}
