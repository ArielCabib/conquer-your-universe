import type { MouseEventHandler } from "react";
import type { InfoEntry } from "../types";

interface ControlModalProps {
  isActive: boolean;
  onClose: () => void;
  onRestart: MouseEventHandler<HTMLButtonElement>;
  onSave: MouseEventHandler<HTMLButtonElement>;
  onOpenFile: MouseEventHandler<HTMLButtonElement>;
}

export function ControlModal({
  isActive,
  onClose,
  onRestart,
  onSave,
  onOpenFile,
}: ControlModalProps) {
  if (!isActive) {
    return null;
  }

  const handleBackdropClick: MouseEventHandler<HTMLDivElement> = (event) => {
    if (event.target === event.currentTarget) {
      onClose();
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-backdrop p-6" onClick={handleBackdropClick}>
      <div className="flex w-[min(90vw,420px)] flex-col gap-5 rounded-3xl border border-orbit-03/35 bg-panel p-6 text-orbit-03">
        <div className="flex items-center justify-between gap-3">
          <h2 className="m-0 font-orbitron text-[1.35rem] uppercase tracking-[0.08em]">Command Center</h2>
          <button
            type="button"
            onClick={onClose}
            className="cursor-pointer rounded-lg bg-panel-soft px-3 py-1 font-trebuchet text-[0.9rem] tracking-[0.06em] text-orbit-03 transition-colors duration-150 hover:bg-orbit-04/20 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-orbit-04"
          >
            Close
          </button>
        </div>
        <p className="m-0 text-left font-trebuchet text-[0.95rem] tracking-[0.04em] text-orbit-03/85">
          <a
            href="https://www.patreon.com/cw/ArielCabib"
            target="_blank"
            rel="noopener noreferrer"
            className="text-orbit-03 underline"
          >
            Support ArielCabib on Patreon
          </a>
        </p>
        <div className="flex flex-col gap-3">
          <button
            type="button"
            onClick={onRestart}
            className="cursor-pointer rounded-xl border border-orbit-03/35 bg-panel-soft px-4 py-3 font-trebuchet text-[1rem] tracking-[0.06em] text-orbit-03 transition-colors duration-150 hover:bg-orbit-04/20 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-orbit-04"
          >
            Restart Game
          </button>
          <button
            type="button"
            onClick={onSave}
            className="cursor-pointer rounded-xl border border-orbit-03/35 bg-panel-soft px-4 py-3 font-trebuchet text-[1rem] tracking-[0.06em] text-orbit-03 transition-colors duration-150 hover:bg-orbit-04/20 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-orbit-04"
          >
            Save Game
          </button>
          <button
            type="button"
            onClick={onOpenFile}
            className="cursor-pointer rounded-xl border border-orbit-03/35 bg-panel-soft px-4 py-3 font-trebuchet text-[1rem] tracking-[0.06em] text-orbit-03 transition-colors duration-150 hover:bg-orbit-04/20 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-orbit-04"
          >
            Load Game
          </button>
        </div>
      </div>
    </div>
  );
}

interface InfoModalProps {
  isActive: boolean;
  onClose: () => void;
  entries: InfoEntry[];
}

export function InfoModal({ isActive, onClose, entries }: InfoModalProps) {
  if (!isActive) {
    return null;
  }

  const hasEntries = entries.length > 0;

  const handleBackdropClick: MouseEventHandler<HTMLDivElement> = (event) => {
    if (event.target === event.currentTarget) {
      onClose();
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-backdrop p-6" onClick={handleBackdropClick}>
      <div className="flex w-[min(90vw,420px)] max-h-[min(85vh,560px)] flex-col gap-5 rounded-3xl border border-orbit-03/35 bg-panel p-6 text-orbit-03">
        <div className="flex items-center justify-between gap-3">
          <h2 className="m-0 font-orbitron text-[1.35rem] uppercase tracking-[0.08em]">Intel Briefing</h2>
          <button
            type="button"
            onClick={onClose}
            className="cursor-pointer rounded-lg bg-panel-soft px-3 py-1 font-trebuchet text-[0.9rem] tracking-[0.06em] text-orbit-03 transition-colors duration-150 hover:bg-orbit-04/20 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-orbit-04"
          >
            Close
          </button>
        </div>
        {hasEntries ? (
          <div className="flex max-h-[360px] flex-col gap-4 overflow-y-auto pr-1">
            {entries.map((entry) => (
              <article
                key={entry.id}
                className="rounded-xl border border-orbit-03/35 bg-panel-soft px-4 py-3 text-left font-trebuchet text-[0.95rem] tracking-[0.04em] text-orbit-03/90"
              >
                <h3 className="m-0 font-orbitron text-[1rem] uppercase tracking-[0.06em] text-orbit-03">{entry.title}</h3>
                <p className="mb-0 mt-2 text-[0.95rem] leading-[1.55] text-orbit-03/85">{entry.description}</p>
              </article>
            ))}
          </div>
        ) : (
          <p className="m-0 text-left font-trebuchet text-[0.95rem] tracking-[0.04em] text-orbit-03/85">
            As new notifications appear, your command intel will be archived here.
          </p>
        )}
      </div>
    </div>
  );
}
