use crate::types::*;
use std::collections::{HashMap, HashSet, VecDeque};

/// Supply chain and production system
pub struct SupplyChainSystem {
    product_dependencies: HashMap<ResourceType, ProductDependency>,
    factory_types: HashMap<FactoryType, Vec<ResourceType>>,
}

impl SupplyChainSystem {
    pub fn new() -> Self {
        let mut system = Self {
            product_dependencies: HashMap::new(),
            factory_types: HashMap::new(),
        };

        system.initialize_product_dependencies();
        system.initialize_factory_types();
        system
    }

    /// Initialize the product dependency DAG
    fn initialize_product_dependencies(&mut self) {
        // Basic resources (no dependencies)
        self.product_dependencies.insert(
            ResourceType::Energy,
            ProductDependency {
                product: ResourceType::Energy,
                dependencies: Vec::new(),
                production_time: 5,
                energy_cost: 0,
            },
        );

        self.product_dependencies.insert(
            ResourceType::Minerals,
            ProductDependency {
                product: ResourceType::Minerals,
                dependencies: Vec::new(),
                production_time: 8,
                energy_cost: 0,
            },
        );

        self.product_dependencies.insert(
            ResourceType::Population,
            ProductDependency {
                product: ResourceType::Population,
                dependencies: Vec::new(),
                production_time: 10,
                energy_cost: 0,
            },
        );

        self.product_dependencies.insert(
            ResourceType::Technology,
            ProductDependency {
                product: ResourceType::Technology,
                dependencies: Vec::new(),
                production_time: 12,
                energy_cost: 0,
            },
        );

        self.product_dependencies.insert(
            ResourceType::Food,
            ProductDependency {
                product: ResourceType::Food,
                dependencies: Vec::new(),
                production_time: 6,
                energy_cost: 0,
            },
        );

        // Intermediate products
        self.product_dependencies.insert(
            ResourceType::Alloys,
            ProductDependency {
                product: ResourceType::Alloys,
                dependencies: vec![
                    ResourceAmount {
                        resource_type: ResourceType::Minerals,
                        amount: 100,
                    },
                    ResourceAmount {
                        resource_type: ResourceType::Energy,
                        amount: 50,
                    },
                ],
                production_time: 10,
                energy_cost: 25,
            },
        );

        self.product_dependencies.insert(
            ResourceType::Electronics,
            ProductDependency {
                product: ResourceType::Electronics,
                dependencies: vec![
                    ResourceAmount {
                        resource_type: ResourceType::Minerals,
                        amount: 50,
                    },
                    ResourceAmount {
                        resource_type: ResourceType::Technology,
                        amount: 30,
                    },
                    ResourceAmount {
                        resource_type: ResourceType::Energy,
                        amount: 40,
                    },
                ],
                production_time: 15,
                energy_cost: 30,
            },
        );

        self.product_dependencies.insert(
            ResourceType::Medicine,
            ProductDependency {
                product: ResourceType::Medicine,
                dependencies: vec![
                    ResourceAmount {
                        resource_type: ResourceType::Food,
                        amount: 20,
                    },
                    ResourceAmount {
                        resource_type: ResourceType::Technology,
                        amount: 25,
                    },
                    ResourceAmount {
                        resource_type: ResourceType::Energy,
                        amount: 15,
                    },
                ],
                production_time: 12,
                energy_cost: 20,
            },
        );

        // Advanced products
        self.product_dependencies.insert(
            ResourceType::Starships,
            ProductDependency {
                product: ResourceType::Starships,
                dependencies: vec![
                    ResourceAmount {
                        resource_type: ResourceType::Alloys,
                        amount: 200,
                    },
                    ResourceAmount {
                        resource_type: ResourceType::Electronics,
                        amount: 100,
                    },
                    ResourceAmount {
                        resource_type: ResourceType::Energy,
                        amount: 300,
                    },
                ],
                production_time: 50,
                energy_cost: 100,
            },
        );

        self.product_dependencies.insert(
            ResourceType::AdvancedWeapons,
            ProductDependency {
                product: ResourceType::AdvancedWeapons,
                dependencies: vec![
                    ResourceAmount {
                        resource_type: ResourceType::Alloys,
                        amount: 150,
                    },
                    ResourceAmount {
                        resource_type: ResourceType::Electronics,
                        amount: 80,
                    },
                    ResourceAmount {
                        resource_type: ResourceType::Technology,
                        amount: 100,
                    },
                    ResourceAmount {
                        resource_type: ResourceType::Energy,
                        amount: 200,
                    },
                ],
                production_time: 40,
                energy_cost: 80,
            },
        );

        self.product_dependencies.insert(
            ResourceType::AISystems,
            ProductDependency {
                product: ResourceType::AISystems,
                dependencies: vec![
                    ResourceAmount {
                        resource_type: ResourceType::Electronics,
                        amount: 200,
                    },
                    ResourceAmount {
                        resource_type: ResourceType::Technology,
                        amount: 300,
                    },
                    ResourceAmount {
                        resource_type: ResourceType::Energy,
                        amount: 500,
                    },
                ],
                production_time: 80,
                energy_cost: 150,
            },
        );

        // End-game products
        self.product_dependencies.insert(
            ResourceType::DysonSpheres,
            ProductDependency {
                product: ResourceType::DysonSpheres,
                dependencies: vec![
                    ResourceAmount {
                        resource_type: ResourceType::Alloys,
                        amount: 10000,
                    },
                    ResourceAmount {
                        resource_type: ResourceType::Electronics,
                        amount: 5000,
                    },
                    ResourceAmount {
                        resource_type: ResourceType::AISystems,
                        amount: 1000,
                    },
                    ResourceAmount {
                        resource_type: ResourceType::Energy,
                        amount: 50000,
                    },
                ],
                production_time: 1000,
                energy_cost: 2000,
            },
        );

        self.product_dependencies.insert(
            ResourceType::GalacticNetworks,
            ProductDependency {
                product: ResourceType::GalacticNetworks,
                dependencies: vec![
                    ResourceAmount {
                        resource_type: ResourceType::AISystems,
                        amount: 2000,
                    },
                    ResourceAmount {
                        resource_type: ResourceType::Electronics,
                        amount: 10000,
                    },
                    ResourceAmount {
                        resource_type: ResourceType::Technology,
                        amount: 5000,
                    },
                    ResourceAmount {
                        resource_type: ResourceType::Energy,
                        amount: 100000,
                    },
                ],
                production_time: 2000,
                energy_cost: 5000,
            },
        );
    }

    /// Initialize factory types and their capabilities
    fn initialize_factory_types(&mut self) {
        self.factory_types.insert(
            FactoryType::BasicManufacturing,
            vec![ResourceType::Alloys, ResourceType::Electronics],
        );

        self.factory_types.insert(
            FactoryType::AdvancedManufacturing,
            vec![
                ResourceType::Alloys,
                ResourceType::Electronics,
                ResourceType::Medicine,
            ],
        );

        self.factory_types
            .insert(FactoryType::Electronics, vec![ResourceType::Electronics]);

        self.factory_types
            .insert(FactoryType::Pharmaceuticals, vec![ResourceType::Medicine]);

        self.factory_types
            .insert(FactoryType::Shipyard, vec![ResourceType::Starships]);

        self.factory_types
            .insert(FactoryType::Weapons, vec![ResourceType::AdvancedWeapons]);

        self.factory_types
            .insert(FactoryType::Research, vec![ResourceType::AISystems]);

        self.factory_types
            .insert(FactoryType::Housing, vec![ResourceType::Population]);
    }

    /// Get all dependencies for a product (recursive)
    pub fn get_all_dependencies(&self, product: &ResourceType) -> Vec<ResourceAmount> {
        let mut all_dependencies = Vec::new();
        let mut visited = HashSet::new();
        self.collect_dependencies_recursive(product, &mut all_dependencies, &mut visited);
        all_dependencies
    }

    /// Recursively collect all dependencies
    fn collect_dependencies_recursive(
        &self,
        product: &ResourceType,
        dependencies: &mut Vec<ResourceAmount>,
        visited: &mut HashSet<ResourceType>,
    ) {
        if visited.contains(product) {
            return; // Prevent cycles
        }
        visited.insert(*product);

        if let Some(product_dep) = self.product_dependencies.get(product) {
            for dep in &product_dep.dependencies {
                dependencies.push(*dep);
                self.collect_dependencies_recursive(&dep.resource_type, dependencies, visited);
            }
        }
    }

    /// Check if a product can be produced with available resources
    pub fn can_produce_product(
        &self,
        product: &ResourceType,
        available_resources: &HashMap<ResourceType, u64>,
    ) -> bool {
        let dependencies = self.get_all_dependencies(product);

        for dep in dependencies {
            let available = available_resources
                .get(&dep.resource_type)
                .copied()
                .unwrap_or(0);
            if available < dep.amount {
                return false;
            }
        }
        true
    }

    /// Calculate the total cost to produce a product
    pub fn calculate_production_cost(&self, product: &ResourceType) -> HashMap<ResourceType, u64> {
        let mut total_cost = HashMap::new();
        let dependencies = self.get_all_dependencies(product);

        for dep in dependencies {
            *total_cost.entry(dep.resource_type).or_insert(0) += dep.amount;
        }

        total_cost
    }

    /// Get the production time for a product
    pub fn get_production_time(&self, product: &ResourceType) -> u64 {
        self.product_dependencies
            .get(product)
            .map(|dep| dep.production_time)
            .unwrap_or(0)
    }

    /// Get the energy cost for producing a product
    pub fn get_energy_cost(&self, product: &ResourceType) -> u64 {
        self.product_dependencies
            .get(product)
            .map(|dep| dep.energy_cost)
            .unwrap_or(0)
    }

    /// Check if a factory can produce a specific product
    pub fn can_factory_produce(&self, factory_type: &FactoryType, product: &ResourceType) -> bool {
        self.factory_types
            .get(factory_type)
            .map(|products| products.contains(product))
            .unwrap_or(false)
    }

    /// Get all products that a factory can produce
    pub fn get_factory_products(&self, factory_type: &FactoryType) -> Vec<ResourceType> {
        self.factory_types
            .get(factory_type)
            .cloned()
            .unwrap_or_default()
    }

    /// Get the optimal factory type for producing a product
    pub fn get_optimal_factory(&self, product: &ResourceType) -> Option<FactoryType> {
        for (factory_type, products) in &self.factory_types {
            if products.contains(product) {
                return Some(*factory_type);
            }
        }
        None
    }

    /// Calculate production efficiency based on planet modifiers
    pub fn calculate_production_efficiency(
        &self,
        planet: &Planet,
        factory_type: &FactoryType,
    ) -> f64 {
        let mut efficiency = 1.0;

        for modifier in &planet.modifiers {
            match modifier.modifier_type {
                ModifierType::ManufacturingBonus => {
                    efficiency *= 1.0 + modifier.value / 100.0;
                }
                ModifierType::ResearchBonus => {
                    if *factory_type == FactoryType::Research {
                        efficiency *= 1.0 + modifier.value / 100.0;
                    }
                }
                _ => {}
            }
        }

        efficiency
    }

    /// Update factory production
    pub fn update_factory_production(
        &self,
        factory: &mut Factory,
        planet: &Planet,
        game_speed: GameSpeed,
    ) -> Vec<ResourceAmount> {
        let mut produced_resources = Vec::new();
        let speed_multiplier = game_speed as u64;

        if !factory.is_active {
            return produced_resources;
        }

        if factory.production_queue.is_empty() {
            if let Some(products) = self.factory_types.get(&factory.factory_type) {
                for product in products {
                    let quantity = self.default_production_quantity(product);
                    factory.production_queue.push(ProductionOrder {
                        product: *product,
                        quantity,
                        priority: u8::MAX / 2,
                        progress: 0.0,
                    });
                }

                factory
                    .production_queue
                    .sort_by(|a, b| b.priority.cmp(&a.priority));
            }
        }

        for order in &mut factory.production_queue {
            if order.progress < 1.0 {
                let production_time = self.get_production_time(&order.product).max(1);
                let production_rate = 1.0 / production_time as f64;
                let efficiency =
                    self.calculate_production_efficiency(planet, &factory.factory_type);

                order.progress += production_rate * efficiency * speed_multiplier as f64;

                if order.progress >= 1.0 {
                    order.progress = 1.0;
                    produced_resources.push(ResourceAmount {
                        resource_type: order.product,
                        amount: order.quantity,
                    });
                }
            }
        }

        // Remove completed orders
        factory
            .production_queue
            .retain(|order| order.progress < 1.0);

        produced_resources
    }

    /// Determine default production quantity when auto-queuing products
    fn default_production_quantity(&self, product: &ResourceType) -> u64 {
        match product {
            ResourceType::Energy => 50,
            ResourceType::Minerals => 40,
            ResourceType::Food => 45,
            ResourceType::Population => 25,
            ResourceType::Technology => 20,
            ResourceType::Alloys => 15,
            ResourceType::Electronics => 15,
            ResourceType::Medicine => 15,
            ResourceType::Starships => 5,
            ResourceType::AdvancedWeapons => 5,
            ResourceType::AISystems => 5,
            ResourceType::DysonSpheres => 1,
            ResourceType::GalacticNetworks => 1,
        }
    }

    /// Add a production order to a factory
    pub fn add_production_order(
        &self,
        factory: &mut Factory,
        product: ResourceType,
        quantity: u64,
        priority: u8,
    ) -> bool {
        // Check if factory can produce this product
        if !self.can_factory_produce(&factory.factory_type, &product) {
            return false;
        }

        let order = ProductionOrder {
            product,
            quantity,
            priority,
            progress: 0.0,
        };

        factory.production_queue.push(order);

        // Sort by priority (higher priority first)
        factory
            .production_queue
            .sort_by(|a, b| b.priority.cmp(&a.priority));

        true
    }

    /// Get the production dependency graph as a topological sort
    pub fn get_production_order(&self) -> Vec<ResourceType> {
        let mut in_degree = HashMap::new();
        let mut graph = HashMap::new();

        // Initialize in-degree and graph
        for (product, _dep) in &self.product_dependencies {
            in_degree.insert(*product, 0);
            graph.insert(*product, Vec::new());
        }

        // Build graph and calculate in-degrees
        for (product, dep) in &self.product_dependencies {
            for dependency in &dep.dependencies {
                if let Some(children) = graph.get_mut(&dependency.resource_type) {
                    children.push(*product);
                }
                *in_degree.get_mut(product).unwrap() += 1;
            }
        }

        // Topological sort
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        // Add nodes with no dependencies
        for (product, degree) in &in_degree {
            if *degree == 0 {
                queue.push_back(*product);
            }
        }

        while let Some(current) = queue.pop_front() {
            result.push(current);

            if let Some(children) = graph.get(&current) {
                for &child in children {
                    if let Some(degree) = in_degree.get_mut(&child) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(child);
                        }
                    }
                }
            }
        }

        result
    }
}
