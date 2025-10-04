use crate::types::*;
use std::collections::HashMap;

/// Transport and resource flow system
pub struct TransportSystem {
    route_counter: u64,
    transit_counter: u64,
}

impl TransportSystem {
    pub fn new() -> Self {
        Self {
            route_counter: 0,
            transit_counter: 0,
        }
    }

    /// Create a transport route between two planets
    pub fn create_route(
        &mut self,
        from_planet: u64,
        to_planet: u64,
        distance: f64,
        capacity: u64,
    ) -> u64 {
        let route_id = self.route_counter;
        self.route_counter += 1;

        let _route = TransportRoute {
            id: route_id,
            from_planet,
            to_planet,
            capacity,
            efficiency: 1.0,
            energy_cost: (distance * 0.1) as u64, // Base energy cost per distance unit
            distance,
        };

        route_id
    }

    /// Calculate transport cost between two planets
    pub fn calculate_transport_cost(
        &self,
        from_planet: &Planet,
        to_planet: &Planet,
        resource_type: &ResourceType,
        amount: u64,
    ) -> u64 {
        let distance = self.calculate_distance(from_planet.position, to_planet.position);
        let base_cost = distance * 0.1; // Base cost per distance unit
        
        // Resource-specific multipliers
        let resource_multiplier = match resource_type {
            ResourceType::Energy => 0.5, // Energy is easier to transport
            ResourceType::Population => 2.0, // Population is harder to transport
            ResourceType::Food => 1.5, // Food requires special handling
            ResourceType::Technology => 1.2, // Technology requires careful transport
            ResourceType::Minerals => 1.0, // Standard transport
            _ => 1.0,
        };

        (base_cost * resource_multiplier * amount as f64) as u64
    }

    /// Calculate distance between two points
    fn calculate_distance(&self, pos1: (f64, f64), pos2: (f64, f64)) -> f64 {
        let dx = pos2.0 - pos1.0;
        let dy = pos2.1 - pos1.1;
        (dx * dx + dy * dy).sqrt()
    }

    /// Start resource transport between planets
    pub fn start_transport(
        &mut self,
        from_planet: u64,
        to_planet: u64,
        resource_type: ResourceType,
        amount: u64,
        distance: f64,
        game_speed: GameSpeed,
    ) -> ResourceInTransit {
        let _transit_id = self.transit_counter;
        self.transit_counter += 1;

        // Calculate transport time based on distance and speed
        let base_transport_time = (distance * 10.0) as u64; // Base time in ticks
        let speed_multiplier = game_speed as u64;
        let transport_time = base_transport_time / speed_multiplier.max(1);

        ResourceInTransit {
            resource_type,
            amount,
            from_planet,
            to_planet,
            arrival_time: transport_time,
            route_id: 0, // Will be set by route management
        }
    }

    /// Update all resources in transit
    pub fn update_transit(
        &mut self,
        resources_in_transit: &mut Vec<ResourceInTransit>,
        game_speed: GameSpeed,
    ) -> Vec<ResourceInTransit> {
        let mut arrived_resources = Vec::new();
        let speed_multiplier = game_speed as u64;

        for resource in resources_in_transit.iter_mut() {
            if resource.arrival_time > 0 {
                resource.arrival_time = resource.arrival_time.saturating_sub(speed_multiplier);
                
                if resource.arrival_time == 0 {
                    arrived_resources.push(resource.clone());
                }
            }
        }

        // Remove arrived resources from transit
        resources_in_transit.retain(|resource| resource.arrival_time > 0);
        
        arrived_resources
    }

    /// Find the optimal route for resource transport
    pub fn find_optimal_route(
        &self,
        from_planet: u64,
        to_planet: u64,
        routes: &HashMap<u64, TransportRoute>,
    ) -> Option<u64> {
        // For now, return direct route if it exists
        // In a more complex implementation, this would use pathfinding algorithms
        for (route_id, route) in routes {
            if route.from_planet == from_planet && route.to_planet == to_planet {
                return Some(*route_id);
            }
        }
        None
    }

    /// Calculate empire-wide resource distribution
    pub fn calculate_resource_distribution(
        &self,
        planets: &HashMap<u64, Planet>,
        empire_resources: &HashMap<ResourceType, u64>,
        conquered_planets: &[u64],
    ) -> HashMap<u64, HashMap<ResourceType, u64>> {
        let mut distribution = HashMap::new();

        // Simple distribution: allocate resources based on planet needs and production
        for planet_id in conquered_planets {
            if let Some(planet) = planets.get(planet_id) {
                let mut planet_allocation = HashMap::new();
                
                // Allocate resources based on planet's production capabilities
                for (resource_type, empire_amount) in empire_resources {
                    let planet_need = self.calculate_planet_resource_need(planet, *resource_type);
                    let allocation = ((*empire_amount as f64) * planet_need) as u64;
                let allocation = allocation.min(*empire_amount);
                    
                    if allocation > 0 {
                        planet_allocation.insert(*resource_type, allocation);
                    }
                }
                
                distribution.insert(*planet_id, planet_allocation);
            }
        }

        distribution
    }

    /// Calculate how much a planet needs a specific resource
    fn calculate_planet_resource_need(&self, planet: &Planet, resource_type: ResourceType) -> f64 {
        // Base need calculation - can be made more sophisticated
        match resource_type {
            ResourceType::Energy => {
                // Energy need based on planet activity
                let base_need = 0.1;
                let modifier_bonus = planet.modifiers.iter()
                    .filter(|m| m.modifier_type == ModifierType::EnergyMultiplier)
                    .map(|m| m.value / 100.0)
                    .sum::<f64>();
                base_need + modifier_bonus
            },
            ResourceType::Minerals => {
                // Mineral need based on manufacturing
                let factory_count = planet.factories.len() as f64;
                0.05 + (factory_count * 0.02)
            },
            ResourceType::Population => {
                // Population need based on planet capacity
                let current_pop = planet.resources.get(&ResourceType::Population).copied().unwrap_or(0) as f64;
                let max_capacity = 1000.0; // Base capacity
                (max_capacity - current_pop) / max_capacity
            },
            ResourceType::Technology => {
                // Technology need based on research facilities
                let research_factories = planet.factories.iter()
                    .filter(|f| f.factory_type == FactoryType::Research)
                    .count() as f64;
                0.02 + (research_factories * 0.05)
            },
            ResourceType::Food => {
                // Food need based on population
                let population = planet.resources.get(&ResourceType::Population).copied().unwrap_or(0) as f64;
                population * 0.1
            },
            _ => 0.0,
        }
    }

    /// Optimize transport routes for efficiency
    pub fn optimize_routes(
        &self,
        routes: &mut HashMap<u64, TransportRoute>,
        planets: &HashMap<u64, Planet>,
    ) {
        // Simple optimization: increase efficiency for frequently used routes
        for route in routes.values_mut() {
            // Calculate route usage based on distance and capacity
            let efficiency_bonus = if route.distance < 100.0 {
                0.1 // Short routes get efficiency bonus
            } else if route.distance > 500.0 {
                -0.05 // Long routes get efficiency penalty
            } else {
                0.0
            };
            
            route.efficiency = (1.0_f64 + efficiency_bonus).max(0.1_f64);
        }
    }

    /// Calculate total transport capacity for a planet
    pub fn calculate_planet_transport_capacity(
        &self,
        planet_id: u64,
        routes: &HashMap<u64, TransportRoute>,
    ) -> u64 {
        routes.values()
            .filter(|route| route.from_planet == planet_id || route.to_planet == planet_id)
            .map(|route| route.capacity)
            .sum()
    }

    /// Get transport statistics for empire overview
    pub fn get_transport_statistics(
        &self,
        routes: &HashMap<u64, TransportRoute>,
        resources_in_transit: &[ResourceInTransit],
    ) -> TransportStatistics {
        let total_routes = routes.len();
        let total_capacity: u64 = routes.values().map(|route| route.capacity).sum();
        let resources_in_transit_count = resources_in_transit.len();
        let total_resources_in_transit: u64 = resources_in_transit.iter()
            .map(|resource| resource.amount)
            .sum();

        TransportStatistics {
            total_routes,
            total_capacity,
            resources_in_transit_count,
            total_resources_in_transit,
            average_route_efficiency: routes.values()
                .map(|route| route.efficiency)
                .sum::<f64>() / total_routes.max(1) as f64,
        }
    }
}

/// Transport statistics for empire overview
#[derive(Debug, Clone)]
pub struct TransportStatistics {
    pub total_routes: usize,
    pub total_capacity: u64,
    pub resources_in_transit_count: usize,
    pub total_resources_in_transit: u64,
    pub average_route_efficiency: f64,
}
