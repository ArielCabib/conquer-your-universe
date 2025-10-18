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
  onOpenInfo: MouseEventHandler<HTMLButtonElement>;
  onOpenResearch: MouseEventHandler<HTMLButtonElement>;
  planetName: string;
  onPlanetNameChange: (name: string) => void;
  hasResearcher: boolean;
}

export function HeaderSection({
  onOpenSettings,
  onOpenInfo,
  onOpenResearch,
  planetName,
  onPlanetNameChange,
  hasResearcher,
}: HeaderSectionProps) {
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
      <div className="flex w-full justify-end gap-3">
        {hasResearcher ? (
          <button
            type="button"
            aria-label="Open research lab"
            onClick={onOpenResearch}
            className="inline-flex h-9 w-9 items-center justify-center rounded-full border border-orbit-03/40 bg-panel-soft text-orbit-03 transition-colors duration-150 hover:bg-orbit-04/20 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-orbit-04"
          >
            <span aria-hidden="true" className="icon-glyph icon-glyph--research cursor-pointer" />
          </button>
        ) : null}
        <button
          type="button"
          aria-label="Open settings"
          onClick={onOpenSettings}
          className="inline-flex h-9 w-9 items-center justify-center rounded-full border border-orbit-03/40 bg-panel-soft text-orbit-03 transition-colors duration-150 hover:bg-orbit-04/20 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-orbit-04"
        >
          <span aria-hidden="true" className="icon-glyph icon-glyph--settings cursor-pointer" />
        </button>
        <button
          type="button"
          aria-label="Open info"
          onClick={onOpenInfo}
          className="inline-flex h-9 w-9 items-center justify-center rounded-full border border-orbit-03/40 bg-panel-soft text-orbit-03 transition-colors duration-150 hover:bg-orbit-04/20 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-orbit-04"
        >
          <span aria-hidden="true" className="icon-glyph icon-glyph--info cursor-pointer" />
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
          className="m-0 text-center font-orbitron text-[clamp(2.5rem,3vw,3.5rem)] uppercase tracking-[0.12em] text-orbit-0 cursor-pointer"
        >
          {planetName}
        </h1>
      )}
    </div>
  );
}
