use crate::types::*;
use std::collections::HashMap;

/// Resource generation and management system
pub struct ResourceSystem {
    resource_generation_rates: HashMap<ResourceType, f64>,
    resource_storage_limits: HashMap<ResourceType, u64>,
}

impl ResourceSystem {
    pub fn new() -> Self {
        let mut resource_generation_rates = HashMap::new();
        let mut resource_storage_limits = HashMap::new();

        // Set base generation rates (per tick)
        resource_generation_rates.insert(ResourceType::Energy, 1.0);
        resource_generation_rates.insert(ResourceType::Minerals, 0.8);
        resource_generation_rates.insert(ResourceType::Population, 0.5);
        resource_generation_rates.insert(ResourceType::Technology, 0.3);
        resource_generation_rates.insert(ResourceType::Food, 0.7);

        // Set storage limits
        resource_storage_limits.insert(ResourceType::Energy, 10000);
        resource_storage_limits.insert(ResourceType::Minerals, 10000);
        resource_storage_limits.insert(ResourceType::Population, 5000);
        resource_storage_limits.insert(ResourceType::Technology, 2000);
        resource_storage_limits.insert(ResourceType::Food, 8000);

        Self {
            resource_generation_rates,
            resource_storage_limits,
        }
    }

    /// Calculate resource generation for a planet based on its modifiers
    pub fn calculate_planet_resource_generation(
        &self,
        planet: &Planet,
        base_resources: &HashMap<ResourceType, u64>,
    ) -> HashMap<ResourceType, u64> {
        let mut generated_resources = HashMap::new();

        for (resource_type, base_amount) in base_resources {
            let mut generation_rate = self
                .resource_generation_rates
                .get(resource_type)
                .copied()
                .unwrap_or(0.0);

            // Apply planet modifiers
            for modifier in &planet.modifiers {
                if self.modifier_affects_resource(&modifier.modifier_type, *resource_type) {
                    generation_rate = self.apply_modifier(generation_rate, modifier);
                }
            }

            // Apply terraforming bonuses
            for project in &planet.terraforming_projects {
                if project.progress >= 1.0 {
                    // Completed terraforming projects might provide bonuses
                    // This would be implemented based on specific project types
                }
            }

            let generated_amount = ((*base_amount as f64) * generation_rate) as u64;
            generated_resources.insert(*resource_type, generated_amount);
        }

        generated_resources
    }

    /// Check if a modifier affects a specific resource type
    fn modifier_affects_resource(
        &self,
        modifier_type: &ModifierType,
        resource_type: ResourceType,
    ) -> bool {
        match (modifier_type, resource_type) {
            (ModifierType::EnergyMultiplier, ResourceType::Energy) => true,
            (ModifierType::MineralMultiplier, ResourceType::Minerals) => true,
            (ModifierType::PopulationMultiplier, ResourceType::Population) => true,
            (ModifierType::TechnologyMultiplier, ResourceType::Technology) => true,
            (ModifierType::FoodMultiplier, ResourceType::Food) => true,
            (ModifierType::ResourcePenalty, _) => true,
            _ => false,
        }
    }

    /// Apply a modifier to a value
    fn apply_modifier(&self, base_value: f64, modifier: &Modifier) -> f64 {
        if modifier.is_percentage {
            base_value * (1.0 + modifier.value / 100.0)
        } else {
            base_value + modifier.value
        }
    }

    /// Calculate empire-wide resource generation
    pub fn calculate_empire_resource_generation(
        &self,
        planets: &HashMap<u64, Planet>,
        conquered_planets: &[u64],
    ) -> HashMap<ResourceType, u64> {
        let mut empire_resources = HashMap::new();

        for planet_id in conquered_planets {
            if let Some(planet) = planets.get(planet_id) {
                let planet_resources =
                    self.calculate_planet_resource_generation(planet, &planet.resources);

                for (resource_type, amount) in planet_resources {
                    *empire_resources.entry(resource_type).or_insert(0) += amount;
                }
            }
        }

        empire_resources
    }

    /// Check if there are enough resources for a cost
    pub fn can_afford(
        &self,
        available: &HashMap<ResourceType, u64>,
        cost: &HashMap<ResourceType, u64>,
    ) -> bool {
        for (resource_type, required_amount) in cost {
            let available_amount = available.get(resource_type).copied().unwrap_or(0);
            if available_amount < *required_amount {
                return false;
            }
        }
        true
    }

    /// Deduct resources from available resources
    pub fn deduct_resources(
        &self,
        available: &mut HashMap<ResourceType, u64>,
        cost: &HashMap<ResourceType, u64>,
    ) -> bool {
        if !self.can_afford(available, cost) {
            return false;
        }

        for (resource_type, required_amount) in cost {
            if let Some(available_amount) = available.get_mut(resource_type) {
                *available_amount -= required_amount;
            }
        }

        true
    }

    /// Add resources to available resources
    pub fn add_resources(
        &self,
        available: &mut HashMap<ResourceType, u64>,
        resources: &HashMap<ResourceType, u64>,
    ) {
        for (resource_type, amount) in resources {
            *available.entry(*resource_type).or_insert(0) += amount;
        }
    }

    /// Calculate storage capacity for a resource type
    pub fn get_storage_capacity(&self, resource_type: &ResourceType) -> u64 {
        self.resource_storage_limits
            .get(resource_type)
            .copied()
            .unwrap_or(1000)
    }

    /// Get all storage limits
    pub fn get_storage_limits(&self) -> HashMap<ResourceType, u64> {
        self.resource_storage_limits.clone()
    }

    /// Check if adding resources would exceed storage capacity
    pub fn would_exceed_capacity(
        &self,
        current: &HashMap<ResourceType, u64>,
        to_add: &HashMap<ResourceType, u64>,
    ) -> bool {
        for (resource_type, amount_to_add) in to_add {
            let current_amount = current.get(resource_type).copied().unwrap_or(0);
            let capacity = self.get_storage_capacity(resource_type);

            if current_amount + amount_to_add > capacity {
                return true;
            }
        }
        false
    }
}
