export interface ResearchNodeRequirements {
  clickCount?: number;
  coinCost?: number;
}

export interface ResearchNodeDefinition {
  id: string;
  title: string;
  description: string;
  dependsOn: string[];
  requirements?: ResearchNodeRequirements;
}

export const RESEARCH_NODES: ResearchNodeDefinition[] = [
  {
    id: "core-theory",
    title: "Core Hypothesis",
    description: "Establish a unified model for planetary growth to unlock future breakthroughs.",
    dependsOn: [],
    requirements: {
      clickCount: 100,
      coinCost: 100,
    },
  },
  {
    id: "habitation-efficiency",
    title: "Habitation Efficiency",
    description: "+1 settler per 2 seconds.",
    dependsOn: ["core-theory"],
    requirements: {
      clickCount: 150,
      coinCost: 200,
    }
  },
  {
    id: "coin-storage",
    title: "Coin Storage",
    description: "Increase the maximum coin capacity to 400.",
    dependsOn: ["core-theory"],
    requirements: {
      clickCount: 150,
      coinCost: 200,
    },
  }
];

export function getResearchNodeRequirements(nodeId: string): ResearchNodeRequirements | null {
  const node = RESEARCH_NODES.find((definition) => definition.id === nodeId);
  return node?.requirements ?? null;
}
