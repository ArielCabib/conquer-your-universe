interface BuildPromptProps {
  shouldShow: boolean;
  message: string;
}

export function BuildPrompt({ shouldShow, message }: BuildPromptProps) {
  if (!shouldShow) {
    return null;
  }

  return (
    <div className="rounded-xl border border-orbit-03/35 bg-panel-soft px-4 py-2 font-trebuchet text-[0.95rem] uppercase tracking-[0.04em] text-orbit-03">
      {message}
    </div>
  );
}
