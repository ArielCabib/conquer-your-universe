import {
  BASE_COIN_CAPACITY,
  BASE_SETTLER_MAX_LIFESPAN_MS,
  BASE_SETTLER_MIN_LIFESPAN_MS,
  GRAIN_PILE_CAPACITY,
  GAME_STATE_VERSION,
} from "./constants";
import {
  CropProjectileState,
  CropState,
  FarmState,
  GameState,
  GrainPileState,
  GrainProjectileState,
  CoinProjectileState,
  HarvesterState,
  HouseState,
  MarketState,
  ResearcherState,
  SettlerPhase,
  SettlerState,
} from "./types";
import { currentTimeMs } from "./app/helpers";

interface RawSettlerPhaseAlive {
  Alive: Record<string, never>;
}

interface RawSettlerPhaseFading {
  Fading: {
    started_ms: number;
  };
}

type RawSettlerPhase = RawSettlerPhaseAlive | RawSettlerPhaseFading;

type RawSettlerState = Omit<
  SettlerState,
  | "anchorX"
  | "anchorY"
  | "targetX"
  | "targetY"
  | "moveStartMs"
  | "lastDirectionChangeMs"
  | "birthMs"
  | "phase"
  | "lifespanMs"
> & {
  anchor_x: number;
  anchor_y: number;
  target_x: number;
  target_y: number;
  move_start_ms: number;
  last_direction_change_ms: number;
  birth_ms: number;
  lifespan_ms?: number;
  lifespanMs?: number;
  phase: RawSettlerPhase | SettlerPhase;
};

type RawHouseState = Omit<HouseState, "builtMs" | "lastSpawnMs"> & {
  built_ms?: number;
  last_spawn_ms?: number;
  builtMs?: number;
  lastSpawnMs?: number;
};

type RawFarmState = Omit<FarmState, "builtMs" | "lastProducedMs"> & {
  built_ms?: number;
  last_produced_ms?: number;
  builtMs?: number;
  lastProducedMs?: number;
};

type RawCropState = Omit<CropState, "createdMs"> & {
  created_ms?: number;
  createdMs?: number;
};

type RawHarvesterState = Omit<
  HarvesterState,
  "builtMs" | "lastHarvestMs" | "lastSpinUpdateMs" | "spinLevel" | "rotationAngle"
> & {
  built_ms?: number;
  builtMs?: number;
  last_harvest_ms?: number;
  lastHarvestMs?: number;
  last_spin_update_ms?: number;
  lastSpinUpdateMs?: number;
  spin_level?: number;
  spinLevel?: number;
  rotation_angle?: number;
  rotationAngle?: number;
};

type RawGrainPileState = Omit<GrainPileState, "createdMs"> & {
  created_ms?: number;
  createdMs?: number;
};

type RawGrainProjectileState = GrainProjectileState;

type RawCropProjectileState = CropProjectileState;

type RawCoinProjectileState = CoinProjectileState;

type RawMarketState = Omit<MarketState, "builtMs" | "lastSaleMs"> & {
  built_ms?: number;
  builtMs?: number;
  last_sale_ms?: number;
  lastSaleMs?: number;
};

type RawResearcherState = Omit<ResearcherState, "builtMs"> & {
  built_ms?: number;
  builtMs?: number;
};

type RawGameState = Omit<
  GameState,
  | "settlers"
  | "houses"
  | "farms"
  | "crops"
  | "planetName"
  | "nextSettlerId"
  | "settlerMinLifespanMs"
  | "settlerMaxLifespanMs"
  | "nextHouseId"
  | "nextFarmId"
  | "nextCropId"
  | "settlersBaseCapacity"
  | "housesBaseCapacity"
  | "farmsBaseCapacity"
  | "settlersPerHouse"
  | "farmLifespanBonusPerFarmMs"
  | "farmCropCapacity"
  | "farmCropSpawnIntervalMs"
  | "houseSpawnIntervalMs"
  | "houseSpawnAmount"
  | "harvester"
  | "grainPile"
  | "cropProjectiles"
  | "grainProjectiles"
  | "marketGrainProjectiles"
  | "coinProjectiles"
  | "nextCropProjectileId"
  | "nextGrainProjectileId"
  | "nextCoinProjectileId"
  | "market"
  | "researcher"
  | "grainPileCapacity"
  | "coins"
  | "version"
  | "infoEntryIds"
  | "completedResearchNodeIds"
> & {
  settlers: RawSettlerState[];
  houses: RawHouseState[];
  farms: RawFarmState[];
  crops: RawCropState[];
  completed_research_node_ids?: string[];
  completedResearchNodeIds?: string[];
  harvester?: RawHarvesterState | null;
  grain_pile?: RawGrainPileState | null;
  grainPile?: RawGrainPileState | null;
  grain_projectiles?: RawGrainProjectileState[];
  grainProjectiles?: RawGrainProjectileState[];
  market_grain_projectiles?: RawGrainProjectileState[];
  marketGrainProjectiles?: RawGrainProjectileState[];
  crop_projectiles?: RawCropProjectileState[];
  cropProjectiles?: RawCropProjectileState[];
  next_grain_projectile_id?: number;
  nextGrainProjectileId?: number;
  coin_projectiles?: RawCoinProjectileState[];
  coinProjectiles?: RawCoinProjectileState[];
  next_crop_projectile_id?: number;
  nextCropProjectileId?: number;
  next_coin_projectile_id?: number;
  nextCoinProjectileId?: number;
  market?: RawMarketState | null;
  researcher?: RawResearcherState | null;
  planet_name?: string;
  info_entry_ids?: string[];
  next_settler_id: number;
  settler_min_lifespan_ms: number;
  settler_max_lifespan_ms: number;
  next_house_id: number;
  next_farm_id: number;
  next_crop_id: number;
  settlers_base_capacity: number;
  houses_base_capacity: number;
  farms_base_capacity: number;
  settlers_per_house: number;
  farm_lifespan_bonus_per_farm_ms: number;
  farm_crop_capacity: number;
  farm_crop_spawn_interval_ms: number;
  house_spawn_interval_ms: number;
  house_spawn_amount: number;
  grain_pile_capacity?: number;
  grainPileCapacity?: number;
  coin_capacity?: number;
  coins?: number;
  version?: number;
  time_reference_ms?: number;
};

type VersionedRawGameState = RawGameState & { version: number };

type MigrationFn = (state: VersionedRawGameState) => VersionedRawGameState | null;

const migrations: Partial<Record<number, MigrationFn>> = {
  1: (state) => {
    return {
      ...state,
      info_entry_ids: [],
      version: 2,
    };
  },
  2: (state) => {
    return {
      ...state,
      researcher: state.researcher ?? null,
      version: 3,
    };
  },
  3: (state) => {
    return {
      ...state,
      coin_capacity: BASE_COIN_CAPACITY,
      version: 4,
    };
  },
  4: (state) => {
    return {
      ...state,
      completed_research_node_ids: [],
      version: 5,
    };
  },
};

function toVersionedRawGameState(raw: RawGameState): VersionedRawGameState | null {
  const version = raw.version;
  if (typeof version !== "number" || !Number.isInteger(version) || version < 1) {
    return null;
  }

  return { ...raw, version } as VersionedRawGameState;
}

function migrateToCurrentVersion(raw: RawGameState): VersionedRawGameState | null {
  const versioned = toVersionedRawGameState(raw);
  if (!versioned) {
    return null;
  }

  const visitedVersions = new Set<number>();
  let current: VersionedRawGameState = { ...versioned };

  while (current.version !== GAME_STATE_VERSION) {
    console.log("Migrating game state from version", current.version);
    if (visitedVersions.has(current.version)) {
      return null;
    }

    visitedVersions.add(current.version);
    const migrate = migrations[current.version];
    if (!migrate) {
      return null;
    }

    const next = migrate(current);
    if (!next) {
      return null;
    }

    current = { ...next };
  }

  return { ...current, version: GAME_STATE_VERSION };
}

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

function deserializeFarm(raw: RawFarmState): FarmState {
  const builtMs = raw.built_ms ?? raw.builtMs ?? 0;
  const lastProducedMs = raw.last_produced_ms ?? raw.lastProducedMs ?? builtMs;
  return {
    id: raw.id,
    x: raw.x,
    y: raw.y,
    builtMs,
    lastProducedMs,
  };
}

function deserializeCrop(raw: RawCropState): CropState {
  return {
    id: raw.id,
    farmId: raw.farmId,
    x: raw.x,
    y: raw.y,
    createdMs: raw.created_ms ?? raw.createdMs ?? 0,
  };
}

function deserializeHarvester(raw: RawHarvesterState | null | undefined): HarvesterState | null {
  if (!raw) {
    return null;
  }

  return {
    x: raw.x,
    y: raw.y,
    builtMs: raw.built_ms ?? raw.builtMs ?? 0,
    lastHarvestMs: raw.last_harvest_ms ?? raw.lastHarvestMs ?? raw.built_ms ?? raw.builtMs ?? 0,
    lastSpinUpdateMs:
      raw.last_spin_update_ms ??
      raw.lastSpinUpdateMs ??
      raw.last_harvest_ms ??
      raw.lastHarvestMs ??
      raw.built_ms ??
      raw.builtMs ??
      0,
    spinLevel: raw.spin_level ?? raw.spinLevel ?? 0,
    rotationAngle: raw.rotation_angle ?? raw.rotationAngle ?? 0,
  };
}

function deserializeGrainPile(raw: RawGrainPileState | null | undefined): GrainPileState | null {
  if (!raw) {
    return null;
  }

  return {
    x: raw.x,
    y: raw.y,
    grains: raw.grains ?? 0,
    createdMs: raw.created_ms ?? raw.createdMs ?? 0,
  };
}

function deserializeGrainProjectiles(
  rawProjectiles: RawGrainProjectileState[] | null | undefined,
): GrainProjectileState[] {
  if (!Array.isArray(rawProjectiles)) {
    return [];
  }

  return rawProjectiles.map((projectile) => ({ ...projectile }));
}

function deserializeCropProjectiles(
  rawProjectiles: RawCropProjectileState[] | null | undefined,
): CropProjectileState[] {
  if (!Array.isArray(rawProjectiles)) {
    return [];
  }

  return rawProjectiles.map((projectile) => ({ ...projectile }));
}

function deserializeCoinProjectiles(
  rawProjectiles: RawCoinProjectileState[] | null | undefined,
): CoinProjectileState[] {
  if (!Array.isArray(rawProjectiles)) {
    return [];
  }

  return rawProjectiles.map((projectile) => ({ ...projectile }));
}

function deserializeMarket(raw: RawMarketState | null | undefined): MarketState | null {
  if (!raw) {
    return null;
  }

  return {
    x: raw.x,
    y: raw.y,
    builtMs: raw.built_ms ?? raw.builtMs ?? 0,
    lastSaleMs:
      raw.last_sale_ms ?? raw.lastSaleMs ?? raw.built_ms ?? raw.builtMs ?? 0,
  };
}

function deserializeResearcher(
  raw: RawResearcherState | null | undefined,
): ResearcherState | null {
  if (!raw) {
    return null;
  }

  return {
    x: raw.x,
    y: raw.y,
    builtMs: raw.built_ms ?? raw.builtMs ?? 0,
  };
}

function isFiniteNumber(value: unknown): value is number {
  return typeof value === "number" && Number.isFinite(value);
}

function getLatestTimestamp(state: GameState, reference?: number | null): number | null {
  let latest: number | null = null;
  const consider = (value: number | null | undefined) => {
    if (isFiniteNumber(value)) {
      if (latest === null || value > latest) {
        latest = value;
      }
    }
  };

  consider(reference);

  state.settlers.forEach((settler) => {
    consider(settler.moveStartMs);
    consider(settler.lastDirectionChangeMs);
    consider(settler.birthMs);
    if (settler.phase.kind === "Fading") {
      consider(settler.phase.startedMs);
    }
  });

  state.houses.forEach((house) => {
    consider(house.builtMs);
    consider(house.lastSpawnMs);
  });

  state.farms.forEach((farm) => {
    consider(farm.builtMs);
    consider(farm.lastProducedMs);
  });

  state.crops.forEach((crop) => {
    consider(crop.createdMs);
  });

  if (state.harvester) {
    consider(state.harvester.builtMs);
    consider(state.harvester.lastHarvestMs);
    consider(state.harvester.lastSpinUpdateMs);
  }

  if (state.grainPile) {
    consider(state.grainPile.createdMs);
  }

  state.cropProjectiles.forEach((projectile) => {
    consider(projectile.launchedMs);
  });

  state.grainProjectiles.forEach((projectile) => {
    consider(projectile.launchedMs);
  });

  state.marketGrainProjectiles.forEach((projectile) => {
    consider(projectile.launchedMs);
  });

  state.coinProjectiles.forEach((projectile) => {
    consider(projectile.launchedMs);
  });

  if (state.market) {
    consider(state.market.builtMs);
    consider(state.market.lastSaleMs);
  }

  if (state.researcher) {
    consider(state.researcher.builtMs);
  }

  return latest;
}

function shiftIfFinite(value: number, delta: number): number {
  return Number.isFinite(value) ? value + delta : value;
}

function shiftGameStateTimestamps(state: GameState, delta: number): void {
  if (delta === 0) {
    return;
  }

  state.settlers.forEach((settler) => {
    settler.moveStartMs += delta;
    settler.lastDirectionChangeMs += delta;
    settler.birthMs += delta;
    if (settler.phase.kind === "Fading") {
      settler.phase.startedMs += delta;
    }
  });

  state.houses.forEach((house) => {
    house.builtMs = shiftIfFinite(house.builtMs, delta);
    house.lastSpawnMs = shiftIfFinite(house.lastSpawnMs, delta);
  });

  state.farms.forEach((farm) => {
    farm.builtMs = shiftIfFinite(farm.builtMs, delta);
    farm.lastProducedMs = shiftIfFinite(farm.lastProducedMs, delta);
  });

  state.crops.forEach((crop) => {
    crop.createdMs = shiftIfFinite(crop.createdMs, delta);
  });

  if (state.harvester) {
    state.harvester.builtMs = shiftIfFinite(state.harvester.builtMs, delta);
    state.harvester.lastHarvestMs = shiftIfFinite(state.harvester.lastHarvestMs, delta);
    state.harvester.lastSpinUpdateMs = shiftIfFinite(state.harvester.lastSpinUpdateMs, delta);
  }

  if (state.grainPile) {
    state.grainPile.createdMs = shiftIfFinite(state.grainPile.createdMs, delta);
  }

  state.cropProjectiles.forEach((projectile) => {
    projectile.launchedMs = shiftIfFinite(projectile.launchedMs, delta);
  });

  state.grainProjectiles.forEach((projectile) => {
    projectile.launchedMs = shiftIfFinite(projectile.launchedMs, delta);
  });

  state.marketGrainProjectiles.forEach((projectile) => {
    projectile.launchedMs = shiftIfFinite(projectile.launchedMs, delta);
  });

  state.coinProjectiles.forEach((projectile) => {
    projectile.launchedMs = shiftIfFinite(projectile.launchedMs, delta);
  });

  if (state.market) {
    state.market.builtMs = shiftIfFinite(state.market.builtMs, delta);
    state.market.lastSaleMs = shiftIfFinite(state.market.lastSaleMs, delta);
  }

  if (state.researcher) {
    state.researcher.builtMs = shiftIfFinite(state.researcher.builtMs, delta);
  }
}

export function deserializeGameState(serialized: string): GameState | null {
  try {
    const raw = JSON.parse(serialized) as RawGameState;
    if (!raw || typeof raw !== "object") {
      return null;
    }

    const data = migrateToCurrentVersion(raw);
    if (!data) {
      console.warn("Unsupported game state version");
      return null;
    }

    const settlers = Array.isArray(data.settlers)
      ? data.settlers.map(deserializeSettler)
      : [];
    const houses = Array.isArray(data.houses) ? data.houses.map(deserializeHouse) : [];
    const farms = Array.isArray(data.farms) ? data.farms.map(deserializeFarm) : [];
    const crops = Array.isArray(data.crops) ? data.crops.map(deserializeCrop) : [];
    const harvester = deserializeHarvester(data.harvester);
    const grainPile = deserializeGrainPile(data.grain_pile ?? data.grainPile);
    const grainProjectiles = deserializeGrainProjectiles(
      data.grain_projectiles ?? data.grainProjectiles,
    );
    const marketGrainProjectiles = deserializeGrainProjectiles(
      data.market_grain_projectiles ?? data.marketGrainProjectiles,
    );
    const cropProjectiles = deserializeCropProjectiles(
      data.crop_projectiles ?? data.cropProjectiles,
    );
    const nextGrainProjectileId = data.next_grain_projectile_id ?? data.nextGrainProjectileId ?? 0;
    const nextCropProjectileId = data.next_crop_projectile_id ?? data.nextCropProjectileId ?? 0;
    const coinProjectiles = deserializeCoinProjectiles(data.coin_projectiles ?? data.coinProjectiles);
    const nextCoinProjectileId = data.next_coin_projectile_id ?? data.nextCoinProjectileId ?? 0;
    const market = deserializeMarket(data.market);
    const researcher = deserializeResearcher(data.researcher);
    const infoEntryIds = Array.isArray(data.info_entry_ids)
      ? data.info_entry_ids.filter((value): value is string => typeof value === "string")
      : [];
    const completedResearchSource =
      data.completed_research_node_ids ?? data.completedResearchNodeIds;
    const completedResearchNodeIds = Array.isArray(completedResearchSource)
      ? completedResearchSource.filter((value): value is string => typeof value === "string")
      : [];

    const rawCoinCapacity =
      data.coin_capacity ?? (data as { coinCapacity?: number }).coinCapacity;
    const coinCapacity =
      typeof rawCoinCapacity === "number" && Number.isFinite(rawCoinCapacity)
        ? Math.max(0, rawCoinCapacity)
        : BASE_COIN_CAPACITY;

    const state: GameState = {
      version: GAME_STATE_VERSION,
      settlers,
      houses,
      farms,
      planetName: data.planet_name ?? "Your Planet",
      nextSettlerId: data.next_settler_id ?? 0,
      settlerMinLifespanMs: data.settler_min_lifespan_ms ?? BASE_SETTLER_MIN_LIFESPAN_MS,
      settlerMaxLifespanMs: data.settler_max_lifespan_ms ?? BASE_SETTLER_MAX_LIFESPAN_MS,
      nextHouseId: data.next_house_id ?? 0,
      nextFarmId: data.next_farm_id ?? 0,
      crops,
      nextCropId: data.next_crop_id ?? 0,
      harvester,
      grainPile,
      cropProjectiles,
      grainProjectiles,
      marketGrainProjectiles,
      coinProjectiles,
      infoEntryIds,
      completedResearchNodeIds,
      nextCropProjectileId,
      nextGrainProjectileId,
      nextCoinProjectileId,
      market,
      researcher,
      coinCapacity,
      coins:
        typeof data.coins === "number" && Number.isFinite(data.coins)
          ? Math.max(0, data.coins)
          : 0,
      settlersBaseCapacity: data.settlers_base_capacity ?? 10,
      housesBaseCapacity: data.houses_base_capacity ?? 5,
      farmsBaseCapacity: data.farms_base_capacity ?? 5,
      settlersPerHouse: data.settlers_per_house ?? 10,
      farmLifespanBonusPerFarmMs: data.farm_lifespan_bonus_per_farm_ms ?? 1_000,
      farmCropCapacity: data.farm_crop_capacity ?? 5,
      farmCropSpawnIntervalMs: data.farm_crop_spawn_interval_ms ?? 4_500,
      houseSpawnIntervalMs: data.house_spawn_interval_ms ?? 5_000,
      houseSpawnAmount: data.house_spawn_amount ?? 1,
      grainPileCapacity: data.grain_pile_capacity ?? data.grainPileCapacity ?? GRAIN_PILE_CAPACITY,
    };

    const referenceTimestamp = data.time_reference_ms;
    const baseline = getLatestTimestamp(state, referenceTimestamp);

    if (baseline !== null) {
      const delta = currentTimeMs() - baseline;
      if (Math.abs(delta) > 0.001) {
        shiftGameStateTimestamps(state, delta);
      }
    }

    return state;
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

function serializeFarm(farm: FarmState): RawFarmState {
  return {
    id: farm.id,
    x: farm.x,
    y: farm.y,
    built_ms: farm.builtMs,
    last_produced_ms: farm.lastProducedMs,
  };
}

function serializeCrop(crop: CropState): RawCropState {
  return {
    id: crop.id,
    farmId: crop.farmId,
    x: crop.x,
    y: crop.y,
    created_ms: crop.createdMs,
  };
}

function serializeHarvester(harvester: HarvesterState | null): RawHarvesterState | null {
  if (!harvester) {
    return null;
  }

  return {
    x: harvester.x,
    y: harvester.y,
    built_ms: harvester.builtMs,
    last_harvest_ms: harvester.lastHarvestMs,
    last_spin_update_ms: harvester.lastSpinUpdateMs,
    spin_level: harvester.spinLevel,
    rotation_angle: harvester.rotationAngle,
  };
}

function serializeGrainPile(pile: GrainPileState | null): RawGrainPileState | null {
  if (!pile) {
    return null;
  }

  return {
    x: pile.x,
    y: pile.y,
    grains: pile.grains,
    created_ms: pile.createdMs,
  };
}

function serializeGrainProjectiles(
  projectiles: GrainProjectileState[],
): RawGrainProjectileState[] {
  return projectiles.map((projectile) => ({ ...projectile }));
}

function serializeCropProjectiles(
  projectiles: CropProjectileState[],
): RawCropProjectileState[] {
  return projectiles.map((projectile) => ({ ...projectile }));
}

function serializeCoinProjectiles(
  projectiles: CoinProjectileState[],
): RawCoinProjectileState[] {
  return projectiles.map((projectile) => ({ ...projectile }));
}

function serializeMarket(market: MarketState | null): RawMarketState | null {
  if (!market) {
    return null;
  }

  return {
    x: market.x,
    y: market.y,
    built_ms: market.builtMs,
    last_sale_ms: market.lastSaleMs,
  };
}

function serializeResearcher(researcher: ResearcherState | null): RawResearcherState | null {
  if (!researcher) {
    return null;
  }

  return {
    x: researcher.x,
    y: researcher.y,
    built_ms: researcher.builtMs,
  };
}

export function serializeGameState(state: GameState, timestampMs: number = currentTimeMs()): string {
  const payload: RawGameState = {
    version: state.version,
    settlers: state.settlers.map(serializeSettler),
    houses: state.houses.map(serializeHouse),
    farms: state.farms.map(serializeFarm),
    crops: state.crops.map(serializeCrop),
    harvester: serializeHarvester(state.harvester),
    grain_pile: serializeGrainPile(state.grainPile),
    crop_projectiles: serializeCropProjectiles(state.cropProjectiles),
    grain_projectiles: serializeGrainProjectiles(state.grainProjectiles),
    market_grain_projectiles: serializeGrainProjectiles(state.marketGrainProjectiles),
    coin_projectiles: serializeCoinProjectiles(state.coinProjectiles),
    info_entry_ids: state.infoEntryIds.slice(),
    completed_research_node_ids: state.completedResearchNodeIds.slice(),
    next_crop_projectile_id: state.nextCropProjectileId,
    next_grain_projectile_id: state.nextGrainProjectileId,
    next_coin_projectile_id: state.nextCoinProjectileId,
    market: serializeMarket(state.market),
    researcher: serializeResearcher(state.researcher),
    planet_name: state.planetName,
    next_settler_id: state.nextSettlerId,
    settler_min_lifespan_ms: state.settlerMinLifespanMs,
    settler_max_lifespan_ms: state.settlerMaxLifespanMs,
    next_house_id: state.nextHouseId,
    next_farm_id: state.nextFarmId,
    next_crop_id: state.nextCropId,
    settlers_base_capacity: state.settlersBaseCapacity,
    houses_base_capacity: state.housesBaseCapacity,
    farms_base_capacity: state.farmsBaseCapacity,
    settlers_per_house: state.settlersPerHouse,
    farm_lifespan_bonus_per_farm_ms: state.farmLifespanBonusPerFarmMs,
    farm_crop_capacity: state.farmCropCapacity,
    farm_crop_spawn_interval_ms: state.farmCropSpawnIntervalMs,
    house_spawn_interval_ms: state.houseSpawnIntervalMs,
    house_spawn_amount: state.houseSpawnAmount,
    grain_pile_capacity: state.grainPileCapacity,
    coin_capacity: state.coinCapacity,
    coins: state.coins,
    time_reference_ms: timestampMs,
  };

  return JSON.stringify(payload);
}
