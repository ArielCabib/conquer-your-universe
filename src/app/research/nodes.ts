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
    description: "Streamline housing blueprints to support larger settler populations.",
    dependsOn: ["core-theory"],
  },
  {
    id: "agro-dynamics",
    title: "Agro Dynamics",
    description: "Optimize farm nutrient cycles to accelerate crop production.",
    dependsOn: ["core-theory"],
  },
  {
    id: "quantum-trade",
    title: "Quantum Trade",
    description: "Stabilize markets with predictive commerce models for rapid coin generation.",
    dependsOn: ["habitation-efficiency", "agro-dynamics"],
  },
];

export function getResearchNodeRequirements(nodeId: string): ResearchNodeRequirements | null {
  const node = RESEARCH_NODES.find((definition) => definition.id === nodeId);
  return node?.requirements ?? null;
}
