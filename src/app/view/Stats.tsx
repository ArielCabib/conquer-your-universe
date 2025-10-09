import type { CSSProperties } from "react";
import { ORBIT_02, ORBIT_03 } from "../../constants";

interface StatsPanelProps {
  aliveNow: number;
  settlersCapacityLimit: number;
  housesBuilt: number;
  housesCapacityLimit: number;
  settlerMinLifespanMs: number;
  settlerMaxLifespanMs: number;
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
}: StatsPanelProps) {
  const containerStyle: CSSProperties = {
    display: "flex",
    flexDirection: "row",
    alignItems: "center",
    gap: "1.5rem",
    justifyContent: "center",
    flexWrap: "wrap",
    maxWidth: "600px",
    width: "min(80vw, 540px)",
  };

  const cardStyle: CSSProperties = {
    marginTop: "1.5rem",
    padding: "0.75rem 1.25rem",
    border: `1px solid ${ORBIT_02}`,
    borderRadius: "0.75rem",
    backgroundColor: "rgba(0,0,0,0.25)",
    color: ORBIT_03,
    fontSize: "clamp(1rem, 2vw, 1.15rem)",
    letterSpacing: "0.05em",
  };

  return (
    <div style={containerStyle}>
      <div style={cardStyle}>Settlers alive: {aliveNow}/{settlersCapacityLimit}</div>
      <div style={cardStyle}>Houses built: {housesBuilt}/{housesCapacityLimit}</div>
      <div style={cardStyle}>
        {`Settler lifespan: ${formatSeconds(settlerMinLifespanMs)}s – ${formatSeconds(settlerMaxLifespanMs)}s`}
      </div>
    </div>
  );
}
