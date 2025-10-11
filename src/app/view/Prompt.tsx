import { useEffect, useState } from "react";

interface BuildPromptProps {
  message: string | null;
}

const ENTRY_ANIMATION_DURATION_MS = 420;
const EXIT_ANIMATION_DURATION_MS = 260;

type AnimationState = "hidden" | "enter" | "idle" | "exit";

export function BuildPrompt({ message }: BuildPromptProps) {
  const [renderedMessage, setRenderedMessage] = useState<string | null>(message);
  const [animationState, setAnimationState] = useState<AnimationState>(
    message ? "enter" : "hidden",
  );

  useEffect(() => {
    if (!message) {
      if (renderedMessage) {
        setAnimationState("exit");
      }
      return;
    }

    setRenderedMessage(message);
    setAnimationState("enter");
  }, [message, renderedMessage]);

  useEffect(() => {
    if (animationState !== "enter") {
      return;
    }

    const timeout = window.setTimeout(() => {
      setAnimationState("idle");
    }, ENTRY_ANIMATION_DURATION_MS);

    return () => {
      window.clearTimeout(timeout);
    };
  }, [animationState]);

  useEffect(() => {
    if (animationState !== "exit") {
      return;
    }

    const timeout = window.setTimeout(() => {
      setRenderedMessage(null);
      setAnimationState("hidden");
    }, EXIT_ANIMATION_DURATION_MS);

    return () => {
      window.clearTimeout(timeout);
    };
  }, [animationState]);

  if (!renderedMessage) {
    return null;
  }

  const animationClassName =
    animationState === "enter"
      ? "prompt-badge--enter"
      : animationState === "exit"
        ? "prompt-badge--exit"
        : animationState === "idle"
          ? "prompt-badge--idle"
          : "";

  return (
    <div className="pointer-events-none fixed left-1/2 top-1/3 z-20 flex w-full max-w-md -translate-x-1/2 -translate-y-1/2 justify-center px-4">
      <div
        className={`prompt-badge ${animationClassName} rounded-xl border border-orbit-03/35 bg-panel-soft px-4 py-2 font-trebuchet text-[0.95rem] uppercase tracking-[0.04em] text-orbit-05 shadow-[0_12px_24px_rgba(var(--orbit-01-rgb)_/_0.45)]`}
      >
        {renderedMessage}
      </div>
    </div>
  );
}
