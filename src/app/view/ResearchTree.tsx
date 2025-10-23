import { useEffect, useRef } from "react";

import { RESEARCH_NODES } from "../research/nodes";

interface ResearchTreeProps {
  completedNodeIds: readonly string[];
  onResearchNode: (nodeId: string) => void;
  progressByNodeId: Readonly<Record<string, number>>;
  coinCount: number;
}

type ResearchNodeStatus = "available" | "completed";

export function ResearchTree({
  completedNodeIds,
  onResearchNode,
  progressByNodeId,
  coinCount,
}: ResearchTreeProps) {
  const completed = new Set(completedNodeIds);
  const nodeRefs = useRef<Map<string, HTMLLIElement>>(new Map());

  const visibleNodes = RESEARCH_NODES.filter((node) =>
    node.dependsOn.every((dependency) => completed.has(dependency)),
  ).map((node) => ({
    definition: node,
    status: (completed.has(node.id) ? "completed" : "available") as ResearchNodeStatus,
  }));

  useEffect(() => {
    const observer = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting) {
            const element = entry.target as HTMLElement;
            element.classList.add("research-node-item--visible");
            observer.unobserve(element);
          }
        });
      },
      {
        threshold: 0.2,
      },
    );

    nodeRefs.current.forEach((element) => {
      if (!element.classList.contains("research-node-item--visible")) {
        observer.observe(element);
      }
    });

    return () => observer.disconnect();
  }, [visibleNodes]);

  return (
    <div className="flex h-full w-full flex-col items-center justify-center gap-8  text-orbit-03">
      
      <div className="w-full max-w-[520px] overflow-y-auto">
        <ul className="m-0 flex flex-col gap-6 p-0">
          {visibleNodes.map(({ definition, status }, index) => {
            const isCompleted = status === "completed";
            const statusLabel = isCompleted ? "Completed" : "Available";
            const requirements = definition.requirements;
            const clickTarget = requirements?.clickCount ?? 0;
            const coinCost = requirements?.coinCost ?? 0;
            const recordedProgress = progressByNodeId[definition.id] ?? 0;
            const displayProgress =
              clickTarget > 0 ? Math.min(recordedProgress, clickTarget) : recordedProgress;
            const hasCoins = coinCount >= coinCost;
            const buttonDisabled = isCompleted || !hasCoins;
            const buttonClass = `mt-3 w-full rounded-xl border border-orbit-03/35 px-4 py-2 font-trebuchet text-[0.95rem] tracking-[0.05em] transition-colors duration-150 ${
              buttonDisabled
                ? "cursor-not-allowed bg-panel text-orbit-03/60"
                : "cursor-pointer bg-panel text-orbit-03 hover:bg-orbit-04/20"
            }`;

            const progressLabel =
              clickTarget > 0 ? `Progress: ${displayProgress}/${clickTarget} clicks` : null;
            const coinRequirementLabel =
              coinCost > 0
                ? `Requires ${coinCost} coins${hasCoins ? "" : " (need more)"}`
                : null;

            return (
              <li
                key={definition.id}
                className="research-node-item list-none"
                ref={(element) => {
                  if (element) {
                    nodeRefs.current.set(definition.id, element);
                  } else {
                    nodeRefs.current.delete(definition.id);
                  }
                }}
                style={{ transitionDelay: `${index * 60}ms` }}
              >
                <div className="flex items-start gap-4">
                  <div className="relative flex flex-col items-center">
                    <div
                      className={`h-3 w-3 rounded-full border border-orbit-03/45 ${
                        isCompleted ? "bg-orbit-04" : "bg-panel"
                      }`}
                      aria-hidden="true"
                    />
                    {index < visibleNodes.length - 1 ? (
                      <div className="mt-1 h-[56px] w-px bg-orbit-03/25" aria-hidden="true" />
                    ) : null}
                  </div>
                  <div className="flex-1 rounded-2xl border border-orbit-03/35 bg-panel-soft px-4 py-3 shadow-[0_12px_24px_rgba(var(--orbit-01-rgb)_/_0.35)]">
                    <div className="flex items-center justify-between gap-3">
                      <h3 className="m-0 font-orbitron text-[1rem] uppercase tracking-[0.06em] text-orbit-03">
                        {definition.title}
                      </h3>
                      <span
                        className={`rounded-full border border-orbit-03/25 px-3 py-[2px] font-trebuchet text-[0.75rem] uppercase tracking-[0.08em] ${
                          isCompleted
                            ? "bg-orbit-04/25 text-orbit-04"
                            : "bg-panel text-orbit-03/80"
                        }`}
                      >
                        {statusLabel}
                      </span>
                    </div>
                    <p className="mb-0 mt-2 font-trebuchet text-[0.95rem] leading-[1.55] tracking-[0.04em] text-orbit-03/85">
                      {definition.description}
                    </p>
                    <button
                      type="button"
                      className={buttonClass}
                      onClick={() => onResearchNode(definition.id)}
                      disabled={buttonDisabled}
                    >
                      {isCompleted ? "Researched" : "Research"}
                    </button>
                    {(progressLabel || coinRequirementLabel) && !isCompleted ? (
                      <p className="mb-0 mt-2 text-left font-trebuchet text-[0.75rem] tracking-[0.05em] text-orbit-03/80">
                        {[progressLabel, coinRequirementLabel]
                          .filter((value): value is string => Boolean(value))
                          .join(" | ")}
                      </p>
                    ) : null}
                  </div>
                </div>
              </li>
            );
          })}
        </ul>
      </div>
    </div>
  );
}
