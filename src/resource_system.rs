use crate::types::*;
use std::collections::HashMap;

/// Resource generation and management system
pub struct ResourceSystem {
    resource_storage_limits: HashMap<ResourceType, u64>,
}

impl ResourceSystem {
    pub fn new() -> Self {
        let mut resource_storage_limits = HashMap::new();

        // Set storage limits
        resource_storage_limits.insert(ResourceType::Energy, 10000);
        resource_storage_limits.insert(ResourceType::Minerals, 10000);
        resource_storage_limits.insert(ResourceType::Population, 0);
        resource_storage_limits.insert(ResourceType::Technology, 0);
        resource_storage_limits.insert(ResourceType::Food, 8000);

        Self {
            resource_storage_limits,
        }
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
