import {
  ChangeEvent,
  FormEvent,
  KeyboardEvent,
  MouseEventHandler,
  useCallback,
  useEffect,
  useRef,
  useState,
} from "react";

interface HeaderSectionProps {
  onOpenSettings: MouseEventHandler<HTMLButtonElement>;
  planetName: string;
  onPlanetNameChange: (name: string) => void;
}

export function HeaderSection({ onOpenSettings, planetName, onPlanetNameChange }: HeaderSectionProps) {
  const [isEditingName, setIsEditingName] = useState(false);
  const [draftName, setDraftName] = useState(planetName);
  const inputRef = useRef<HTMLInputElement | null>(null);

  useEffect(() => {
    if (!isEditingName) {
      setDraftName(planetName);
    }
  }, [isEditingName, planetName]);

  useEffect(() => {
    if (isEditingName) {
      inputRef.current?.focus();
      inputRef.current?.select();
    }
  }, [isEditingName]);

  const startEditing = useCallback(() => {
    setDraftName(planetName);
    setIsEditingName(true);
  }, [planetName]);

  const handleDraftChange = useCallback((event: ChangeEvent<HTMLInputElement>) => {
    setDraftName(event.target.value);
  }, []);

  const finalizeName = useCallback(() => {
    const trimmed = draftName.trim();
    const nextName = trimmed.length > 0 ? trimmed : planetName;
    setIsEditingName(false);
    setDraftName(nextName);
    onPlanetNameChange(nextName);
  }, [draftName, onPlanetNameChange, planetName]);

  const handleSubmit = useCallback(
    (event: FormEvent<HTMLFormElement>) => {
      event.preventDefault();
      finalizeName();
    },
    [finalizeName],
  );

  const handleBlur = useCallback(() => {
    finalizeName();
  }, [finalizeName]);

  const handleHeadingKeyDown = useCallback(
    (event: KeyboardEvent<HTMLHeadingElement>) => {
      if (event.key === "Enter" || event.key === " ") {
        event.preventDefault();
        startEditing();
      }
    },
    [startEditing],
  );

  return (
    <div className="mx-auto flex w-[min(80vw,540px)] max-w-[600px] flex-col items-center gap-4">
      <div className="flex w-full justify-end">
        <button
          type="button"
          aria-label="Open settings"
          onClick={onOpenSettings}
          className="inline-flex h-9 w-9 items-center justify-center rounded-full border border-orbit-03/40 bg-panel-soft text-orbit-03 transition-colors duration-150 hover:bg-orbit-04/20 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-orbit-04"
        >
          <svg width="28" height="28" viewBox="0 0 24 24" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path d="M11.983 2a1 1 0 0 1 .993.883l.007.117v1.19a5.52 5.52 0 0 1 1.45.6l.84-.84a1 1 0 0 1 1.497 1.32l-.083.094-.84.84a5.52 5.52 0 0 1 .6 1.451h1.19a1 1 0 0 1 .117 1.993l-.117.007h-1.19a5.52 5.52 0 0 1-.6 1.45l.84.841a1 1 0 0 1-1.32 1.497l-.094-.083-.84-.84a5.52 5.52 0 0 1-1.451.6v1.19a1 1 0 0 1-1.993.117l-.007-.117v-1.19a5.52 5.52 0 0 1-1.45-.6l-.84.84a1 1 0 0 1-1.497-1.32l.083-.094.84-.84a5.52 5.52 0 0 1-.6-1.451h-1.19a1 1 0 0 1-.117-1.993l.117-.007h1.19a5.52 5.52 0 0 1 .6-1.45l-.84-.841a1 1 0 0 1 1.32-1.497l.094.083.84.84a5.52 5.52 0 0 1 1.451-.6v-1.19A1 1 0 0 1 11.983 2Zm.017 5a3 3 0 1 0 0 6a3 3 0 0 0 0-6Z" />
          </svg>
        </button>
      </div>
      {isEditingName ? (
        <form onSubmit={handleSubmit} className="w-full">
          <input
            ref={inputRef}
            value={draftName}
            onChange={handleDraftChange}
            onBlur={handleBlur}
            aria-label="Planet name"
            className="w-full rounded-lg border border-orbit-03/45 bg-panel-soft px-4 py-2 text-center font-orbitron text-[clamp(2.5rem,3vw,3.5rem)] uppercase tracking-[0.12em] text-orbit-03 shadow-[0_0_0_1px_rgba(255,255,255,0.08)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-orbit-04"
            maxLength={40}
          />
        </form>
      ) : (
        <h1
          role="button"
          tabIndex={0}
          onClick={startEditing}
          onKeyDown={handleHeadingKeyDown}
          className="m-0 text-center font-orbitron text-[clamp(2.5rem,3vw,3.5rem)] uppercase tracking-[0.12em] text-orbit-03"
        >
          {planetName}
        </h1>
      )}
    </div>
  );
}
