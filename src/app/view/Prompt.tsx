import { useEffect, useState } from "react";

interface BuildPromptProps {
  message: string | null;
}

const EXIT_ANIMATION_DURATION_MS = 200;

export function BuildPrompt({ message }: BuildPromptProps) {
  const [renderedMessage, setRenderedMessage] = useState<string | null>(message);
  const [isVisible, setIsVisible] = useState(false);

  useEffect(() => {
    if (message) {
      setRenderedMessage(message);
      const animationFrame = requestAnimationFrame(() => {
        setIsVisible(true);
      });

      return () => cancelAnimationFrame(animationFrame);
    }

    setIsVisible(false);
  }, [message]);

  useEffect(() => {
    if (!message && !isVisible) {
      const timeout = window.setTimeout(() => {
        setRenderedMessage(null);
      }, EXIT_ANIMATION_DURATION_MS);

      return () => {
        window.clearTimeout(timeout);
      };
    }
  }, [isVisible, message]);

  if (!renderedMessage) {
    return null;
  }

  return (
    <div className="pointer-events-none fixed left-1/2 top-12 z-20 flex w-full max-w-md -translate-x-1/2 justify-center px-4">
      <div
        className={`rounded-xl border border-orbit-03/35 bg-panel-soft px-4 py-2 font-trebuchet text-[0.95rem] uppercase tracking-[0.04em] text-orbit-05 shadow-[0_12px_24px_rgba(var(--orbit-01-rgb)_/_0.45)] transition-all duration-200 ease-out ${isVisible ? "translate-y-0 opacity-100" : "-translate-y-3 opacity-0"}`}
      >
        {renderedMessage}
      </div>
    </div>
  );
}
