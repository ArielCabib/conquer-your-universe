use crate::types::*;
use rand::Rng;
use std::collections::HashMap;

/// Galaxy and solar system management
pub struct GalaxySystem {
    rng: rand::rngs::ThreadRng,
    galaxy_counter: u64,
    system_counter: u64,
    planet_system: crate::planet_system::PlanetSystem,
}

impl GalaxySystem {
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
            galaxy_counter: 0,
            system_counter: 0,
            planet_system: crate::planet_system::PlanetSystem::new(),
        }
    }

    /// Generate a new galaxy with solar systems
    pub fn generate_galaxy(&mut self, name: String, size: u32) -> Galaxy {
        let galaxy_id = self.galaxy_counter;
        self.galaxy_counter += 1;

        let mut solar_systems = Vec::new();
        let galaxy_modifiers = self.generate_galaxy_modifiers();

        // Generate solar systems for the galaxy
        for i in 0..size {
            let system = self.generate_solar_system(galaxy_id, i);
            solar_systems.push(system.id);
        }

        Galaxy {
            id: galaxy_id,
            name,
            solar_systems,
            galaxy_modifiers,
            is_conquered: false,
        }
    }

    /// Generate a solar system with planets
    pub fn generate_solar_system(&mut self, _galaxy_id: u64, system_index: u32) -> SolarSystem {
        let system_id = self.system_counter;
        self.system_counter += 1;

        let name = self.generate_system_name(system_index);
        let position = self.generate_system_position(_galaxy_id, system_index);
        let system_modifiers = self.generate_system_modifiers();

        // Generate planets for this system
        let planet_count = self.rng.gen_range(3..=8);
        let mut planets = Vec::new();

        let guaranteed_terran_index = if system_index == 0 {
            Some(self.rng.gen_range(0..planet_count))
        } else {
            None
        };

        for i in 0..planet_count {
            let planet_position =
                self.generate_planet_position_in_system(position, i, planet_count);
            let planet = if Some(i) == guaranteed_terran_index {
                self
                    .planet_system
                    .generate_planet_with_class(PlanetClass::Terran, planet_position, system_id)
            } else {
                self
                    .planet_system
                    .generate_planet(planet_position, system_id)
            };
            planets.push(planet.id);
        }

        SolarSystem {
            id: system_id,
            name,
            planets,
            system_modifiers,
            position,
            is_conquered: false,
        }
    }

    /// Generate a solar system and return both the system and its planets
    pub fn generate_solar_system_with_planets(
        &mut self,
        _galaxy_id: u64,
        system_index: u32,
    ) -> (SolarSystem, Vec<Planet>) {
        let system_id = self.system_counter;
        self.system_counter += 1;

        let name = self.generate_system_name(system_index);
        let position = self.generate_system_position(_galaxy_id, system_index);
        let system_modifiers = self.generate_system_modifiers();

        // Generate planets for this system
        let planet_count = self.rng.gen_range(3..=8);
        let mut planets = Vec::new();
        let mut planet_objects = Vec::new();

        let guaranteed_terran_index = if system_index == 0 {
            Some(self.rng.gen_range(0..planet_count))
        } else {
            None
        };

        for i in 0..planet_count {
            let planet_position =
                self.generate_planet_position_in_system(position, i, planet_count);
            let planet = if Some(i) == guaranteed_terran_index {
                self
                    .planet_system
                    .generate_planet_with_class(PlanetClass::Terran, planet_position, system_id)
            } else {
                self
                    .planet_system
                    .generate_planet(planet_position, system_id)
            };
            planets.push(planet.id);
            planet_objects.push(planet);
        }

        let system = SolarSystem {
            id: system_id,
            name,
            planets,
            system_modifiers,
            position,
            is_conquered: false,
        };

        (system, planet_objects)
    }

    /// Generate system name
    fn generate_system_name(&mut self, index: u32) -> String {
        let prefixes = [
            "Alpha", "Beta", "Gamma", "Delta", "Epsilon", "Zeta", "Eta", "Theta",
        ];
        let suffixes = [
            "Sector", "Quadrant", "Region", "Zone", "Cluster", "Nebula", "Star", "System",
        ];

        let prefix = prefixes[index as usize % prefixes.len()];
        let suffix = suffixes[self.rng.gen_range(0..suffixes.len())];
        let number = index + 1;

        format!("{} {} {}", prefix, suffix, number)
    }

    /// Generate system position within galaxy (grid coordinates)
    fn generate_system_position(&mut self, _galaxy_id: u64, system_index: u32) -> (f64, f64) {
        // Generate positions as integer grid coordinates
        let grid_size = 10; // 10x10 grid
        let x = (system_index % grid_size) as f64;
        let y = (system_index / grid_size) as f64;

        (x, y)
    }

    /// Generate planet position within a solar system
    pub fn generate_planet_position_in_system(
        &mut self,
        system_position: (f64, f64),
        planet_index: u32,
        total_planets: u32,
    ) -> (f64, f64) {
        // Generate planets in orbital rings around the system center
        let ring_radius = 30.0 + (planet_index as f64 * 15.0);
        let angle = (planet_index as f64) * 2.0 * std::f64::consts::PI / (total_planets as f64);

        let x = system_position.0 + angle.cos() * ring_radius;
        let y = system_position.1 + angle.sin() * ring_radius;

        (x, y)
    }

    /// Generate galaxy-level modifiers
    fn generate_galaxy_modifiers(&mut self) -> Vec<Modifier> {
        let mut modifiers = Vec::new();

        // Galaxy type determines base modifiers
        let galaxy_type = self.rng.gen_range(0..5);
        match galaxy_type {
            0 => {
                // Spiral Galaxy
                modifiers.push(Modifier {
                    modifier_type: ModifierType::ResearchBonus,
                    value: 25.0,
                    is_percentage: true,
                });
            }
            1 => {
                // Elliptical Galaxy
                modifiers.push(Modifier {
                    modifier_type: ModifierType::MineralMultiplier,
                    value: 50.0,
                    is_percentage: true,
                });
            }
            2 => {
                // Irregular Galaxy
                modifiers.push(Modifier {
                    modifier_type: ModifierType::EnergyMultiplier,
                    value: 30.0,
                    is_percentage: true,
                });
                modifiers.push(Modifier {
                    modifier_type: ModifierType::TechnologyMultiplier,
                    value: 20.0,
                    is_percentage: true,
                });
            }
            3 => {
                // Dwarf Galaxy
                modifiers.push(Modifier {
                    modifier_type: ModifierType::PopulationMultiplier,
                    value: 40.0,
                    is_percentage: true,
                });
            }
            _ => {
                // Barred Spiral Galaxy
                modifiers.push(Modifier {
                    modifier_type: ModifierType::TradeBonus,
                    value: 35.0,
                    is_percentage: true,
                });
            }
        }

        // Random additional galaxy modifiers
        if self.rng.gen_bool(0.3) {
            modifiers.push(Modifier {
                modifier_type: ModifierType::DefensiveBonus,
                value: self.rng.gen_range(10..50) as f64,
                is_percentage: true,
            });
        }

        modifiers
    }

    /// Generate system-level modifiers
    fn generate_system_modifiers(&mut self) -> Vec<Modifier> {
        let mut modifiers = Vec::new();

        // System type determines modifiers
        let system_type = self.rng.gen_range(0..4);
        match system_type {
            0 => {
                // Binary Star System
                modifiers.push(Modifier {
                    modifier_type: ModifierType::EnergyMultiplier,
                    value: 75.0,
                    is_percentage: true,
                });
                modifiers.push(Modifier {
                    modifier_type: ModifierType::Radiation,
                    value: 25.0,
                    is_percentage: true,
                });
            }
            1 => {
                // Single Star System
                modifiers.push(Modifier {
                    modifier_type: ModifierType::PopulationMultiplier,
                    value: 20.0,
                    is_percentage: true,
                });
            }
            2 => {
                // Multiple Star System
                modifiers.push(Modifier {
                    modifier_type: ModifierType::TechnologyMultiplier,
                    value: 40.0,
                    is_percentage: true,
                });
                modifiers.push(Modifier {
                    modifier_type: ModifierType::MineralMultiplier,
                    value: 30.0,
                    is_percentage: true,
                });
            }
            _ => {
                // Young Star System
                modifiers.push(Modifier {
                    modifier_type: ModifierType::MineralMultiplier,
                    value: 60.0,
                    is_percentage: true,
                });
                modifiers.push(Modifier {
                    modifier_type: ModifierType::PopulationPenalty,
                    value: -30.0,
                    is_percentage: true,
                });
            }
        }

        modifiers
    }

    /// Check if a solar system is fully conquered
    pub fn is_system_conquered(
        &self,
        system: &SolarSystem,
        planets: &HashMap<u64, Planet>,
    ) -> bool {
        system.planets.iter().all(|planet_id| {
            planets
                .get(planet_id)
                .map(|planet| planet.state == PlanetState::Conquered)
                .unwrap_or(false)
        })
    }

    /// Check if a galaxy is fully conquered
    pub fn is_galaxy_conquered(
        &self,
        galaxy: &Galaxy,
        solar_systems: &HashMap<u64, SolarSystem>,
        planets: &HashMap<u64, Planet>,
    ) -> bool {
        galaxy.solar_systems.iter().all(|system_id| {
            solar_systems
                .get(system_id)
                .map(|system| self.is_system_conquered(system, planets))
                .unwrap_or(false)
        })
    }

    /// Get conquest progress for a galaxy
    pub fn get_galaxy_conquest_progress(
        &self,
        galaxy: &Galaxy,
        solar_systems: &HashMap<u64, SolarSystem>,
        planets: &HashMap<u64, Planet>,
    ) -> f64 {
        let total_systems = galaxy.solar_systems.len();
        if total_systems == 0 {
            return 1.0;
        }

        let conquered_systems = galaxy
            .solar_systems
            .iter()
            .filter(|system_id| {
                solar_systems
                    .get(system_id)
                    .map(|system| self.is_system_conquered(system, planets))
                    .unwrap_or(false)
            })
            .count();

        conquered_systems as f64 / total_systems as f64
    }

    /// Get system conquest progress
    pub fn get_system_conquest_progress(
        &self,
        system: &SolarSystem,
        planets: &HashMap<u64, Planet>,
    ) -> f64 {
        let total_planets = system.planets.len();
        if total_planets == 0 {
            return 1.0;
        }

        let conquered_planets = system
            .planets
            .iter()
            .filter(|planet_id| {
                planets
                    .get(planet_id)
                    .map(|planet| planet.state == PlanetState::Conquered)
                    .unwrap_or(false)
            })
            .count();

        conquered_planets as f64 / total_planets as f64
    }

    /// Get next planet ID
    pub fn get_next_planet_id(&mut self) -> u64 {
        self.planet_system.get_next_planet_id()
    }

    /// Generate a new planet (delegates to planet system)
    pub fn generate_planet(&mut self, position: (f64, f64), solar_system_id: u64) -> Planet {
        self.planet_system
            .generate_planet(position, solar_system_id)
    }
}
