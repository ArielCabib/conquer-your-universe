import type { JSX } from "react";

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
}: StatsPanelProps) {
  const spawnIntervalSeconds = formatSeconds(houseSpawnIntervalMs);
  const spawnLabel = houseSpawnAmount === 1 ? "settler" : "settlers";

  const statCards: JSX.Element[] = [];

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

  if (statCards.length === 0) {
    return null;
  }

  return (
    <div className="mt-4 flex w-[min(80vw,540px)] max-w-[600px] flex-wrap items-center justify-center gap-4">
      {statCards}
    </div>
  );
}
