use crate::types::*;
use std::collections::HashMap;

/// Prestige system for galaxy progression
pub struct PrestigeSystem {
    prestige_multipliers: HashMap<PrestigeBonusType, f64>,
}

impl PrestigeSystem {
    pub fn new() -> Self {
        let mut prestige_multipliers = HashMap::new();

        // Initialize base prestige multipliers
        prestige_multipliers.insert(PrestigeBonusType::ResourceMultiplier, 1.1);
        prestige_multipliers.insert(PrestigeBonusType::ResearchSpeed, 1.2);
        prestige_multipliers.insert(PrestigeBonusType::ConquestSpeed, 1.15);
        prestige_multipliers.insert(PrestigeBonusType::TerraformingSpeed, 1.25);
        prestige_multipliers.insert(PrestigeBonusType::TransportEfficiency, 1.1);
        prestige_multipliers.insert(PrestigeBonusType::FactoryEfficiency, 1.15);
        prestige_multipliers.insert(PrestigeBonusType::StartingResources, 1.5);
        prestige_multipliers.insert(PrestigeBonusType::GalaxyModifier, 1.0);

        Self {
            prestige_multipliers,
        }
    }

    /// Calculate prestige points earned from conquering a galaxy
    pub fn calculate_galaxy_prestige_points(
        &self,
        galaxy: &Galaxy,
        conquest_time: u64,
        efficiency_score: f64,
    ) -> u64 {
        let base_points = 1000; // Base points per galaxy
        let time_bonus = self.calculate_time_bonus(conquest_time);
        let efficiency_bonus = (efficiency_score * 0.5) as u64;

        // Galaxy modifiers affect prestige points
        let galaxy_bonus = galaxy
            .galaxy_modifiers
            .iter()
            .map(|modifier| match modifier.modifier_type {
                ModifierType::ResearchBonus => modifier.value as u64,
                ModifierType::MineralMultiplier => (modifier.value * 0.5) as u64,
                ModifierType::EnergyMultiplier => (modifier.value * 0.3) as u64,
                _ => 0,
            })
            .sum::<u64>();

        base_points + time_bonus + efficiency_bonus + galaxy_bonus
    }

    /// Calculate time bonus for fast conquest
    fn calculate_time_bonus(&self, conquest_time: u64) -> u64 {
        // Faster conquest gives more prestige points
        match conquest_time {
            0..=1000 => 500,     // Very fast conquest
            1001..=5000 => 300,  // Fast conquest
            5001..=10000 => 150, // Normal conquest
            _ => 50,             // Slow conquest
        }
    }

    /// Calculate efficiency score based on resource usage
    pub fn calculate_efficiency_score(
        &self,
        total_resources_used: &HashMap<ResourceType, u64>,
        total_resources_generated: &HashMap<ResourceType, u64>,
    ) -> f64 {
        let mut efficiency_sum = 0.0;
        let mut resource_count = 0;

        for (resource_type, used) in total_resources_used {
            if let Some(generated) = total_resources_generated.get(resource_type) {
                if *generated > 0 {
                    let efficiency = (*used as f64) / (*generated as f64);
                    efficiency_sum += efficiency.min(1.0); // Cap at 100% efficiency
                    resource_count += 1;
                }
            }
        }

        if resource_count > 0 {
            efficiency_sum / resource_count as f64
        } else {
            0.0
        }
    }

    /// Create prestige bonuses based on galaxy conquest
    pub fn create_prestige_bonuses(
        &self,
        prestige_points: u64,
        galaxy_modifiers: &[Modifier],
    ) -> Vec<PrestigeBonus> {
        let mut bonuses = Vec::new();

        // Base bonuses that always apply
        bonuses.push(PrestigeBonus {
            bonus_type: PrestigeBonusType::ResourceMultiplier,
            value: 1.0 + (prestige_points as f64 * 0.001), // 0.1% per prestige point
            is_percentage: true,
        });

        bonuses.push(PrestigeBonus {
            bonus_type: PrestigeBonusType::ResearchSpeed,
            value: 1.0 + (prestige_points as f64 * 0.0005), // 0.05% per prestige point
            is_percentage: true,
        });

        // Galaxy-specific bonuses
        for modifier in galaxy_modifiers {
            let bonus_type = match modifier.modifier_type {
                ModifierType::ResearchBonus => PrestigeBonusType::ResearchSpeed,
                ModifierType::MineralMultiplier => PrestigeBonusType::ResourceMultiplier,
                ModifierType::EnergyMultiplier => PrestigeBonusType::ResourceMultiplier,
                ModifierType::TechnologyMultiplier => PrestigeBonusType::ResearchSpeed,
                ModifierType::DefensiveBonus => PrestigeBonusType::ConquestSpeed,
                ModifierType::TradeBonus => PrestigeBonusType::TransportEfficiency,
                ModifierType::ManufacturingBonus => PrestigeBonusType::FactoryEfficiency,
                _ => continue,
            };

            bonuses.push(PrestigeBonus {
                bonus_type,
                value: 1.0 + (modifier.value * 0.01), // Convert percentage to multiplier
                is_percentage: true,
            });
        }

        // Special bonuses for high prestige
        if prestige_points >= 10000 {
            bonuses.push(PrestigeBonus {
                bonus_type: PrestigeBonusType::StartingResources,
                value: 2.0, // Double starting resources
                is_percentage: true,
            });
        }

        if prestige_points >= 50000 {
            bonuses.push(PrestigeBonus {
                bonus_type: PrestigeBonusType::GalaxyModifier,
                value: 1.5, // Galaxy-wide bonuses
                is_percentage: true,
            });
        }

        bonuses
    }

    /// Apply prestige bonuses to game state
    pub fn apply_prestige_bonuses(&self, game_state: &mut GameState, bonuses: &[PrestigeBonus]) {
        for bonus in bonuses {
            match bonus.bonus_type {
                PrestigeBonusType::ResourceMultiplier => {
                    // Apply to all resource generation
                    // This would be applied during resource generation calculations
                }
                PrestigeBonusType::ResearchSpeed => {
                    // Apply to research and technology generation
                    // This would be applied during technology calculations
                }
                PrestigeBonusType::ConquestSpeed => {
                    // Apply to conquest time calculations
                    // This would be applied during conquest calculations
                }
                PrestigeBonusType::TerraformingSpeed => {
                    // Apply to terraforming project duration
                    // This would be applied during terraforming calculations
                }
                PrestigeBonusType::TransportEfficiency => {
                    // Apply to transport route efficiency
                    // This would be applied during transport calculations
                }
                PrestigeBonusType::FactoryEfficiency => {
                    // Apply to factory production efficiency
                    // This would be applied during factory calculations
                }
                PrestigeBonusType::StartingResources => {
                    // Apply to initial empire resources
                    self.apply_starting_resource_bonus(game_state, bonus);
                }
                PrestigeBonusType::GalaxyModifier => {
                    // Apply galaxy-wide bonuses
                    // This would be applied to all planets in the galaxy
                }
            }
        }
    }

    /// Apply starting resource bonus
    fn apply_starting_resource_bonus(&self, game_state: &mut GameState, bonus: &PrestigeBonus) {
        for (_resource_type, amount) in game_state.empire_resources.iter_mut() {
            *amount = (*amount as f64 * bonus.value) as u64;
        }
    }

    /// Calculate prestige requirements for next galaxy
    pub fn calculate_prestige_requirements(&self, current_prestige: u64) -> u64 {
        // Exponential scaling for prestige requirements
        let base_requirement = 1000;
        let scaling_factor = 1.5_f64;

        (base_requirement as f64 * scaling_factor.powf(current_prestige as f64 / 10000.0)) as u64
    }

    /// Check if player can prestige to next galaxy
    pub fn can_prestige(&self, current_prestige: u64, galaxy_conquest_progress: f64) -> bool {
        let required_prestige = self.calculate_prestige_requirements(current_prestige);
        current_prestige >= required_prestige && galaxy_conquest_progress >= 0.8
        // 80% galaxy conquest required
    }

    /// Get prestige bonus value for a specific type
    pub fn get_prestige_bonus_value(
        &self,
        bonuses: &[PrestigeBonus],
        bonus_type: PrestigeBonusType,
    ) -> f64 {
        bonuses
            .iter()
            .filter(|bonus| bonus.bonus_type == bonus_type)
            .map(|bonus| bonus.value)
            .fold(1.0, |acc, value| acc * value)
    }

    /// Calculate total prestige multiplier
    pub fn calculate_total_prestige_multiplier(&self, bonuses: &[PrestigeBonus]) -> f64 {
        bonuses
            .iter()
            .map(|bonus| bonus.value)
            .fold(1.0, |acc, value| acc * value)
    }

    /// Get prestige statistics
    pub fn get_prestige_statistics(
        &self,
        total_prestige: u64,
        bonuses: &[PrestigeBonus],
    ) -> PrestigeStatistics {
        let total_multiplier = self.calculate_total_prestige_multiplier(bonuses);
        let next_requirement = self.calculate_prestige_requirements(total_prestige);

        PrestigeStatistics {
            total_prestige,
            total_multiplier,
            next_requirement,
            bonus_count: bonuses.len(),
        }
    }
}

/// Prestige statistics for display
#[derive(Debug, Clone)]
pub struct PrestigeStatistics {
    pub total_prestige: u64,
    pub total_multiplier: f64,
    pub next_requirement: u64,
    pub bonus_count: usize,
}
