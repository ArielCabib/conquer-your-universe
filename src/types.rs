use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

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

/// Building for producing products
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Building {
    pub id: u64,
    pub building_type: BuildingType,
    pub production_queue: Vec<ProductionOrder>,
    pub efficiency: f64,
    pub is_active: bool,
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
