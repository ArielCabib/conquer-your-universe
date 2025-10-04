use crate::types::*;
use rand::Rng;
use std::collections::HashMap;

/// Planet generation and management system
pub struct PlanetSystem {
    rng: rand::rngs::ThreadRng,
    planet_counter: u64,
    terraforming_project_counter: u64,
    factory_counter: u64,
}

impl PlanetSystem {
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
            planet_counter: 0,
            terraforming_project_counter: 0,
            factory_counter: 0,
        }
    }

    /// Generate a new planet with random properties
    pub fn generate_planet(&mut self, position: (f64, f64), solar_system_id: u64) -> Planet {
        let planet_id = self.planet_counter;
        self.planet_counter += 1;

        let planet_class = self.generate_planet_class();
        let name = self.generate_planet_name();
        let resources = self.generate_planet_resources(&planet_class);
        let modifiers = self.generate_planet_modifiers(&planet_class);

        Planet {
            id: planet_id,
            name,
            class: planet_class,
            state: PlanetState::Unexplored,
            resources,
            modifiers,
            terraforming_projects: Vec::new(),
            factories: Vec::new(),
            storage: HashMap::new(),
            position,
            solar_system_id,
        }
    }

    /// Generate planet class based on random distribution
    fn generate_planet_class(&mut self) -> PlanetClass {
        let roll = self.rng.gen_range(0..100);
        match roll {
            0..=15 => PlanetClass::Barren,
            16..=25 => PlanetClass::Terran,
            26..=35 => PlanetClass::GasGiant,
            36..=45 => PlanetClass::Ocean,
            46..=55 => PlanetClass::Desert,
            56..=65 => PlanetClass::Ice,
            66..=75 => PlanetClass::Volcanic,
            76..=85 => PlanetClass::Toxic,
            86..=95 => PlanetClass::Crystalline,
            96..=99 => PlanetClass::Metallic,
            _ => PlanetClass::Barren,
        }
    }

    /// Generate planet name
    fn generate_planet_name(&mut self) -> String {
        let prefixes = [
            "Alpha", "Beta", "Gamma", "Delta", "Epsilon", "Zeta", "Eta", "Theta",
        ];
        let suffixes = [
            "Prime", "Secundus", "Tertius", "Quartus", "Quintus", "Major", "Minor", "Nova",
        ];
        let numbers = ["I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X"];

        let prefix = prefixes[self.rng.gen_range(0..prefixes.len())];
        let suffix = if self.rng.gen_bool(0.3) {
            suffixes[self.rng.gen_range(0..suffixes.len())]
        } else {
            numbers[self.rng.gen_range(0..numbers.len())]
        };

        format!("{} {}", prefix, suffix)
    }

    /// Generate base resources for a planet based on its class
    fn generate_planet_resources(
        &mut self,
        planet_class: &PlanetClass,
    ) -> HashMap<ResourceType, u64> {
        let mut resources = HashMap::new();

        match planet_class {
            PlanetClass::Barren => {
                resources.insert(ResourceType::Minerals, self.rng.gen_range(50..200));
                resources.insert(ResourceType::Energy, self.rng.gen_range(10..50));
                resources.insert(ResourceType::Population, 0);
                resources.insert(ResourceType::Technology, self.rng.gen_range(5..30));
                resources.insert(ResourceType::Food, 0);
            }
            PlanetClass::Terran => {
                resources.insert(ResourceType::Population, self.rng.gen_range(100..500));
                resources.insert(ResourceType::Food, self.rng.gen_range(80..300));
                resources.insert(ResourceType::Minerals, self.rng.gen_range(30..150));
                resources.insert(ResourceType::Energy, self.rng.gen_range(20..100));
                resources.insert(ResourceType::Technology, self.rng.gen_range(10..80));
            }
            PlanetClass::GasGiant => {
                resources.insert(ResourceType::Energy, self.rng.gen_range(200..800));
                resources.insert(ResourceType::Minerals, self.rng.gen_range(10..50));
                resources.insert(ResourceType::Population, 0);
                resources.insert(ResourceType::Technology, self.rng.gen_range(5..20));
                resources.insert(ResourceType::Food, 0);
            }
            PlanetClass::Ocean => {
                resources.insert(ResourceType::Food, self.rng.gen_range(150..600));
                resources.insert(ResourceType::Population, self.rng.gen_range(50..300));
                resources.insert(ResourceType::Minerals, self.rng.gen_range(20..100));
                resources.insert(ResourceType::Energy, self.rng.gen_range(30..120));
                resources.insert(ResourceType::Technology, self.rng.gen_range(5..40));
            }
            PlanetClass::Desert => {
                resources.insert(ResourceType::Energy, self.rng.gen_range(100..400));
                resources.insert(ResourceType::Minerals, self.rng.gen_range(40..200));
                resources.insert(ResourceType::Population, self.rng.gen_range(10..80));
                resources.insert(ResourceType::Technology, self.rng.gen_range(15..60));
                resources.insert(ResourceType::Food, self.rng.gen_range(5..30));
            }
            PlanetClass::Ice => {
                resources.insert(ResourceType::Technology, self.rng.gen_range(50..200));
                resources.insert(ResourceType::Minerals, self.rng.gen_range(30..150));
                resources.insert(ResourceType::Population, self.rng.gen_range(5..50));
                resources.insert(ResourceType::Energy, self.rng.gen_range(10..60));
                resources.insert(ResourceType::Food, self.rng.gen_range(5..40));
            }
            PlanetClass::Volcanic => {
                resources.insert(ResourceType::Energy, self.rng.gen_range(150..600));
                resources.insert(ResourceType::Minerals, self.rng.gen_range(80..300));
                resources.insert(ResourceType::Population, 0);
                resources.insert(ResourceType::Technology, self.rng.gen_range(10..50));
                resources.insert(ResourceType::Food, 0);
            }
            PlanetClass::Toxic => {
                resources.insert(ResourceType::Minerals, self.rng.gen_range(100..400));
                resources.insert(ResourceType::Technology, self.rng.gen_range(20..100));
                resources.insert(ResourceType::Population, 0);
                resources.insert(ResourceType::Energy, self.rng.gen_range(30..150));
                resources.insert(ResourceType::Food, 0);
            }
            PlanetClass::Crystalline => {
                resources.insert(ResourceType::Technology, self.rng.gen_range(100..500));
                resources.insert(ResourceType::Minerals, self.rng.gen_range(200..800));
                resources.insert(ResourceType::Population, self.rng.gen_range(10..100));
                resources.insert(ResourceType::Energy, self.rng.gen_range(50..200));
                resources.insert(ResourceType::Food, self.rng.gen_range(5..50));
            }
            PlanetClass::Metallic => {
                resources.insert(ResourceType::Minerals, self.rng.gen_range(300..1000));
                resources.insert(ResourceType::Technology, self.rng.gen_range(50..250));
                resources.insert(ResourceType::Energy, self.rng.gen_range(40..180));
                resources.insert(ResourceType::Population, self.rng.gen_range(20..150));
                resources.insert(ResourceType::Food, self.rng.gen_range(10..80));
            }
        }

        resources
    }

    /// Generate modifiers for a planet based on its class
    fn generate_planet_modifiers(&mut self, planet_class: &PlanetClass) -> Vec<Modifier> {
        let mut modifiers = Vec::new();

        match planet_class {
            PlanetClass::Barren => {
                modifiers.push(Modifier {
                    modifier_type: ModifierType::PopulationPenalty,
                    value: -80.0,
                    is_percentage: true,
                });
                modifiers.push(Modifier {
                    modifier_type: ModifierType::MineralMultiplier,
                    value: 50.0,
                    is_percentage: true,
                });
            }
            PlanetClass::Terran => {
                // Terran planets have balanced modifiers
                if self.rng.gen_bool(0.3) {
                    modifiers.push(Modifier {
                        modifier_type: ModifierType::PopulationMultiplier,
                        value: 25.0,
                        is_percentage: true,
                    });
                }
            }
            PlanetClass::GasGiant => {
                modifiers.push(Modifier {
                    modifier_type: ModifierType::PopulationPenalty,
                    value: -100.0,
                    is_percentage: true,
                });
                modifiers.push(Modifier {
                    modifier_type: ModifierType::EnergyMultiplier,
                    value: 100.0,
                    is_percentage: true,
                });
            }
            PlanetClass::Ocean => {
                modifiers.push(Modifier {
                    modifier_type: ModifierType::FoodMultiplier,
                    value: 75.0,
                    is_percentage: true,
                });
                if self.rng.gen_bool(0.4) {
                    modifiers.push(Modifier {
                        modifier_type: ModifierType::PopulationMultiplier,
                        value: 30.0,
                        is_percentage: true,
                    });
                }
            }
            PlanetClass::Desert => {
                modifiers.push(Modifier {
                    modifier_type: ModifierType::ResourcePenalty,
                    value: -60.0,
                    is_percentage: true,
                });
                modifiers.push(Modifier {
                    modifier_type: ModifierType::EnergyMultiplier,
                    value: 50.0,
                    is_percentage: true,
                });
            }
            PlanetClass::Ice => {
                modifiers.push(Modifier {
                    modifier_type: ModifierType::TechnologyMultiplier,
                    value: 100.0,
                    is_percentage: true,
                });
                modifiers.push(Modifier {
                    modifier_type: ModifierType::PopulationPenalty,
                    value: -40.0,
                    is_percentage: true,
                });
            }
            PlanetClass::Volcanic => {
                modifiers.push(Modifier {
                    modifier_type: ModifierType::PopulationPenalty,
                    value: -100.0,
                    is_percentage: true,
                });
                modifiers.push(Modifier {
                    modifier_type: ModifierType::EnergyMultiplier,
                    value: 150.0,
                    is_percentage: true,
                });
                modifiers.push(Modifier {
                    modifier_type: ModifierType::MineralMultiplier,
                    value: 80.0,
                    is_percentage: true,
                });
            }
            PlanetClass::Toxic => {
                modifiers.push(Modifier {
                    modifier_type: ModifierType::PopulationPenalty,
                    value: -100.0,
                    is_percentage: true,
                });
                modifiers.push(Modifier {
                    modifier_type: ModifierType::ToxicAtmosphere,
                    value: 1.0,
                    is_percentage: false,
                });
                modifiers.push(Modifier {
                    modifier_type: ModifierType::MineralMultiplier,
                    value: 60.0,
                    is_percentage: true,
                });
            }
            PlanetClass::Crystalline => {
                modifiers.push(Modifier {
                    modifier_type: ModifierType::TechnologyMultiplier,
                    value: 200.0,
                    is_percentage: true,
                });
                modifiers.push(Modifier {
                    modifier_type: ModifierType::MineralMultiplier,
                    value: 150.0,
                    is_percentage: true,
                });
            }
            PlanetClass::Metallic => {
                modifiers.push(Modifier {
                    modifier_type: ModifierType::MineralMultiplier,
                    value: 300.0,
                    is_percentage: true,
                });
                modifiers.push(Modifier {
                    modifier_type: ModifierType::TechnologyMultiplier,
                    value: 80.0,
                    is_percentage: true,
                });
            }
        }

        // Add random additional modifiers
        if self.rng.gen_bool(0.2) {
            modifiers.push(Modifier {
                modifier_type: ModifierType::DefensiveBonus,
                value: self.rng.gen_range(10..50) as f64,
                is_percentage: true,
            });
        }

        if self.rng.gen_bool(0.15) {
            modifiers.push(Modifier {
                modifier_type: ModifierType::ResearchBonus,
                value: self.rng.gen_range(20..80) as f64,
                is_percentage: true,
            });
        }

        modifiers
    }

    /// Start a terraforming project on a planet
    pub fn start_terraforming_project(
        &mut self,
        planet: &mut Planet,
        target_modifier: ModifierType,
        required_resources: HashMap<ResourceType, u64>,
        duration: u64,
        energy_cost: u64,
    ) -> bool {
        // Check if planet has enough resources
        let mut can_afford = true;
        for (resource_type, required_amount) in &required_resources {
            let available = planet.resources.get(resource_type).copied().unwrap_or(0);
            if available < *required_amount {
                can_afford = false;
                break;
            }
        }

        if !can_afford {
            return false;
        }

        // Deduct resources
        for (resource_type, required_amount) in &required_resources {
            if let Some(available) = planet.resources.get_mut(resource_type) {
                *available -= required_amount;
            }
        }

        // Create terraforming project
        let project = TerraformingProject {
            id: self.terraforming_project_counter,
            name: format!("Terraform {:?}", target_modifier),
            target_modifier,
            required_resources,
            progress: 0.0,
            duration,
            energy_cost,
        };

        self.terraforming_project_counter += 1;
        planet.terraforming_projects.push(project);
        planet.state = PlanetState::Terraforming;

        true
    }

    /// Update terraforming projects on a planet
    pub fn update_terraforming_projects(
        &mut self,
        planet: &mut Planet,
        game_speed: GameSpeed,
    ) -> bool {
        let mut any_completed = false;
        let speed_multiplier = game_speed as u64;

        for project in &mut planet.terraforming_projects {
            if project.progress < 1.0 {
                project.progress += (1.0 / project.duration as f64) * speed_multiplier as f64;
                if project.progress >= 1.0 {
                    project.progress = 1.0;
                    any_completed = true;
                }
            }
        }

        // Remove completed projects and apply their effects
        let completed_projects: Vec<ModifierType> = planet
            .terraforming_projects
            .iter()
            .filter(|project| project.progress >= 1.0)
            .map(|project| project.target_modifier)
            .collect();

        for modifier in completed_projects {
            self.apply_terraforming_effect(planet, &modifier);
        }

        planet
            .terraforming_projects
            .retain(|project| project.progress < 1.0);

        // If no more terraforming projects, return planet to conquered state
        if planet.terraforming_projects.is_empty() && planet.state == PlanetState::Terraforming {
            planet.state = PlanetState::Conquered;
        }

        any_completed
    }

    /// Apply the effect of a completed terraforming project
    fn apply_terraforming_effect(&mut self, planet: &mut Planet, target_modifier: &ModifierType) {
        // Remove negative modifiers of the same type
        planet
            .modifiers
            .retain(|modifier| modifier.modifier_type != *target_modifier);

        // Add positive modifier
        let new_modifier = Modifier {
            modifier_type: *target_modifier,
            value: 50.0, // Base terraforming bonus
            is_percentage: true,
        };
        planet.modifiers.push(new_modifier);
    }

    /// Add a factory to a planet
    pub fn add_factory(&mut self, planet: &mut Planet, factory_type: FactoryType) -> u64 {
        let factory_id = self.factory_counter;
        self.factory_counter += 1;

        let factory = Factory {
            id: factory_id,
            factory_type,
            production_queue: Vec::new(),
            efficiency: 1.0,
            is_active: true,
        };

        planet.factories.push(factory);
        factory_id
    }

    /// Get the next planet ID
    pub fn get_next_planet_id(&mut self) -> u64 {
        let id = self.planet_counter;
        self.planet_counter += 1;
        id
    }
}
