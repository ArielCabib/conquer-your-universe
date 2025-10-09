import { useLayoutEffect, useRef, useState, type JSX } from "react";

interface StatsPanelProps {
  aliveNow: number;
  settlersCapacityLimit: number;
  housesBuilt: number;
  housesCapacityLimit: number;
  settlerMinLifespanMs: number;
  settlerMaxLifespanMs: number;
  farmsBuilt: number;
  farmCapacityLimit: number;
  farmLifespanBonusMs: number;
  houseSpawnIntervalMs: number;
  houseSpawnAmount: number;
  grainCount: number;
  grainCapacity: number;
  grainsInFlight: number;
  hasHarvester: boolean;
  hasMarket: boolean;
}

function formatSeconds(ms: number): string {
  const seconds = ms / 1000;
  return Number.isInteger(seconds) ? `${seconds}` : seconds.toFixed(1);
}

export function StatsPanel({
  aliveNow,
  settlersCapacityLimit,
  housesBuilt,
  housesCapacityLimit,
  settlerMinLifespanMs,
  settlerMaxLifespanMs,
  farmsBuilt,
  farmCapacityLimit,
  farmLifespanBonusMs,
  houseSpawnIntervalMs,
  houseSpawnAmount,
  grainCount,
  grainCapacity,
  grainsInFlight,
  hasHarvester,
  hasMarket,
}: StatsPanelProps) {
  const spawnIntervalSeconds = formatSeconds(houseSpawnIntervalMs);
  const spawnLabel = houseSpawnAmount === 1 ? "settler" : "settlers";

  const statCards: JSX.Element[] = [];
  const wrapperRef = useRef<HTMLDivElement>(null);
  const contentRef = useRef<HTMLDivElement>(null);
  const [hasMeasuredHeight, setHasMeasuredHeight] = useState(false);

  useLayoutEffect(() => {
    const wrapper = wrapperRef.current;
    const content = contentRef.current;

    if (!wrapper || !content) {
      return;
    }

    const updateHeight = () => {
      wrapper.style.setProperty(
        "--stats-panel-height",
        `${content.offsetHeight}px`,
      );
      setHasMeasuredHeight(true);
    };

    updateHeight();

    if (typeof ResizeObserver === "undefined") {
      return;
    }

    const resizeObserver = new ResizeObserver(updateHeight);
    resizeObserver.observe(content);

    return () => {
      resizeObserver.disconnect();
    };
  }, []);

  useLayoutEffect(() => {
    if (typeof ResizeObserver !== "undefined") {
      return;
    }

    const wrapper = wrapperRef.current;
    const content = contentRef.current;

    if (!wrapper || !content) {
      return;
    }

    wrapper.style.setProperty("--stats-panel-height", `${content.offsetHeight}px`);
    setHasMeasuredHeight(true);
  }, [
    aliveNow,
    settlersCapacityLimit,
    housesBuilt,
    housesCapacityLimit,
    settlerMinLifespanMs,
    settlerMaxLifespanMs,
    farmsBuilt,
    farmCapacityLimit,
    farmLifespanBonusMs,
    houseSpawnIntervalMs,
    houseSpawnAmount,
    grainCount,
    grainCapacity,
    grainsInFlight,
    hasHarvester,
    hasMarket,
  ]);

  if (aliveNow > 0) {
    statCards.push(
      <div
        key="settlers"
        className="rounded-2xl border border-orbit-02 bg-panel-soft px-4 py-2 font-trebuchet text-[clamp(1rem,2vw,1.15rem)] tracking-[0.05em] text-orbit-03"
      >
        Settlers alive: {aliveNow}/{settlersCapacityLimit}
      </div>,
    );
  }

  if (housesBuilt > 0) {
    statCards.push(
      <div
        key="houses"
        className="rounded-2xl border border-orbit-02 bg-panel-soft px-4 py-2 font-trebuchet text-[clamp(1rem,2vw,1.15rem)] tracking-[0.05em] text-orbit-03"
      >
        Houses built: {housesBuilt}/{housesCapacityLimit}
        <span className="mt-1 block text-[0.85rem] tracking-[0.04em] text-orbit-03/85">
          +{houseSpawnAmount} {spawnLabel} per {spawnIntervalSeconds}s
        </span>
      </div>,
    );
  }

  if (farmsBuilt > 0) {
    statCards.push(
      <div
        key="farms"
        className="rounded-2xl border border-orbit-02 bg-panel-soft px-4 py-2 font-trebuchet text-[clamp(1rem,2vw,1.15rem)] tracking-[0.05em] text-orbit-03"
      >
        Farms built: {farmsBuilt}/{farmCapacityLimit}
        {farmCapacityLimit > 0 && farmsBuilt >= farmCapacityLimit ? (
          <span className="mt-1 block text-[0.85rem] tracking-[0.04em] text-orbit-03/85">Farm limit reached</span>
        ) : null}
      </div>,
    );
  }

  if (farmLifespanBonusMs > 0) {
    statCards.push(
      <div
        key="lifespan"
        className="rounded-2xl border border-orbit-02 bg-panel-soft px-4 py-2 font-trebuchet text-[clamp(1rem,2vw,1.15rem)] tracking-[0.05em] text-orbit-03"
      >
        {`Settler lifespan: ${formatSeconds(settlerMinLifespanMs)}s – ${formatSeconds(settlerMaxLifespanMs)}s`}
        <span className="mt-1 block text-[0.85rem] tracking-[0.04em] text-orbit-03/85">
          +{formatSeconds(farmLifespanBonusMs)}s from farms
        </span>
      </div>,
    );
  }

  if (hasHarvester || grainCount > 0 || grainsInFlight > 0) {
    statCards.push(
      <div
        key="grain"
        className="rounded-2xl border border-orbit-02 bg-panel-soft px-4 py-2 font-trebuchet text-[clamp(1rem,2vw,1.15rem)] tracking-[0.05em] text-orbit-03"
      >
        Grain pile: {grainCount}/{grainCapacity}
        {grainsInFlight > 0 ? (
          <span className="mt-1 block text-[0.85rem] tracking-[0.04em] text-orbit-03/85">
            +{grainsInFlight} in flight
          </span>
        ) : null}
      </div>,
    );
  }

  if (hasMarket) {
    statCards.push(
      <div
        key="market"
        className="rounded-2xl border border-orbit-02 bg-panel-soft px-4 py-2 font-trebuchet text-[clamp(1rem,2vw,1.15rem)] tracking-[0.05em] text-orbit-03"
      >
        Market established
      </div>,
    );
  }

  return (
    <div
      ref={wrapperRef}
      className={`stats-panel ${hasMeasuredHeight ? "opacity-100" : "opacity-0"}`}
    >
      <div ref={contentRef} className="flex flex-wrap items-center justify-center gap-4">
        {statCards}
      </div>
    </div>
  );
}
