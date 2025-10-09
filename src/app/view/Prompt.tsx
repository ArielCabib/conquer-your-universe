interface BuildPromptProps {
  shouldShow: boolean;
}

export function BuildPrompt({ shouldShow }: BuildPromptProps) {
  if (!shouldShow) {
    return null;
  }

  return (
    <div className="rounded-xl border border-orbit-03/35 bg-panel-soft px-4 py-2 font-trebuchet text-[0.95rem] uppercase tracking-[0.04em] text-orbit-03">
      Right click the planet to build a house
    </div>
  );
}
