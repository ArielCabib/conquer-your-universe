use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

// Population and housing balancing constants
pub const HOUSING_CAPACITY_PER_LEVEL: u64 = 100;
pub const HOUSING_FOOD_UPKEEP_PER_LEVEL: u64 = 1;
pub const HOUSING_PRODUCTION_PER_LEVEL: u64 = 1;
pub const POPULATION_MIN_LIFESPAN_SECONDS: u64 = 90;
pub const POPULATION_MAX_LIFESPAN_SECONDS: u64 = 180;
pub const STARVATION_LOSS_FRACTION: f64 = 0.25;
pub const STARVATION_MIN_LOSS: u64 = 5;

/// Core resource types in the game
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceType {
    Energy,
    Minerals,
    Population,
    Technology,
    Food,
    // Advanced resources
    Alloys,
    Electronics,
    Medicine,
    Starships,
    AdvancedWeapons,
    AISystems,
    // End-game resources
    DysonSpheres,
    GalacticNetworks,
}

/// Resource amount with type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceAmount {
    pub resource_type: ResourceType,
    pub amount: u64,
}

/// Planet modifier types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModifierType {
    // Environmental modifiers
    Gravity,
    Atmosphere,
    Temperature,
    Radiation,

    // Resource modifiers
    EnergyMultiplier,
    MineralMultiplier,
    PopulationMultiplier,
    TechnologyMultiplier,
    FoodMultiplier,

    // Strategic modifiers
    DefensiveBonus,
    ResearchBonus,
    TradeBonus,
    ManufacturingBonus,

    // Negative modifiers
    ToxicAtmosphere,
    HighRadiation,
    ExtremeGravity,
    ResourcePenalty,
    PopulationPenalty,
}

/// A planet modifier with value and type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Modifier {
    pub modifier_type: ModifierType,
    pub value: f64,          // Can be positive or negative
    pub is_percentage: bool, // true for multipliers, false for flat bonuses
}

/// Planet class/types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlanetClass {
    Barren,
    Terran,
    GasGiant,
    Ocean,
    Desert,
    Ice,
    Volcanic,
    Toxic,
    Crystalline,
    Metallic,
}

/// Planet state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PlanetState {
    Unexplored,
    Explored,
    Conquered,
    Terraforming,
}

/// A planet in the game
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Planet {
    pub id: u64,
    pub name: String,
    pub class: PlanetClass,
    pub state: PlanetState,
    pub resources: HashMap<ResourceType, u64>,
    pub modifiers: Vec<Modifier>,
    pub terraforming_projects: Vec<TerraformingProject>,
    pub buildings: Vec<Building>,
    pub storage: HashMap<ResourceType, u64>,
    pub position: (f64, f64), // x, y coordinates
    pub solar_system_id: u64,
    #[serde(default)]
    pub population_cohorts: Vec<PopulationCohort>,
}

/// Terraforming project
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TerraformingProject {
    pub id: u64,
    pub name: String,
    pub target_modifier: ModifierType,
    pub required_resources: HashMap<ResourceType, u64>,
    pub progress: f64, // 0.0 to 1.0
    pub duration: u64, // in game ticks
    pub energy_cost: u64,
}

/// Tracks population groups with a shared death tick
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PopulationCohort {
    pub amount: u64,
    pub death_tick: u64,
}

/// Building for producing products
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Building {
    pub id: u64,
    pub building_type: BuildingType,
    pub production_queue: Vec<ProductionOrder>,
    pub efficiency: f64,
    pub is_active: bool,
    #[serde(default = "default_building_level")]
    pub level: u8,
}

const fn default_building_level() -> u8 {
    1
}

/// Building types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BuildingType {
    BasicManufacturing,
    AdvancedManufacturing,
    Electronics,
    Pharmaceuticals,
    Shipyard,
    Weapons,
    Research,
    Housing,
    Observatory,
    Farming,
    PowerPlant,
    Mining,
}

/// Production order in a building
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductionOrder {
    pub product: ResourceType,
    pub quantity: u64,
    pub priority: u8,  // 0-255, higher is more important
    pub progress: f64, // 0.0 to 1.0
}

/// Product dependency in the DAG
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductDependency {
    pub product: ResourceType,
    pub dependencies: Vec<ResourceAmount>,
    pub production_time: u64, // in game ticks
    pub energy_cost: u64,
}

/// Transport route between planets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportRoute {
    pub id: u64,
    pub from_planet: u64,
    pub to_planet: u64,
    pub capacity: u64,
    pub efficiency: f64,
    pub energy_cost: u64,
    pub distance: f64,
}

/// Resource in transit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceInTransit {
    pub resource_type: ResourceType,
    pub amount: u64,
    pub from_planet: u64,
    pub to_planet: u64,
    pub arrival_time: u64,
    pub route_id: u64,
}

/// Solar system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SolarSystem {
    pub id: u64,
    pub name: String,
    pub planets: Vec<u64>, // Planet IDs
    pub system_modifiers: Vec<Modifier>,
    pub position: (f64, f64),
    pub is_conquered: bool,
}

/// Galaxy
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Galaxy {
    pub id: u64,
    pub name: String,
    pub solar_systems: Vec<u64>, // Solar system IDs
    pub galaxy_modifiers: Vec<Modifier>,
    pub is_conquered: bool,
}

/// Game speed multiplier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameSpeed {
    Normal = 1,
    Fast = 10,
    VeryFast = 100,
    UltraFast = 1000,
}

/// View mode for the game interface
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViewMode {
    Galaxy,
    SolarSystem,
    Planet,
}

impl Planet {
    /// Total number of housing units (including upgrades) on the planet.
    pub fn housing_units(&self) -> u64 {
        self.buildings
            .iter()
            .filter(|building| building.building_type == BuildingType::Housing)
            .map(|building| building.level as u64)
            .sum()
    }

    /// Maximum population supported by current housing.
    pub fn population_capacity(&self) -> u64 {
        self.housing_units() * HOUSING_CAPACITY_PER_LEVEL
    }
}

/// Determine the resource cost for constructing a building on a planet.
pub fn building_cost(building_type: BuildingType, current_level: u8) -> HashMap<ResourceType, u64> {
    let mut cost = match building_type {
        BuildingType::BasicManufacturing => HashMap::from([
            (ResourceType::Energy, 200),
            (ResourceType::Minerals, 100),
            (ResourceType::Population, 50),
        ]),
        BuildingType::AdvancedManufacturing => HashMap::from([
            (ResourceType::Energy, 500),
            (ResourceType::Minerals, 300),
            (ResourceType::Population, 100),
            (ResourceType::Technology, 50),
        ]),
        BuildingType::Electronics => HashMap::from([
            (ResourceType::Energy, 300),
            (ResourceType::Minerals, 200),
            (ResourceType::Population, 75),
            (ResourceType::Technology, 100),
        ]),
        BuildingType::Pharmaceuticals => HashMap::from([
            (ResourceType::Energy, 250),
            (ResourceType::Minerals, 150),
            (ResourceType::Population, 60),
            (ResourceType::Technology, 80),
        ]),
        BuildingType::Shipyard => HashMap::from([
            (ResourceType::Energy, 800),
            (ResourceType::Minerals, 600),
            (ResourceType::Population, 200),
            (ResourceType::Technology, 150),
        ]),
        BuildingType::Weapons => HashMap::from([
            (ResourceType::Energy, 400),
            (ResourceType::Minerals, 500),
            (ResourceType::Population, 150),
            (ResourceType::Technology, 200),
        ]),
        BuildingType::Research => HashMap::from([
            (ResourceType::Energy, 300),
            (ResourceType::Minerals, 200),
            (ResourceType::Population, 100),
            (ResourceType::Technology, 100),
        ]),
        BuildingType::Observatory => HashMap::from([
            (ResourceType::Energy, 250),
            (ResourceType::Minerals, 150),
            (ResourceType::Technology, 120),
        ]),
        BuildingType::Farming => HashMap::from([
            (ResourceType::Energy, 180),
            (ResourceType::Minerals, 90),
            (ResourceType::Population, 40),
        ]),
        BuildingType::PowerPlant => HashMap::from([
            (ResourceType::Energy, 220),
            (ResourceType::Minerals, 160),
            (ResourceType::Population, 60),
        ]),
        BuildingType::Mining => HashMap::from([
            (ResourceType::Energy, 200),
            (ResourceType::Minerals, 180),
            (ResourceType::Population, 80),
        ]),
        BuildingType::Housing => HashMap::from([
            (ResourceType::Energy, 10),
            (ResourceType::Minerals, 7),
            (ResourceType::Food, 10),
        ]),
    };

    let multiplier = match building_type {
        BuildingType::Housing => 1.25_f64.powi(current_level as i32),
        _ => 1.35_f64.powi(current_level as i32),
    };

    if multiplier > 1.0 {
        for value in cost.values_mut() {
            *value = ((*value as f64) * multiplier).ceil() as u64;
        }
    }

    cost
}

/// Prestige bonus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrestigeBonus {
    pub bonus_type: PrestigeBonusType,
    pub value: f64,
    pub is_percentage: bool,
}

/// Types of prestige bonuses
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PrestigeBonusType {
    ResourceMultiplier,
    ResearchSpeed,
    ConquestSpeed,
    TerraformingSpeed,
    TransportEfficiency,
    BuildingEfficiency,
    StartingResources,
    GalaxyModifier,
}

/// Game state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub current_tick: u64,
    pub game_speed: GameSpeed,
    pub is_paused: bool,
    pub current_galaxy: u64,
    pub galaxies: HashMap<u64, Galaxy>,
    pub solar_systems: HashMap<u64, SolarSystem>,
    pub planets: HashMap<u64, Planet>,
    pub transport_routes: HashMap<u64, TransportRoute>,
    pub resources_in_transit: Vec<ResourceInTransit>,
    pub product_dependencies: HashMap<ResourceType, ProductDependency>,
    pub prestige_bonuses: Vec<PrestigeBonus>,
    pub total_prestige_points: u64,
    pub empire_resources: HashMap<ResourceType, u64>,
    pub explored_solar_systems: HashSet<u64>,
    pub discovered_solar_systems: HashSet<u64>,
    #[serde(default)]
    pub last_resource_generation: HashMap<ResourceType, u64>,
}

/// Conquest result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConquestResult {
    Success {
        planet_id: u64,
        cost: HashMap<ResourceType, u64>,
    },
    InsufficientResources {
        required: HashMap<ResourceType, u64>,
        available: HashMap<ResourceType, u64>,
    },
    AlreadyConquered,
    AlreadyExplored,
    CurrentlyTerraforming,
    PlanetNotFound,
}

/// Game configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub galaxy_size: u32,
    pub systems_per_galaxy: u32,
    pub planets_per_system: u32,
    pub resource_generation_rate: f64,
    pub conquest_difficulty_scaling: f64,
    pub terraforming_base_cost: u64,
    pub transport_base_cost: u64,
}
