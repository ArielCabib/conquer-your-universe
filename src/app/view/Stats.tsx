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

  return (
    <div className="mt-6 flex w-[min(80vw,540px)] max-w-[600px] flex-wrap items-center justify-center gap-6">
      <div className="rounded-2xl border border-orbit-02 bg-panel-soft px-5 py-3 font-trebuchet text-[clamp(1rem,2vw,1.15rem)] tracking-[0.05em] text-orbit-03">
        Settlers alive: {aliveNow}/{settlersCapacityLimit}
      </div>
      <div className="rounded-2xl border border-orbit-02 bg-panel-soft px-5 py-3 font-trebuchet text-[clamp(1rem,2vw,1.15rem)] tracking-[0.05em] text-orbit-03">
        Houses built: {housesBuilt}/{housesCapacityLimit}
        <span className="mt-1 block text-[0.85rem] tracking-[0.04em] text-orbit-03/85">
          +{houseSpawnAmount} {spawnLabel} per {spawnIntervalSeconds}s
        </span>
      </div>
      <div className="rounded-2xl border border-orbit-02 bg-panel-soft px-5 py-3 font-trebuchet text-[clamp(1rem,2vw,1.15rem)] tracking-[0.05em] text-orbit-03">
        Farms built: {farmsBuilt}/{farmCapacityLimit}
        {farmCapacityLimit > 0 && farmsBuilt >= farmCapacityLimit ? (
          <span className="mt-1 block text-[0.85rem] tracking-[0.04em] text-orbit-03/85">Farm limit reached</span>
        ) : null}
      </div>
      <div className="rounded-2xl border border-orbit-02 bg-panel-soft px-5 py-3 font-trebuchet text-[clamp(1rem,2vw,1.15rem)] tracking-[0.05em] text-orbit-03">
        {`Settler lifespan: ${formatSeconds(settlerMinLifespanMs)}s – ${formatSeconds(settlerMaxLifespanMs)}s`}
        {farmLifespanBonusMs > 0 ? (
          <span className="mt-1 block text-[0.85rem] tracking-[0.04em] text-orbit-03/85">
            +{formatSeconds(farmLifespanBonusMs)}s from farms
          </span>
        ) : null}
      </div>
    </div>
  );
}
