import { GameState, HouseState, SettlerPhase, SettlerState } from "./types";

interface RawSettlerPhaseAlive {
  Alive: Record<string, never>;
}

interface RawSettlerPhaseFading {
  Fading: {
    started_ms: number;
  };
}

type RawSettlerPhase = RawSettlerPhaseAlive | RawSettlerPhaseFading;

type RawSettlerState = Omit<SettlerState, "anchorX" | "anchorY" | "targetX" | "targetY" | "moveStartMs" | "lastDirectionChangeMs" | "birthMs" | "phase" | "lifespanMs"> & {
  anchor_x: number;
  anchor_y: number;
  target_x: number;
  target_y: number;
  move_start_ms: number;
  last_direction_change_ms: number;
  birth_ms: number;
  lifespan_ms?: number;
  phase: RawSettlerPhase | SettlerPhase;
};

type RawHouseState = Omit<HouseState, "builtMs" | "lastSpawnMs"> & {
  built_ms?: number;
  last_spawn_ms?: number;
};

type RawGameState = Omit<
  GameState,
  | "settlers"
  | "houses"
  | "nextSettlerId"
  | "settlerMinLifespanMs"
  | "settlerMaxLifespanMs"
  | "nextHouseId"
  | "settlersBaseCapacity"
  | "housesBaseCapacity"
  | "settlersPerHouse"
> & {
  settlers: RawSettlerState[];
  houses: RawHouseState[];
  next_settler_id: number;
  settler_min_lifespan_ms: number;
  settler_max_lifespan_ms: number;
  next_house_id: number;
  settlers_base_capacity: number;
  houses_base_capacity: number;
  settlers_per_house: number;
};

function normalizeSettlerPhase(phase: RawSettlerPhase | SettlerPhase): SettlerPhase {
  if ("kind" in phase) {
    return phase;
  }

  if ("Alive" in phase) {
    return { kind: "Alive" };
  }

  if ("Fading" in phase) {
    const started = phase.Fading?.started_ms ?? 0;
    return { kind: "Fading", startedMs: started };
  }

  return { kind: "Alive" };
}

function deserializeSettler(raw: RawSettlerState): SettlerState {
  return {
    id: raw.id,
    anchorX: raw.anchor_x,
    anchorY: raw.anchor_y,
    targetX: raw.target_x,
    targetY: raw.target_y,
    moveStartMs: raw.move_start_ms,
    lastDirectionChangeMs: raw.last_direction_change_ms,
    birthMs: raw.birth_ms,
    phase: normalizeSettlerPhase(raw.phase),
    lifespanMs: raw.lifespan_ms ?? raw.lifespanMs ?? 0,
  };
}

function deserializeHouse(raw: RawHouseState): HouseState {
  return {
    id: raw.id,
    x: raw.x,
    y: raw.y,
    builtMs: raw.built_ms ?? raw.builtMs ?? 0,
    lastSpawnMs: raw.last_spawn_ms ?? raw.lastSpawnMs ?? Number.NEGATIVE_INFINITY,
  };
}

export function deserializeGameState(serialized: string): GameState | null {
  try {
    const data = JSON.parse(serialized) as RawGameState;
    if (!data || typeof data !== "object") {
      return null;
    }

    const settlers = Array.isArray(data.settlers)
      ? data.settlers.map(deserializeSettler)
      : [];
    const houses = Array.isArray(data.houses) ? data.houses.map(deserializeHouse) : [];

    return {
      settlers,
      houses,
      nextSettlerId: data.next_settler_id ?? 0,
      settlerMinLifespanMs: data.settler_min_lifespan_ms ?? 5_000,
      settlerMaxLifespanMs: data.settler_max_lifespan_ms ?? 10_000,
      nextHouseId: data.next_house_id ?? 0,
      settlersBaseCapacity: data.settlers_base_capacity ?? 10,
      housesBaseCapacity: data.houses_base_capacity ?? 5,
      settlersPerHouse: data.settlers_per_house ?? 10,
    };
  } catch (error) {
    console.warn("Failed to deserialize game state", error);
    return null;
  }
}

function serializeSettlerPhase(phase: SettlerPhase): RawSettlerPhase {
  if (phase.kind === "Alive") {
    return { Alive: {} };
  }

  return {
    Fading: {
      started_ms: phase.startedMs,
    },
  };
}

function serializeSettler(settler: SettlerState): RawSettlerState {
  return {
    id: settler.id,
    anchor_x: settler.anchorX,
    anchor_y: settler.anchorY,
    target_x: settler.targetX,
    target_y: settler.targetY,
    move_start_ms: settler.moveStartMs,
    last_direction_change_ms: settler.lastDirectionChangeMs,
    birth_ms: settler.birthMs,
    phase: serializeSettlerPhase(settler.phase),
    lifespan_ms: settler.lifespanMs,
  };
}

function serializeHouse(house: HouseState): RawHouseState {
  return {
    id: house.id,
    x: house.x,
    y: house.y,
    built_ms: house.builtMs,
    last_spawn_ms: house.lastSpawnMs,
  };
}

export function serializeGameState(state: GameState): string {
  const payload: RawGameState = {
    settlers: state.settlers.map(serializeSettler),
    houses: state.houses.map(serializeHouse),
    next_settler_id: state.nextSettlerId,
    settler_min_lifespan_ms: state.settlerMinLifespanMs,
    settler_max_lifespan_ms: state.settlerMaxLifespanMs,
    next_house_id: state.nextHouseId,
    settlers_base_capacity: state.settlersBaseCapacity,
    houses_base_capacity: state.housesBaseCapacity,
    settlers_per_house: state.settlersPerHouse,
  };

  return JSON.stringify(payload);
}
