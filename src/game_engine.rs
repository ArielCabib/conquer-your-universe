use crate::galaxy_system::GalaxySystem;
use crate::planet_system::PlanetSystem;
use crate::prestige_system::PrestigeSystem;
use crate::resource_system::ResourceSystem;
use crate::supply_chain::SupplyChainSystem;
use crate::transport_system::TransportSystem;
use crate::types::*;
use chrono;
use std::collections::{HashMap, HashSet};
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

/// Main game engine that coordinates all systems
pub struct GameEngine {
    pub game_state: GameState,
    pub resource_system: ResourceSystem,
    pub planet_system: PlanetSystem,
    pub supply_chain_system: SupplyChainSystem,
    pub transport_system: TransportSystem,
    pub galaxy_system: GalaxySystem,
    pub prestige_system: PrestigeSystem,
    pub config: GameConfig,
}

impl GameEngine {
    pub fn new() -> Self {
        let game_state = GameState {
            current_tick: 0,
            game_speed: GameSpeed::Normal,
            is_paused: false,
            current_galaxy: 0,
            galaxies: HashMap::new(),
            solar_systems: HashMap::new(),
            planets: HashMap::new(),
            transport_routes: HashMap::new(),
            resources_in_transit: Vec::new(),
            product_dependencies: HashMap::new(),
            prestige_bonuses: Vec::new(),
            total_prestige_points: 0,
            empire_resources: HashMap::new(),
            explored_solar_systems: HashSet::new(),
            discovered_solar_systems: HashSet::new(),
        };

        let config = GameConfig {
            galaxy_size: 50,
            systems_per_galaxy: 20,
            planets_per_system: 5,
            resource_generation_rate: 1.0,
            conquest_difficulty_scaling: 1.1,
            terraforming_base_cost: 1000,
            transport_base_cost: 100,
        };

        Self {
            game_state,
            resource_system: ResourceSystem::new(),
            planet_system: PlanetSystem::new(),
            supply_chain_system: SupplyChainSystem::new(),
            transport_system: TransportSystem::new(),
            galaxy_system: GalaxySystem::new(),
            prestige_system: PrestigeSystem::new(),
            config,
        }
    }

    /// Initialize the game with starting galaxy
    pub fn initialize_game(&mut self) {
        // Try to load existing save from localStorage first
        if self.load_from_storage() {
            log::info!("Loaded existing game from localStorage");
            return;
        }

        log::info!("No existing save found, initializing new game");
        // Generate starting galaxy
        let mut galaxy = self
            .galaxy_system
            .generate_galaxy("Milky Way".to_string(), self.config.systems_per_galaxy);
        self.game_state.current_galaxy = galaxy.id;

        // Generate solar systems and planets
        let mut solar_system_ids = Vec::new();
        for i in 0..self.config.systems_per_galaxy {
            let (system, planets) = self
                .galaxy_system
                .generate_solar_system_with_planets(galaxy.id, i);
            let system_id = system.id;

            // Store the solar system
            self.game_state.solar_systems.insert(system.id, system);
            solar_system_ids.push(system_id);

            // Store the planets
            for planet in planets {
                log::info!("Storing planet {} in system {}", planet.id, system_id);
                self.game_state.planets.insert(planet.id, planet);
            }
        }

        // Update galaxy with solar system IDs
        galaxy.solar_systems = solar_system_ids.clone();
        self.game_state.galaxies.insert(galaxy.id, galaxy);

        // Initialize exploration state - only reveal the first solar system initially
        if let Some(first_system_id) = solar_system_ids.first() {
            self.game_state
                .explored_solar_systems
                .insert(*first_system_id);
            self.game_state
                .discovered_solar_systems
                .insert(*first_system_id);
        }

        // Conquer the first planet in the first solar system
        if let Some(first_system_id) = solar_system_ids.first() {
            if let Some(first_system) = self.game_state.solar_systems.get(first_system_id) {
                if let Some(first_planet_id) = first_system.planets.first() {
                    if let Some(planet) = self.game_state.planets.get_mut(first_planet_id) {
                        // Conquer the first planet
                        planet.state = PlanetState::Conquered;
                        log::info!(
                            "Conquered starting planet {} in system {}",
                            planet.id,
                            first_system_id
                        );
                    }
                }
            }
        }

        // Initialize empire resources
        self.initialize_empire_resources();

        // Debug: Log the final state
        log::info!(
            "Game initialized with {} galaxies, {} solar systems, {} planets",
            self.game_state.galaxies.len(),
            self.game_state.solar_systems.len(),
            self.game_state.planets.len()
        );
    }

    /// Initialize starting empire resources
    fn initialize_empire_resources(&mut self) {
        self.game_state
            .empire_resources
            .insert(ResourceType::Energy, 1000);
        self.game_state
            .empire_resources
            .insert(ResourceType::Minerals, 500);
        self.game_state
            .empire_resources
            .insert(ResourceType::Population, 100);
        self.game_state
            .empire_resources
            .insert(ResourceType::Technology, 50);
        self.game_state
            .empire_resources
            .insert(ResourceType::Food, 200);
    }

    /// Main game update loop
    pub fn update(&mut self) {
        if self.game_state.is_paused {
            return;
        }

        let speed_multiplier = self.game_state.game_speed as u64;

        // Update all systems
        self.update_resource_generation();
        self.update_factories();
        self.update_transport();
        self.update_terraforming();
        self.update_conquest();
        self.update_exploration();

        // Auto-save every second (assuming 60 ticks per second)
        if self.game_state.current_tick % 60 == 0 {
            self.auto_save();
        }

        // Increment game tick
        self.game_state.current_tick += speed_multiplier;
    }

    /// Update resource generation across all conquered planets
    fn update_resource_generation(&mut self) {
        let conquered_planets: Vec<u64> = self
            .game_state
            .planets
            .values()
            .filter(|planet| planet.state == PlanetState::Conquered)
            .map(|planet| planet.id)
            .collect();

        let empire_generation = self
            .resource_system
            .calculate_empire_resource_generation(&self.game_state.planets, &conquered_planets);

        // Add generated resources to empire
        for (resource_type, amount) in empire_generation {
            *self
                .game_state
                .empire_resources
                .entry(resource_type)
                .or_insert(0) += amount;
        }
    }

    /// Update factory production
    fn update_factories(&mut self) {
        for planet in self.game_state.planets.values_mut() {
            let planet_clone = planet.clone();
            for factory in &mut planet.factories {
                let produced_resources = self.supply_chain_system.update_factory_production(
                    factory,
                    &planet_clone,
                    self.game_state.game_speed,
                );

                // Add produced resources to planet storage
                for resource in produced_resources {
                    *planet.storage.entry(resource.resource_type).or_insert(0) += resource.amount;
                }
            }
        }
    }

    /// Update transport system
    fn update_transport(&mut self) {
        let arrived_resources = self.transport_system.update_transit(
            &mut self.game_state.resources_in_transit,
            self.game_state.game_speed,
        );

        // Distribute arrived resources
        for resource in arrived_resources {
            if let Some(planet) = self.game_state.planets.get_mut(&resource.to_planet) {
                *planet.storage.entry(resource.resource_type).or_insert(0) += resource.amount;
            }
        }
    }

    /// Update terraforming projects
    fn update_terraforming(&mut self) {
        for planet in self.game_state.planets.values_mut() {
            self.planet_system
                .update_terraforming_projects(planet, self.game_state.game_speed);
        }
    }

    /// Update conquest mechanics
    fn update_conquest(&mut self) {
        // This would handle automatic conquest processes
        // For now, conquest is manual through player actions
    }

    /// Attempt to conquer a planet
    pub fn attempt_planet_conquest(&mut self, planet_id: u64) -> ConquestResult {
        // First, get a copy of the planet to calculate cost
        let planet = if let Some(planet) = self.game_state.planets.get(&planet_id) {
            planet.clone()
        } else {
            return ConquestResult::PlanetNotFound;
        };

        match planet.state {
            PlanetState::Unexplored => {
                // Calculate conquest cost
                let conquest_cost = self.calculate_conquest_cost(&planet);

                // Check if player can afford conquest
                if self
                    .resource_system
                    .can_afford(&self.game_state.empire_resources, &conquest_cost)
                {
                    // Deduct resources
                    self.resource_system
                        .deduct_resources(&mut self.game_state.empire_resources, &conquest_cost);

                    // Change planet state
                    if let Some(planet) = self.game_state.planets.get_mut(&planet_id) {
                        planet.state = PlanetState::Conquered;

                        // Explore the solar system when a planet is conquered
                        if let Some(system) =
                            self.game_state.solar_systems.get(&planet.solar_system_id)
                        {
                            self.explore_solar_system(system.id);
                        }
                    }

                    ConquestResult::Success {
                        planet_id,
                        cost: conquest_cost,
                    }
                } else {
                    ConquestResult::InsufficientResources {
                        required: conquest_cost,
                        available: self.game_state.empire_resources.clone(),
                    }
                }
            }
            PlanetState::Explored => ConquestResult::AlreadyExplored,
            PlanetState::Conquered => ConquestResult::AlreadyConquered,
            PlanetState::Terraforming => ConquestResult::CurrentlyTerraforming,
        }
    }

    /// Calculate conquest cost for a planet
    fn calculate_conquest_cost(&self, planet: &Planet) -> HashMap<ResourceType, u64> {
        let mut cost = HashMap::new();

        // Base conquest cost
        cost.insert(ResourceType::Energy, 100);
        cost.insert(ResourceType::Minerals, 50);
        cost.insert(ResourceType::Population, 25);

        // Apply planet modifiers to cost
        for modifier in &planet.modifiers {
            match modifier.modifier_type {
                ModifierType::DefensiveBonus => {
                    // Higher defensive bonus = higher cost
                    let energy_cost = cost.get(&ResourceType::Energy).copied().unwrap_or(0);
                    cost.insert(
                        ResourceType::Energy,
                        energy_cost + (modifier.value * 2.0) as u64,
                    );
                }
                ModifierType::ResourcePenalty => {
                    // Resource penalties make conquest cheaper
                    for (_resource_type, amount) in cost.iter_mut() {
                        *amount = (*amount as f64 * (1.0 - modifier.value.abs() / 100.0)) as u64;
                    }
                }
                _ => {}
            }
        }

        // Apply galaxy conquest difficulty scaling
        let scaling_factor = self.config.conquest_difficulty_scaling;
        for (_resource_type, amount) in cost.iter_mut() {
            *amount = (*amount as f64 * scaling_factor) as u64;
        }

        cost
    }

    /// Set game speed
    pub fn set_game_speed(&mut self, speed: GameSpeed) {
        self.game_state.game_speed = speed;
    }

    /// Pause/unpause game
    pub fn toggle_pause(&mut self) {
        self.game_state.is_paused = !self.game_state.is_paused;
    }

    /// Start terraforming project on a planet
    pub fn start_terraforming_project(
        &mut self,
        planet_id: u64,
        target_modifier: ModifierType,
        required_resources: HashMap<ResourceType, u64>,
        duration: u64,
        energy_cost: u64,
    ) -> bool {
        if let Some(planet) = self.game_state.planets.get_mut(&planet_id) {
            self.planet_system.start_terraforming_project(
                planet,
                target_modifier,
                required_resources,
                duration,
                energy_cost,
            )
        } else {
            false
        }
    }

    /// Add factory to a planet
    pub fn add_factory(&mut self, planet_id: u64, factory_type: FactoryType) -> Option<u64> {
        if let Some(planet) = self.game_state.planets.get_mut(&planet_id) {
            Some(self.planet_system.add_factory(planet, factory_type))
        } else {
            None
        }
    }

    /// Start resource transport between planets
    pub fn start_resource_transport(
        &mut self,
        from_planet: u64,
        to_planet: u64,
        resource_type: ResourceType,
        amount: u64,
    ) -> bool {
        if let (Some(from), Some(to)) = (
            self.game_state.planets.get(&from_planet),
            self.game_state.planets.get(&to_planet),
        ) {
            let distance = ((from.position.0 - to.position.0).powi(2)
                + (from.position.1 - to.position.1).powi(2))
            .sqrt();
            let transport = self.transport_system.start_transport(
                from_planet,
                to_planet,
                resource_type,
                amount,
                distance,
                self.game_state.game_speed,
            );
            self.game_state.resources_in_transit.push(transport);
            true
        } else {
            false
        }
    }

    /// Check if galaxy is ready for prestige
    pub fn check_prestige_eligibility(&self) -> bool {
        if let Some(galaxy) = self
            .game_state
            .galaxies
            .get(&self.game_state.current_galaxy)
        {
            let progress = self.galaxy_system.get_galaxy_conquest_progress(
                galaxy,
                &self.game_state.solar_systems,
                &self.game_state.planets,
            );
            self.prestige_system
                .can_prestige(self.game_state.total_prestige_points, progress)
        } else {
            false
        }
    }

    /// Perform prestige to next galaxy
    pub fn perform_prestige(&mut self) -> bool {
        if !self.check_prestige_eligibility() {
            return false;
        }

        // Calculate prestige points and bonuses
        if let Some(galaxy) = self
            .game_state
            .galaxies
            .get(&self.game_state.current_galaxy)
        {
            let prestige_points = self.prestige_system.calculate_galaxy_prestige_points(
                galaxy,
                self.game_state.current_tick,
                0.8, // Efficiency score - would be calculated from actual performance
            );

            let bonuses = self
                .prestige_system
                .create_prestige_bonuses(prestige_points, &galaxy.galaxy_modifiers);

            // Apply prestige bonuses
            self.prestige_system
                .apply_prestige_bonuses(&mut self.game_state, &bonuses);

            // Update prestige points
            self.game_state.total_prestige_points += prestige_points;
            self.game_state.prestige_bonuses.extend(bonuses);

            // Generate new galaxy
            let new_galaxy = self.galaxy_system.generate_galaxy(
                format!("Galaxy {}", self.game_state.galaxies.len() + 1),
                self.config.systems_per_galaxy,
            );
            self.game_state.current_galaxy = new_galaxy.id;
            self.game_state.galaxies.insert(new_galaxy.id, new_galaxy);

            // Reset game state for new galaxy
            self.game_state.current_tick = 0;
            self.initialize_empire_resources();

            true
        } else {
            false
        }
    }

    /// Save game state
    pub fn save_game(&self) -> String {
        serde_json::to_string(&self.game_state).unwrap_or_default()
    }

    /// Load game state
    pub fn load_game(&mut self, save_data: &str) -> bool {
        match serde_json::from_str::<GameState>(save_data) {
            Ok(game_state) => {
                self.game_state = game_state;
                true
            }
            Err(_) => false,
        }
    }

    /// Auto-save game state to localStorage
    pub fn auto_save(&self) {
        let save_data = self.save_game();
        if let Some(window) = web_sys::window() {
            if let Ok(storage) = window.local_storage() {
                if let Some(storage) = storage {
                    let _ = storage.set_item("conquer_universe_save", &save_data);
                    log::info!("Game auto-saved to localStorage");
                }
            }
        }
    }

    /// Load game state from localStorage
    pub fn load_from_storage(&mut self) -> bool {
        if let Some(window) = web_sys::window() {
            if let Ok(storage) = window.local_storage() {
                if let Some(storage) = storage {
                    if let Ok(Some(save_data)) = storage.get_item("conquer_universe_save") {
                        let success = self.load_game(&save_data);
                        if success {
                            log::info!("Game loaded from localStorage");
                        } else {
                            log::warn!("Failed to load game from localStorage");
                        }
                        return success;
                    }
                }
            }
        }
        log::info!("No save data found in localStorage");
        false
    }

    /// Save game to JSON file (download)
    pub fn save_to_json_file(&self) {
        let save_data = self.save_game();
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let filename = format!("conquer_universe_save_{}.json", timestamp);

        if let Some(window) = web_sys::window() {
            // Create a blob with the save data
            let blob_options = web_sys::BlobPropertyBag::new();
            blob_options.set_type("application/json");
            let blob = web_sys::Blob::new_with_str_sequence_and_options(
                &js_sys::Array::from_iter([JsValue::from_str(&save_data)]),
                &blob_options,
            )
            .ok();

            if let Some(blob) = blob {
                // Create download URL
                let url = web_sys::Url::create_object_url_with_blob(&blob).ok();
                if let Some(url) = url {
                    // Create download link
                    if let Some(document) = window.document() {
                        if let Ok(link) = document.create_element("a") {
                            let _ = link.set_attribute("href", &url);
                            let _ = link.set_attribute("download", &filename);
                            let _ = link.set_attribute("style", "display: none");

                            if let Some(body) = document.body() {
                                let _ = body.append_child(&link);
                                let html_link = link.dyn_into::<web_sys::HtmlElement>().unwrap();
                                let _ = html_link.click();
                                let _ = body.remove_child(&html_link);
                            }

                            // Clean up URL
                            let _ = web_sys::Url::revoke_object_url(&url);
                            log::info!("Game saved to JSON file: {}", filename);
                        }
                    }
                }
            }
        }
    }

    /// Load game from JSON file (upload)
    pub fn load_from_json_file(&mut self, file_content: &str) -> bool {
        let success = self.load_game(file_content);
        if success {
            log::info!("Game loaded from JSON file");
        } else {
            log::warn!("Failed to load game from JSON file");
        }
        success
    }

    /// Get game statistics
    pub fn get_game_statistics(&self) -> GameStatistics {
        let conquered_planets = self
            .game_state
            .planets
            .values()
            .filter(|planet| planet.state == PlanetState::Conquered)
            .count();

        let total_planets = self.game_state.planets.len();
        let total_factories: usize = self
            .game_state
            .planets
            .values()
            .map(|planet| planet.factories.len())
            .sum();

        let total_resources: u64 = self.game_state.empire_resources.values().sum();

        GameStatistics {
            current_tick: self.game_state.current_tick,
            conquered_planets,
            total_planets,
            total_factories,
            total_resources,
            prestige_points: self.game_state.total_prestige_points,
            game_speed: self.game_state.game_speed,
            is_paused: self.game_state.is_paused,
        }
    }

    /// Discover a new solar system (make it visible but not explored)
    pub fn discover_solar_system(&mut self, system_id: u64) {
        self.game_state.discovered_solar_systems.insert(system_id);
    }

    /// Explore a solar system (make it fully visible and explorable)
    pub fn explore_solar_system(&mut self, system_id: u64) {
        self.game_state.explored_solar_systems.insert(system_id);
        self.game_state.discovered_solar_systems.insert(system_id);
    }

    /// Check if a solar system is discovered
    pub fn is_solar_system_discovered(&self, system_id: u64) -> bool {
        self.game_state
            .discovered_solar_systems
            .contains(&system_id)
    }

    /// Check if a solar system is explored
    pub fn is_solar_system_explored(&self, system_id: u64) -> bool {
        self.game_state.explored_solar_systems.contains(&system_id)
    }

    /// Get discovered solar systems
    pub fn get_discovered_solar_systems(&self) -> &HashSet<u64> {
        &self.game_state.discovered_solar_systems
    }

    /// Get explored solar systems
    pub fn get_explored_solar_systems(&self) -> &HashSet<u64> {
        &self.game_state.explored_solar_systems
    }

    /// Get total planet count for debugging
    pub fn get_planet_count(&self) -> usize {
        self.game_state.planets.len()
    }

    /// Update exploration - gradually discover new solar systems
    fn update_exploration(&mut self) {
        // Exploration is now manual - no automatic discovery
        // Players must use exploration mechanics to discover new systems
    }
}

/// Game statistics for display
#[derive(Debug, Clone, PartialEq)]
pub struct GameStatistics {
    pub current_tick: u64,
    pub conquered_planets: usize,
    pub total_planets: usize,
    pub total_factories: usize,
    pub total_resources: u64,
    pub prestige_points: u64,
    pub game_speed: GameSpeed,
    pub is_paused: bool,
}
