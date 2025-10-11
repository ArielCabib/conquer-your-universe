export type InfoEntryId = "explore" | "build" | "farm" | "harvester" | "market";

export interface InfoEntry {
  id: InfoEntryId;
  title: string;
  description: string;
}

export const PROMPT_INFORMATION: Record<InfoEntryId, InfoEntry> = {
  explore: {
    id: "explore",
    title: "Scout the Surface",
    description: "Clicking the planet produces a settler.",
  },
  build: {
    id: "build",
    title: "Establish Housing",
    description:
      "Having a settler allows you to build a house. Houses expand your population capacity.",
  },
  farm: {
    id: "farm",
    title: "Cultivate Farms",
    description:
      "Having at least ten settlers allows you to build a farm. Farms produce crops that can be processed into grains.",
  },
  harvester: {
    id: "harvester",
    title: "Deploy a Harvester",
    description:
      "Gather at least five crop bundles to assemble a harvester. It automates grain collection to keep supplies flowing.",
  },
  market: {
    id: "market",
    title: "Open the Market",
    description: "Stockpile thirty grains to build a market. Markets convert grains into coins.",
  },
};

export function getAllInfoEntries(): InfoEntry[] {
  return Object.values(PROMPT_INFORMATION).map((entry) => ({ ...entry }));
}

export function resolveInfoEntries(ids: readonly string[] | undefined | null): InfoEntry[] {
  if (!Array.isArray(ids)) {
    return [];
  }

  const seen = new Set<string>();
  const entries: InfoEntry[] = [];

  ids.forEach((id) => {
    if (typeof id !== "string" || seen.has(id)) {
      return;
    }

    const entry = PROMPT_INFORMATION[id as InfoEntryId];
    if (entry) {
      entries.push({ ...entry });
      seen.add(id);
    }
  });

  return entries;
}

export function normalizeInfoEntries(
  rawEntries: unknown,
  fallbackIds?: readonly string[] | null,
): InfoEntry[] {
  if (Array.isArray(rawEntries)) {
    const normalized = rawEntries
      .map((entry) => {
        if (!entry || typeof entry !== "object") {
          return null;
        }

        const candidate = entry as Partial<InfoEntry> & { id?: string };
        const { id, title, description } = candidate;
        if (typeof id !== "string") {
          return null;
        }

        const predefined = PROMPT_INFORMATION[id as InfoEntryId];
        if (predefined) {
          return { ...predefined };
        }

        if (typeof title === "string" && typeof description === "string") {
          return { id: id as InfoEntryId, title, description };
        }

        return null;
      })
      .filter((entry): entry is InfoEntry => Boolean(entry));

    if (normalized.length > 0) {
      const seen = new Set<string>();
      return normalized.filter((entry) => {
        if (seen.has(entry.id)) {
          return false;
        }
        seen.add(entry.id);
        return true;
      });
    }
  }

  if (fallbackIds) {
    return resolveInfoEntries(fallbackIds);
  }

  return [];
}
