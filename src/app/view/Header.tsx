import { MouseEventHandler } from "react";

interface HeaderSectionProps {
  onOpenSettings: MouseEventHandler<HTMLButtonElement>;
}

export function HeaderSection({ onOpenSettings }: HeaderSectionProps) {
  return (
    <div className="flex w-[min(80vw,540px)] max-w-[600px] items-center justify-between gap-4">
      <div>
        <div className="flex items-center">
          <span />
          <button
            type="button"
            aria-label="Open settings"
            onClick={onOpenSettings}
            className="ml-auto inline-flex h-9 w-9 items-center justify-center rounded-full border border-orbit-03/40 bg-panel-soft text-orbit-03 transition-colors duration-150 hover:bg-orbit-04/20 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-orbit-04"
          >
            <svg width="28" height="28" viewBox="0 0 24 24" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
              <path d="M11.983 2a1 1 0 0 1 .993.883l.007.117v1.19a5.52 5.52 0 0 1 1.45.6l.84-.84a1 1 0 0 1 1.497 1.32l-.083.094-.84.84a5.52 5.52 0 0 1 .6 1.451h1.19a1 1 0 0 1 .117 1.993l-.117.007h-1.19a5.52 5.52 0 0 1-.6 1.45l.84.841a1 1 0 0 1-1.32 1.497l-.094-.083-.84-.84a5.52 5.52 0 0 1-1.451.6v1.19a1 1 0 0 1-1.993.117l-.007-.117v-1.19a5.52 5.52 0 0 1-1.45-.6l-.84.84a1 1 0 0 1-1.497-1.32l.083-.094.84-.84a5.52 5.52 0 0 1-.6-1.451h-1.19a1 1 0 0 1-.117-1.993l.117-.007h1.19a5.52 5.52 0 0 1 .6-1.45l-.84-.841a1 1 0 0 1 1.32-1.497l.094.083.84.84a5.52 5.52 0 0 1 1.451-.6v-1.19A1 1 0 0 1 11.983 2Zm.017 5a3 3 0 1 0 0 6a3 3 0 0 0 0-6Z" />
            </svg>
          </button>
        </div>
        <h1
          className="m-0 font-orbitron text-[clamp(2.5rem,3vw,3.5rem)] uppercase tracking-[0.12em] text-orbit-03"
        >
          Your Planet
        </h1>
        <h3 className="mt-1 font-trebuchet text-[clamp(1rem,1vw,1rem)] tracking-[0.05em] text-orbit-05">
          Click around and find out.
        </h3>
      </div>
    </div>
  );
}
