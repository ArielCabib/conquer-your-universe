use crate::game_engine::GameStatistics;
use crate::types::*;
use std::collections::{HashMap, HashSet};
use web_sys::KeyboardEvent;
use yew::prelude::*;

fn format_modifier_value(modifier: &Modifier) -> String {
    let value = modifier.value;
    let suffix = if modifier.is_percentage { "%" } else { "" };
    if value.fract().abs() < 1e-6 {
        format!("{:+.0}{}", value, suffix)
    } else {
        format!("{:+.1}{}", value, suffix)
    }
}

fn modifier_polarity_class(value: f64) -> &'static str {
    if value > 0.0 {
        "positive"
    } else if value < 0.0 {
        "negative"
    } else {
        "neutral"
    }
}

/// Galaxy map component showing solar systems and planets
#[derive(Properties, PartialEq, Clone)]
pub struct GalaxyMapProps {
    pub galaxies: HashMap<u64, Galaxy>,
    pub solar_systems: HashMap<u64, SolarSystem>,
    pub planets: HashMap<u64, Planet>,
    pub current_galaxy: u64,
    pub on_planet_click: Callback<u64>,
}

#[function_component]
pub fn GalaxyMap(props: &GalaxyMapProps) -> Html {
    let current_galaxy = props.galaxies.get(&props.current_galaxy);

    html! {
        <div class="galaxy-map">
            <h2>{ "Galaxy Map" }</h2>
            { if let Some(galaxy) = current_galaxy {
                html! {
                    <div class="galaxy-content">
                        <div class="galaxy-info">
                            <h3>{ &galaxy.name }</h3>
                            <p>{ format!("Solar Systems: {}", galaxy.solar_systems.len()) }</p>
                            <p>{ format!("Conquered: {}", galaxy.is_conquered) }</p>
                            <div class="instructions">
                                <p><strong>{ "How to Play:" }</strong></p>
                                <p>{ "- Click on planets (small circles) to conquer them" }</p>
                                <p>{ "- Check the right panel for conquest costs" }</p>
                                <p>{ "- Build your empire by conquering more planets!" }</p>
                            </div>
                        </div>
                        <div class="solar-systems">
                            <p>{ format!("Found {} solar systems", galaxy.solar_systems.len()) }</p>
                            { for galaxy.solar_systems.iter().map(|&system_id| {
                                if let Some(system) = props.solar_systems.get(&system_id) {
                                    html! {
                                        <SolarSystemView
                                            system={system.clone()}
                                            planets={props.planets.clone()}
                                            on_planet_click={props.on_planet_click.clone()}
                                        />
                                    }
                                } else {
                                    html! { <p>{ format!("System {} not found", system_id) }</p> }
                                }
                            }) }
                        </div>
                    </div>
                }
            } else {
                html! { <p>{ "No galaxy loaded" }</p> }
            }}
        </div>
    }
}

/// HTML Grid-based galaxy renderer
#[derive(Properties, PartialEq, Clone)]
pub struct GalaxyGridProps {
    pub galaxies: HashMap<u64, Galaxy>,
    pub solar_systems: HashMap<u64, SolarSystem>,
    pub planets: HashMap<u64, Planet>,
    pub current_galaxy: u64,
    pub discovered_solar_systems: HashSet<u64>,
    pub explored_solar_systems: HashSet<u64>,
    pub on_system_click: Callback<u64>,
}

/// Solar System Grid Props
#[derive(Properties, PartialEq, Clone)]
pub struct SolarSystemGridProps {
    pub solar_system: SolarSystem,
    pub planets: HashMap<u64, Planet>,
    pub on_planet_click: Callback<u64>,
}

/// Planet Detail Grid Props
#[derive(Properties, PartialEq, Clone)]
pub struct PlanetDetailGridProps {
    pub planet: Planet,
    pub empire_resources: HashMap<ResourceType, u64>,
    pub on_terraform: Callback<(u64, ModifierType)>,
    pub on_add_factory: Callback<(u64, FactoryType)>,
}

#[function_component]
pub fn GalaxyGrid(props: &GalaxyGridProps) -> Html {
    // Get the current galaxy
    let galaxy = props.galaxies.get(&props.current_galaxy);

    if galaxy.is_none() {
        return html! {
            <div class="galaxy-grid">
                <div class="no-galaxy">
                    { "No galaxy found" }
                </div>
            </div>
        };
    }

    let galaxy = galaxy.unwrap();

    // Only show discovered/explored solar systems
    let visible_systems: Vec<_> = galaxy
        .solar_systems
        .iter()
        .filter(|system_id| {
            props.discovered_solar_systems.contains(system_id)
                || props.explored_solar_systems.contains(system_id)
        })
        .collect();

    // Create a flexible grid layout instead of fixed 10x10
    let systems_per_row = 4; // Show 4 systems per row
    let total_rows = (visible_systems.len() + systems_per_row - 1) / systems_per_row;

    html! {
        <div class="galaxy-grid">
            <div class="systems-container">
                { if visible_systems.is_empty() {
                    html! {
                        <div class="no-systems">
                            <h3>{ "No Solar Systems Discovered" }</h3>
                            <p>{ "Explore the galaxy to discover new solar systems!" }</p>
                        </div>
                    }
                } else {
                    html! {
                        { for (0..total_rows).map(|row| {
                            let start_idx = row * systems_per_row;
                            let end_idx = (start_idx + systems_per_row).min(visible_systems.len());
                            let row_systems = &visible_systems[start_idx..end_idx];

                            html! {
                                <div class="systems-row" key={row}>
                                    { for row_systems.iter().map(|system_id| {
                                        if let Some(system) = props.solar_systems.get(system_id) {
                                            let is_discovered = props.discovered_solar_systems.contains(system_id);
                                            let is_explored = props.explored_solar_systems.contains(system_id);
                                            let on_system_click = props.on_system_click.clone();
                                            let system_id_clone = **system_id;

                                            html! {
                                                <div
                                                    class={format!("solar-system-cell {}",
                                                        if is_explored { "explored" }
                                                        else if is_discovered { "discovered" }
                                                        else { "hidden" }
                                                    )}
                                                    onclick={on_system_click.reform(move |_| system_id_clone)}
                                                    title={format!("Click to view {} system", system.name)}
                                                >
                                                    <div class="system-header">
                                                        <h4>{ &system.name }</h4>
                                                        <p class="planet-count">{ format!("{} planets", system.planets.len()) }</p>
                                                    </div>
                                                    <div class="system-info">
                                                        { if is_explored {
                                                            html! { <p class="system-status explored">{ "Explored" }</p> }
                                                        } else if is_discovered {
                                                            html! { <p class="system-status discovered">{ "Discovered" }</p> }
                                                        } else {
                                                            html! { <p class="system-status hidden">{ "Unknown" }</p> }
                                                        }}
                                                    </div>
                                                </div>
                                            }
                                        } else {
                                            html! { <div class="system-error">{ "System not found" }</div> }
                                        }
                                    })}
                                </div>
                            }
                        })}
                    }
                }}
            </div>
        </div>
    }
}

/// Solar System Grid Component
#[function_component]
pub fn SolarSystemGrid(props: &SolarSystemGridProps) -> Html {
    let system = &props.solar_system;

    // Only log once per unique system to avoid noise
    static mut LAST_LOGGED_SYSTEM: u64 = 0;
    unsafe {
        if LAST_LOGGED_SYSTEM != system.id {
            log::info!(
                "Solar system {} has {} planets: {:?}",
                system.id,
                system.planets.len(),
                system.planets
            );
            LAST_LOGGED_SYSTEM = system.id;
        }
    }

    html! {
        <div class="solar-system-grid">
            <div class="system-header">
                <h2>{ &system.name }</h2>
                <p class="system-info">{ format!("{} planets in this system", system.planets.len()) }</p>
            </div>
            <div class="planets-grid-container">
                { for system.planets.iter().map(|planet_id_ref| {
                    let planet_id = *planet_id_ref;
                    let on_planet_click = props.on_planet_click.clone();

                    html! {
                        <div class="planet-cell" key={planet_id}>
                            { if let Some(planet) = props.planets.get(&planet_id) {
                                let planet_class = format!("{:?}", planet.class).to_lowercase().replace("_", "-");
                                let planet_state = format!("{:?}", planet.state).to_lowercase();

                                html! {
                                    <div
                                        class={format!("planet-card {} {}", planet_class, planet_state)}
                                        onclick={on_planet_click.reform(move |_| planet_id)}
                                        title={format!("Click to view {} details", planet.name)}
                                    >
                                        <div class="planet-header">
                                            <h4>{ &planet.name }</h4>
                                            <span class="planet-class">{ format!("{:?}", planet.class) }</span>
                                        </div>
                                        <div class="planet-status">
                                            <span class="status-badge">{ format!("{:?}", planet.state) }</span>
                                        </div>
                                        <div class="planet-resources">
                                            { for planet.resources.iter().take(3).map(|(resource_type, amount)| {
                                                html! {
                                                    <div class="resource-item">
                                                        <span class="resource-name">{ format!("{:?}", resource_type) }</span>
                                                        <span class="resource-amount">{ *amount }</span>
                                                    </div>
                                                }
                                            })}
                                        </div>
                                    </div>
                                }
                            } else {
                                html! { <div class="planet-error">{ "Planet not found" }</div> }
                            }}
                        </div>
                    }
                })}
            </div>
        </div>
    }
}

/// Planet Detail Grid Component
#[function_component]
pub fn PlanetDetailGrid(props: &PlanetDetailGridProps) -> Html {
    let planet = &props.planet;

    html! {
        <div class="planet-detail-grid">
            <div class="planet-header">
                <h2>{ &planet.name }</h2>
                <span class="planet-class-badge">{ format!("{:?}", planet.class) }</span>
                <span class="planet-state-badge">{ format!("{:?}", planet.state) }</span>
            </div>

            <div class="planet-info-grid">
                <div class="info-section">
                    <h3>{ "Resources" }</h3>
                    <div class="resources-grid">
                        { for planet.resources.iter().map(|(resource_type, amount)| {
                            html! {
                                <div class="resource-card">
                                    <span class="resource-name">{ format!("{:?}", resource_type) }</span>
                                    <span class="resource-amount">{ *amount }</span>
                                </div>
                            }
                        })}
                    </div>
                </div>

                <div class="info-section">
                    <h3>{ "Modifiers" }</h3>
                    <div class="modifiers-grid">
                        { for planet.modifiers.iter().map(|modifier| {
                            let formatted_value = format_modifier_value(modifier);
                            let polarity_class = modifier_polarity_class(modifier.value);
                            html! {
                                <div class="modifier-card">
                                    <span class="modifier-type">{ format!("{:?}", modifier.modifier_type) }</span>
                                    <span class={classes!("modifier-value", polarity_class)}>{ formatted_value }</span>
                                </div>
                            }
                        })}
                    </div>
                </div>

                <div class="info-section">
                    <h3>{ "Factories" }</h3>
                    <div class="factories-grid">
                        { for planet.factories.iter().map(|factory| {
                            html! {
                                <div class="factory-card">
                                    <span class="factory-type">{ format!("{:?}", factory.factory_type) }</span>
                                    <span class="factory-level">{ format!("Level {}", factory.efficiency) }</span>
                                </div>
                            }
                        })}
                    </div>
                </div>

                <div class="info-section">
                    <h3>{ "Terraforming Projects" }</h3>
                    <div class="terraforming-grid">
                        { for planet.terraforming_projects.iter().map(|project| {
                            html! {
                                <div class="terraforming-card">
                                    <span class="project-type">{ format!("{:?}", project.target_modifier) }</span>
                                    <span class="project-progress">{ format!("{:.1}%", project.progress * 100.0) }</span>
                                </div>
                            }
                        })}
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Solar system view component
#[derive(Properties, PartialEq, Clone)]
pub struct SolarSystemViewProps {
    pub system: SolarSystem,
    pub planets: HashMap<u64, Planet>,
    pub on_planet_click: Callback<u64>,
}

#[function_component]
pub fn SolarSystemView(props: &SolarSystemViewProps) -> Html {
    let system = &props.system;
    let conquered_planets = system
        .planets
        .iter()
        .filter(|&planet_id| {
            props
                .planets
                .get(planet_id)
                .map(|planet| planet.state == PlanetState::Conquered)
                .unwrap_or(false)
        })
        .count();

    let total_planets = system.planets.len();
    let conquest_progress = if total_planets > 0 {
        conquered_planets as f64 / total_planets as f64
    } else {
        0.0
    };

    html! {
        <div class="solar-system" style={format!("left: {}px; top: {}px;", system.position.0, system.position.1)}>
            <div class="system-header">
                <h4>{ &system.name }</h4>
                <div class="conquest-progress">
                    <div class="progress-bar">
                        <div class="progress-fill" style={format!("width: {}%", conquest_progress * 100.0)}></div>
                    </div>
                    <span>{ format!("{}/{}", conquered_planets, total_planets) }</span>
                </div>
            </div>
            <div class="planets">
                { for system.planets.iter().map(|&planet_id| {
                    if let Some(planet) = props.planets.get(&planet_id) {
                        html! {
                            <PlanetView
                                planet={planet.clone()}
                                on_click={props.on_planet_click.clone()}
                            />
                        }
                    } else {
                        html! {}
                    }
                }) }
            </div>
        </div>
    }
}

/// Planet view component
#[derive(Properties, PartialEq, Clone)]
pub struct PlanetViewProps {
    pub planet: Planet,
    pub on_click: Callback<u64>,
}

#[function_component]
pub fn PlanetView(props: &PlanetViewProps) -> Html {
    let planet = &props.planet;
    let onclick = {
        let planet_id = planet.id;
        let on_click = props.on_click.clone();
        move |_| on_click.emit(planet_id)
    };

    let planet_class = match planet.class {
        PlanetClass::Barren => "barren",
        PlanetClass::Terran => "terran",
        PlanetClass::GasGiant => "gas-giant",
        PlanetClass::Ocean => "ocean",
        PlanetClass::Desert => "desert",
        PlanetClass::Ice => "ice",
        PlanetClass::Volcanic => "volcanic",
        PlanetClass::Toxic => "toxic",
        PlanetClass::Crystalline => "crystalline",
        PlanetClass::Metallic => "metallic",
    };

    let state_class = match planet.state {
        PlanetState::Unexplored => "unexplored",
        PlanetState::Explored => "explored",
        PlanetState::Conquered => "conquered",
        PlanetState::Terraforming => "terraforming",
    };

    html! {
        <div
            class={format!("planet {} {}", planet_class, state_class)}
            {onclick}
            style={format!("left: {}px; top: {}px;", planet.position.0, planet.position.1)}
        >
            <div class="planet-info">
                <h5>{ &planet.name }</h5>
                <div class="planet-resources">
                    { for planet.resources.iter().map(|(resource_type, amount)| {
                        html! {
                            <span class="resource">
                                { format!("{:?}: {}", resource_type, amount) }
                            </span>
                        }
                    }) }
                </div>
                <div class="planet-modifiers">
                    { for planet.modifiers.iter().map(|modifier| {
                        let formatted_value = format_modifier_value(modifier);
                        let polarity_class = modifier_polarity_class(modifier.value);
                        html! {
                            <span class={classes!("modifier", polarity_class)}>
                                { format!("{:?}: {}", modifier.modifier_type, formatted_value) }
                            </span>
                        }
                    }) }
                </div>
            </div>
        </div>
    }
}

/// Planet management panel
#[derive(Properties, PartialEq, Clone)]
pub struct PlanetPanelProps {
    pub planet: Option<Planet>,
    pub on_terraform: Callback<(u64, ModifierType)>,
    pub on_add_factory: Callback<(u64, FactoryType)>,
}

#[function_component]
pub fn PlanetPanel(props: &PlanetPanelProps) -> Html {
    if let Some(planet) = &props.planet {
        let planet_id = planet.id;
        let on_terraform = props.on_terraform.clone();
        let on_add_factory = props.on_add_factory.clone();

        html! {
            <div class="planet-panel">
                <div class="panel-header">
                    <h3>{ &planet.name }</h3>
                </div>

                <div class="planet-status-hint">
                    { match planet.state {
                        PlanetState::Conquered => html! {
                            <div class="status-hint conquered">
                                <strong>{ "Conquered:" }</strong> { "You can now build factories and terraform this planet." }
                            </div>
                        },
                        PlanetState::Explored => html! {
                            <div class="status-hint explored">
                                <strong>{ "Explored:" }</strong> { "This planet has been explored but not yet conquered." }
                            </div>
                        },
                        PlanetState::Terraforming => html! {
                            <div class="status-hint terraforming">
                                <strong>{ "Terraforming:" }</strong> { "This planet is currently being terraformed." }
                            </div>
                        },
                        PlanetState::Unexplored => html! { <></> },
                    }}
                </div>

                <div class="panel-content">
                    <div class="planet-details">
                        <div class="detail-section">
                            <h4>{ "Planet Class" }</h4>
                            <p>{ format!("{:?}", planet.class) }</p>
                        </div>

                        <div class="detail-section">
                            <h4>{ "State" }</h4>
                            <p>{ format!("{:?}", planet.state) }</p>
                        </div>

                        <div class="detail-section">
                            <h4>{ "Resources" }</h4>
                            <div class="resource-list">
                                { for planet.resources.iter().map(|(resource_type, amount)| {
                                    html! {
                                        <div class="resource-item">
                                            <span class="resource-name">{ format!("{:?}", resource_type) }</span>
                                            <span class="resource-amount">{ *amount }</span>
                                        </div>
                                    }
                                }) }
                            </div>
                        </div>

                        <div class="detail-section">
                            <h4>{ "Modifiers" }</h4>
                            <div class="modifier-list">
                                { for planet.modifiers.iter().map(|modifier| {
                                    let formatted_value = format_modifier_value(modifier);
                                    let polarity_class = modifier_polarity_class(modifier.value);
                                    html! {
                                        <div class={classes!("modifier-item", polarity_class)}>
                                            <span class="modifier-name">{ format!("{:?}", modifier.modifier_type) }</span>
                                            <span class={classes!("modifier-value", polarity_class)}>{ formatted_value }</span>
                                        </div>
                                    }
                                }) }
                            </div>
                        </div>
                    </div>

                    { if planet.state == PlanetState::Conquered {
                        html! {
                            <div class="planet-actions">
                                <div class="action-section">
                                    <h4>{ "Terraforming Projects" }</h4>
                                    <div class="terraforming-projects">
                                        { for planet.terraforming_projects.iter().map(|project| {
                                            html! {
                                                <div class="terraforming-project">
                                                    <div class="project-name">{ &project.name }</div>
                                                    <div class="project-progress">
                                                        <div class="progress-bar">
                                                            <div class="progress-fill" style={format!("width: {}%", project.progress * 100.0)}></div>
                                                        </div>
                                                        <span>{ format!("{:.1}%", project.progress * 100.0) }</span>
                                                    </div>
                                                </div>
                                            }
                                        }) }
                                    </div>
                                    <button onclick={move |_| on_terraform.emit((planet_id, ModifierType::EnergyMultiplier))} class="action-btn">
                                        { "Start Terraforming" }
                                    </button>
                                </div>

                                <div class="action-section">
                                    <h4>{ "Factories" }</h4>
                                    <div class="factories">
                                        { for planet.factories.iter().map(|factory| {
                                            html! {
                                                <div class="factory">
                                                    <div class="factory-type">{ format!("{:?}", factory.factory_type) }</div>
                                                    <div class="factory-status">{ if factory.is_active { "Active" } else { "Inactive" } }</div>
                                                </div>
                                            }
                                        }) }
                                    </div>
                                    <button onclick={move |_| on_add_factory.emit((planet_id, FactoryType::BasicManufacturing))} class="action-btn">
                                        { "Add Factory" }
                                    </button>
                                </div>
                            </div>
                        }
                    } else {
                        html! {}
                    }}
                </div>
            </div>
        }
    } else {
        html! {
            <div class="planet-panel">
                <div class="panel-header">
                    <h3>{ "No Planet Selected" }</h3>
                </div>
                <div class="panel-content">
                    <p>{ "Please select a planet to view its details." }</p>
                </div>
            </div>
        }
    }
}

/// Resource dashboard component
#[derive(Properties, PartialEq, Clone)]
pub struct ResourceDashboardProps {
    pub empire_resources: HashMap<ResourceType, u64>,
    pub resource_generation: HashMap<ResourceType, u64>,
    pub storage_limits: HashMap<ResourceType, u64>,
}

#[function_component]
pub fn ResourceDashboard(props: &ResourceDashboardProps) -> Html {
    html! {
        <div class="resource-dashboard">
            <h3>{ "Empire Resources" }</h3>
            <div class="resource-grid">
                { for props.empire_resources.iter().map(|(resource_type, amount)| {
                    let generation = props.resource_generation.get(resource_type).copied().unwrap_or(0);
                    let storage_limit = props.storage_limits.get(resource_type).copied().unwrap_or(1000);
                    let is_at_capacity = *amount >= storage_limit;
                    let capacity_percentage = (*amount as f64 / storage_limit as f64 * 100.0).min(100.0);

                    html! {
                        <div class={format!("resource-card {}", if is_at_capacity { "at-capacity" } else { "" })}>
                            <div>
                                <div class="resource-header">
                                    <h4>{ format!("{:?}", resource_type) }</h4>
                                    { if is_at_capacity {
                                        html! { <span class="capacity-warning">{ "FULL" }</span> }
                                    } else {
                                        html! {}
                                    }}
                                </div>
                                <div class="resource-amount">
                                    { format!("{} / {}", *amount, storage_limit) }
                                </div>
                            </div>
                            <div>
                                <div class="resource-capacity-bar">
                                    <div class="capacity-fill" style={format!("width: {:.1}%", capacity_percentage)}></div>
                                </div>
                                <div class="resource-generation">
                                    { format!("+{}/sec", generation) }
                                </div>
                            </div>
                        </div>
                    }
                }) }
            </div>
        </div>
    }
}

/// Factory management component
#[derive(Properties, PartialEq, Clone)]
pub struct FactoryManagementProps {
    pub planet: Option<Planet>,
    pub empire_resources: HashMap<ResourceType, u64>,
    pub on_add_factory: Callback<(u64, FactoryType)>,
}

// Static factory costs to prevent re-calculation
lazy_static::lazy_static! {
    static ref FACTORY_COSTS: HashMap<FactoryType, HashMap<ResourceType, u64>> = get_factory_costs();
}

#[function_component]
pub fn FactoryManagement(props: &FactoryManagementProps) -> Html {
    if let Some(planet) = &props.planet {
        if planet.state == PlanetState::Conquered {
            html! {
                <div class="factory-management">
                    <h4>{ "Factory Management" }</h4>

                    <div class="existing-factories">
                        <h5>{ "Existing Factories" }</h5>
                        { if planet.factories.is_empty() {
                            html! { <p class="no-factories">{ "No factories built yet." }</p> }
                        } else {
                            html! {
                                <div class="factory-list">
                                    { for planet.factories.iter().map(|factory| {
                                        html! {
                                            <div class="factory-item">
                                                <div class="factory-info">
                                                    <span class="factory-type">{ format!("{:?}", factory.factory_type) }</span>
                                                    <span class="factory-status">{ if factory.is_active { "Active" } else { "Inactive" } }</span>
                                                </div>
                                                <div class="factory-efficiency">
                                                    <span>{ format!("Efficiency: {:.1}%", factory.efficiency * 100.0) }</span>
                                                </div>
                                            </div>
                                        }
                                    }) }
                                </div>
                            }
                        }}
                    </div>

                    <div class="build-factories">
                        <h5>{ "Build New Factory" }</h5>
                        <div class="factory-options">
                            { for FACTORY_COSTS.iter().map(|(factory_type, cost)| {
                                let can_afford = cost.iter().all(|(resource_type, required)| {
                                    props.empire_resources.get(resource_type).copied().unwrap_or(0) >= *required
                                });

                                html! {
                                    <div class={format!("factory-option {}", if can_afford { "affordable" } else { "insufficient" })}>
                                        <div class="factory-header">
                                            <h6>{ format!("{:?}", factory_type) }</h6>
                                            <button
                                                class="build-btn"
                                                disabled={!can_afford}
                                                onclick={
                                                    let on_add_factory = props.on_add_factory.clone();
                                                    let planet_id = planet.id;
                                                    let factory_type = *factory_type;
                                                    move |_| on_add_factory.emit((planet_id, factory_type))
                                                }
                                            >
                                                { if can_afford { "Build" } else { "Insufficient Resources" } }
                                            </button>
                                        </div>
                                        <div class="factory-cost">
                                            <h6>{ "Cost:" }</h6>
                                            <div class="cost-list">
                                                { for cost.iter().map(|(resource_type, amount)| {
                                                    let available = props.empire_resources.get(resource_type).copied().unwrap_or(0);
                                                    let can_afford_resource = available >= *amount;
                                                    html! {
                                                        <div class={format!("cost-item {}", if can_afford_resource { "affordable" } else { "insufficient" })}>
                                                            <span class="resource-name">{ format!("{:?}", resource_type) }</span>
                                                            <span class="cost-amount">{ format!("{}/{}", amount, available) }</span>
                                                        </div>
                                                    }
                                                }) }
                                            </div>
                                        </div>
                                    </div>
                                }
                            }) }
                        </div>
                    </div>
                </div>
            }
        } else {
            html! {}
        }
    } else {
        html! {}
    }
}

/// Get factory costs for different factory types
fn get_factory_costs() -> HashMap<FactoryType, HashMap<ResourceType, u64>> {
    let mut costs = HashMap::new();

    // Basic Manufacturing
    let mut basic_manufacturing = HashMap::new();
    basic_manufacturing.insert(ResourceType::Energy, 200);
    basic_manufacturing.insert(ResourceType::Minerals, 100);
    basic_manufacturing.insert(ResourceType::Population, 50);
    costs.insert(FactoryType::BasicManufacturing, basic_manufacturing);

    // Housing
    let mut housing = HashMap::new();
    housing.insert(ResourceType::Energy, 150);
    housing.insert(ResourceType::Minerals, 75);
    housing.insert(ResourceType::Food, 100);
    costs.insert(FactoryType::Housing, housing);

    // Advanced Manufacturing
    let mut advanced_manufacturing = HashMap::new();
    advanced_manufacturing.insert(ResourceType::Energy, 500);
    advanced_manufacturing.insert(ResourceType::Minerals, 300);
    advanced_manufacturing.insert(ResourceType::Population, 100);
    advanced_manufacturing.insert(ResourceType::Technology, 50);
    costs.insert(FactoryType::AdvancedManufacturing, advanced_manufacturing);

    // Electronics
    let mut electronics = HashMap::new();
    electronics.insert(ResourceType::Energy, 300);
    electronics.insert(ResourceType::Minerals, 200);
    electronics.insert(ResourceType::Population, 75);
    electronics.insert(ResourceType::Technology, 100);
    costs.insert(FactoryType::Electronics, electronics);

    // Pharmaceuticals
    let mut pharmaceuticals = HashMap::new();
    pharmaceuticals.insert(ResourceType::Energy, 250);
    pharmaceuticals.insert(ResourceType::Minerals, 150);
    pharmaceuticals.insert(ResourceType::Population, 60);
    pharmaceuticals.insert(ResourceType::Technology, 80);
    costs.insert(FactoryType::Pharmaceuticals, pharmaceuticals);

    // Research Facility
    let mut research_facility = HashMap::new();
    research_facility.insert(ResourceType::Energy, 300);
    research_facility.insert(ResourceType::Minerals, 200);
    research_facility.insert(ResourceType::Population, 100);
    research_facility.insert(ResourceType::Technology, 100);
    costs.insert(FactoryType::Research, research_facility);

    // Shipyard
    let mut shipyard = HashMap::new();
    shipyard.insert(ResourceType::Energy, 800);
    shipyard.insert(ResourceType::Minerals, 600);
    shipyard.insert(ResourceType::Population, 200);
    shipyard.insert(ResourceType::Technology, 150);
    costs.insert(FactoryType::Shipyard, shipyard);

    // Weapons
    let mut weapons = HashMap::new();
    weapons.insert(ResourceType::Energy, 400);
    weapons.insert(ResourceType::Minerals, 500);
    weapons.insert(ResourceType::Population, 150);
    weapons.insert(ResourceType::Technology, 200);
    costs.insert(FactoryType::Weapons, weapons);

    costs
}

/// Transport system component
#[derive(Properties, PartialEq, Clone)]
pub struct TransportSystemProps {
    pub planets: HashMap<u64, Planet>,
    pub empire_resources: HashMap<ResourceType, u64>,
    pub on_start_transport: Callback<(u64, u64, ResourceType, u64)>, // from_planet, to_planet, resource_type, amount
}

#[function_component]
pub fn TransportSystem(props: &TransportSystemProps) -> Html {
    let from_planet = use_state(|| None::<u64>);
    let to_planet = use_state(|| None::<u64>);
    let selected_resource = use_state(|| ResourceType::Energy);
    let transport_amount = use_state(|| 100u64);

    let conquered_planets: Vec<&Planet> = props
        .planets
        .values()
        .filter(|planet| planet.state == PlanetState::Conquered)
        .collect();

    let available_resources = if let Some(planet_id) = *from_planet {
        props
            .planets
            .get(&planet_id)
            .map(|planet| planet.resources.clone())
            .unwrap_or_default()
    } else {
        props.empire_resources.clone()
    };

    let transport_cost = calculate_transport_cost(
        *from_planet,
        *to_planet,
        &props.planets,
        *selected_resource,
        *transport_amount,
    );

    let can_afford_transport = transport_cost.iter().all(|(resource_type, cost)| {
        props
            .empire_resources
            .get(resource_type)
            .copied()
            .unwrap_or(0)
            >= *cost
    });

    let has_resource = available_resources
        .get(&*selected_resource)
        .copied()
        .unwrap_or(0)
        >= *transport_amount;

    html! {
        <div class="transport-system">
            <h4>{ "Transport System" }</h4>

            <div class="transport-setup">
                <div class="transport-row">
                    <label>{ "From Planet:" }</label>
                    <select
                        value={from_planet.map(|id| id.to_string()).unwrap_or_default()}
                        onchange={
                            let from_planet = from_planet.clone();
                            move |e: Event| {
                                let target = e.target_dyn_into::<web_sys::HtmlElement>().unwrap();
                                let value = target.get_attribute("value").unwrap_or_default();
                                if value.is_empty() {
                                    from_planet.set(None);
                                } else {
                                    from_planet.set(value.parse().ok());
                                }
                            }
                        }
                    >
                        <option value="">{ "Select Source Planet" }</option>
                        { for conquered_planets.iter().map(|planet| {
                            html! {
                                <option value={planet.id.to_string()}>{ &planet.name }</option>
                            }
                        }) }
                    </select>
                </div>

                <div class="transport-row">
                    <label>{ "To Planet:" }</label>
                    <select
                        value={to_planet.map(|id| id.to_string()).unwrap_or_default()}
                        onchange={
                            let to_planet = to_planet.clone();
                            move |e: Event| {
                                let target = e.target_dyn_into::<web_sys::HtmlElement>().unwrap();
                                let value = target.get_attribute("value").unwrap_or_default();
                                if value.is_empty() {
                                    to_planet.set(None);
                                } else {
                                    to_planet.set(value.parse().ok());
                                }
                            }
                        }
                    >
                        <option value="">{ "Select Destination Planet" }</option>
                        { for conquered_planets.iter().map(|planet| {
                            html! {
                                <option value={planet.id.to_string()}>{ &planet.name }</option>
                            }
                        }) }
                    </select>
                </div>

                <div class="transport-row">
                    <label>{ "Resource:" }</label>
                    <select
                        value={format!("{:?}", *selected_resource)}
                        onchange={
                            let selected_resource = selected_resource.clone();
                            move |e: Event| {
                                let target = e.target_dyn_into::<web_sys::HtmlElement>().unwrap();
                                let value = target.get_attribute("value").unwrap_or_default();
                                // Simple mapping for resource types
                                let resource = match value.as_str() {
                                    "Energy" => ResourceType::Energy,
                                    "Minerals" => ResourceType::Minerals,
                                    "Population" => ResourceType::Population,
                                    "Technology" => ResourceType::Technology,
                                    "Food" => ResourceType::Food,
                                    _ => ResourceType::Energy,
                                };
                                selected_resource.set(resource);
                            }
                        }
                    >
                        { for [ResourceType::Energy, ResourceType::Minerals, ResourceType::Population, ResourceType::Technology, ResourceType::Food].iter().map(|resource_type| {
                            html! {
                                <option value={format!("{:?}", resource_type)}>{ format!("{:?}", resource_type) }</option>
                            }
                        }) }
                    </select>
                </div>

                <div class="transport-row">
                    <label>{ "Amount:" }</label>
                    <input
                        type="number"
                        value={transport_amount.to_string()}
                        min="1"
                        max={available_resources.get(&*selected_resource).copied().unwrap_or(0).to_string()}
                        onchange={
                            let transport_amount = transport_amount.clone();
                            move |e: Event| {
                                let target = e.target_dyn_into::<web_sys::HtmlElement>().unwrap();
                                let value = target.get_attribute("value").unwrap_or_default();
                                if let Ok(amount) = value.parse::<u64>() {
                                    transport_amount.set(amount);
                                }
                            }
                        }
                    />
                    <span class="available-resources">
                        { format!("Available: {}", available_resources.get(&*selected_resource).copied().unwrap_or(0)) }
                    </span>
                </div>
            </div>

            <div class="transport-cost">
                <h5>{ "Transport Cost" }</h5>
                <div class="cost-list">
                    { for transport_cost.iter().map(|(resource_type, cost)| {
                        let available = props.empire_resources.get(resource_type).copied().unwrap_or(0);
                        let can_afford_resource = available >= *cost;
                        html! {
                            <div class={format!("cost-item {}", if can_afford_resource { "affordable" } else { "insufficient" })}>
                                <span class="resource-name">{ format!("{:?}", resource_type) }</span>
                                <span class="cost-amount">{ format!("{}/{}", cost, available) }</span>
                            </div>
                        }
                    }) }
                </div>
            </div>

            <div class="transport-actions">
                <button
                    class="transport-btn"
                    disabled={!can_afford_transport || !has_resource || from_planet.is_none() || to_planet.is_none() || from_planet == to_planet}
                    onclick={
                        let on_start_transport = props.on_start_transport.clone();
                        let from_planet = *from_planet;
                        let to_planet = *to_planet;
                        let selected_resource = *selected_resource;
                        let transport_amount = *transport_amount;
                        move |_| {
                            if let (Some(from), Some(to)) = (from_planet, to_planet) {
                                on_start_transport.emit((from, to, selected_resource, transport_amount));
                            }
                        }
                    }
                >
                    { if from_planet == to_planet { "Same Planet" }
                      else if !has_resource { "Insufficient Resource" }
                      else if !can_afford_transport { "Insufficient Transport Cost" }
                      else { "Start Transport" } }
                </button>
            </div>
        </div>
    }
}

/// Calculate transport cost between planets
fn calculate_transport_cost(
    from_planet: Option<u64>,
    to_planet: Option<u64>,
    planets: &HashMap<u64, Planet>,
    resource_type: ResourceType,
    amount: u64,
) -> HashMap<ResourceType, u64> {
    let mut cost = HashMap::new();

    if let (Some(from_id), Some(to_id)) = (from_planet, to_planet) {
        if let (Some(from), Some(to)) = (planets.get(&from_id), planets.get(&to_id)) {
            // Calculate distance
            let distance = ((from.position.0 - to.position.0).powi(2)
                + (from.position.1 - to.position.1).powi(2))
            .sqrt();

            // Base transport cost based on distance and amount
            let base_cost = (distance * 0.1 + amount as f64 * 0.05) as u64;

            // Energy cost for transport
            cost.insert(ResourceType::Energy, base_cost);

            // Additional costs based on resource type
            match resource_type {
                ResourceType::Technology => {
                    cost.insert(ResourceType::Population, base_cost / 2);
                }
                ResourceType::Population => {
                    cost.insert(ResourceType::Energy, base_cost * 2);
                }
                _ => {}
            }
        }
    }

    cost
}

/// Prestige system component
#[derive(Properties, PartialEq, Clone)]
pub struct PrestigeSystemProps {
    pub current_prestige: u64,
    pub galaxy_conquest_progress: f64,
    pub can_prestige: bool,
    pub prestige_requirements: u64,
    pub on_perform_prestige: Callback<()>,
}

#[function_component]
pub fn PrestigeSystem(props: &PrestigeSystemProps) -> Html {
    let progress_percentage = (props.galaxy_conquest_progress * 100.0) as u32;

    html! {
        <div class="prestige-system">
            <h4>{ "Prestige System" }</h4>

            <div class="prestige-status">
                <div class="prestige-info">
                    <div class="prestige-points">
                        <span class="label">{ "Current Prestige:" }</span>
                        <span class="value">{ props.current_prestige }</span>
                    </div>

                    <div class="galaxy-progress">
                        <span class="label">{ "Galaxy Progress:" }</span>
                        <div class="progress-container">
                            <div class="progress-bar">
                                <div
                                    class="progress-fill"
                                    style={format!("width: {}%", progress_percentage)}
                                ></div>
                            </div>
                            <span class="progress-text">{ format!("{:.1}%", props.galaxy_conquest_progress * 100.0) }</span>
                        </div>
                    </div>
                </div>

                <div class="prestige-requirements">
                    <h5>{ "Prestige Requirements" }</h5>
                    <div class="requirement-item">
                        <span class="requirement-label">{ "Galaxy Conquest:" }</span>
                        <span class={format!("requirement-status {}", if props.galaxy_conquest_progress >= 0.8 { "met" } else { "not-met" })}>
                            { if props.galaxy_conquest_progress >= 0.8 {
                                "✓ 80% Complete".to_string()
                            } else {
                                format!("✗ {:.1}% Complete", props.galaxy_conquest_progress * 100.0)
                            } }
                        </span>
                    </div>
                    <div class="requirement-item">
                        <span class="requirement-label">{ "Prestige Points:" }</span>
                        <span class={format!("requirement-status {}", if props.current_prestige >= props.prestige_requirements { "met" } else { "not-met" })}>
                            { if props.current_prestige >= props.prestige_requirements {
                                format!("✓ {} Points", props.current_prestige)
                            } else {
                                format!("✗ {}/{} Points", props.current_prestige, props.prestige_requirements)
                            } }
                        </span>
                    </div>
                </div>

                <div class="prestige-benefits">
                    <h5>{ "Prestige Benefits" }</h5>
                    <div class="benefit-list">
                        <div class="benefit-item">
                            <span class="benefit-icon">{ "🚀" }</span>
                            <span class="benefit-text">{ "Start new galaxy with permanent bonuses" }</span>
                        </div>
                        <div class="benefit-item">
                            <span class="benefit-icon">{ "⚡" }</span>
                            <span class="benefit-text">{ "Increased resource generation rates" }</span>
                        </div>
                        <div class="benefit-item">
                            <span class="benefit-icon">{ "🏭" }</span>
                            <span class="benefit-text">{ "Factory efficiency bonuses" }</span>
                        </div>
                        <div class="benefit-item">
                            <span class="benefit-icon">{ "🌍" }</span>
                            <span class="benefit-text">{ "Faster planet conquest" }</span>
                        </div>
                    </div>
                </div>

                <div class="prestige-actions">
                    <button
                        class={format!("prestige-btn {}", if props.can_prestige { "available" } else { "unavailable" })}
                        disabled={!props.can_prestige}
                        onclick={
                            let on_perform_prestige = props.on_perform_prestige.clone();
                            move |_| on_perform_prestige.emit(())
                        }
                    >
                        { if props.can_prestige { "🌟 Prestige to New Galaxy" } else { "Requirements Not Met" } }
                    </button>

                    { if !props.can_prestige {
                        html! {
                            <div class="prestige-hint">
                                <p>{ "Complete 80% of current galaxy and accumulate prestige points to prestige." }</p>
                            </div>
                        }
                    } else {
                        html! {}
                    }}
                </div>
            </div>
        </div>
    }
}

/// Terraforming project component
#[derive(Properties, PartialEq, Clone)]
pub struct TerraformingProjectProps {
    pub planet: Option<Planet>,
    pub empire_resources: HashMap<ResourceType, u64>,
    pub on_start_terraforming: Callback<(u64, ModifierType)>,
}

#[function_component]
pub fn TerraformingProject(props: &TerraformingProjectProps) -> Html {
    if let Some(planet) = &props.planet {
        if planet.state == PlanetState::Conquered {
            // Get negative modifiers that can be terraformed
            let negative_modifiers: Vec<&Modifier> = planet
                .modifiers
                .iter()
                .filter(|modifier| modifier.value < 0.0)
                .collect();

            html! {
                <div class="terraforming-project">
                    <h4>{ "Terraforming Projects" }</h4>

                    <div class="active-projects">
                        <h5>{ "Active Projects" }</h5>
                        { if planet.terraforming_projects.is_empty() {
                            html! { <p class="no-projects">{ "No active terraforming projects." }</p> }
                        } else {
                            html! {
                                <div class="project-list">
                                    { for planet.terraforming_projects.iter().map(|project| {
                                        html! {
                                            <div class="project-item">
                                                <div class="project-header">
                                                    <span class="project-name">{ &project.name }</span>
                                                    <span class="project-progress">{ format!("{:.1}%", project.progress * 100.0) }</span>
                                                </div>
                                                <div class="project-details">
                                                    <div class="progress-bar">
                                                        <div
                                                            class="progress-fill"
                                                            style={format!("width: {}%", project.progress * 100.0)}
                                                        ></div>
                                                    </div>
                                                    <div class="project-target">
                                                        { format!("Target: {:?}", project.target_modifier) }
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                    }) }
                                </div>
                            }
                        }}
                    </div>

                    <div class="available-terraforming">
                        <h5>{ "Available Terraforming" }</h5>
                        { if negative_modifiers.is_empty() {
                            html! { <p class="no-modifiers">{ "No negative modifiers to terraform." }</p> }
                        } else {
                            html! {
                                <div class="modifier-list">
                                    { for negative_modifiers.iter().map(|modifier| {
                                        let mut terraforming_cost: Vec<_> = calculate_terraforming_cost(modifier.modifier_type)
                                            .into_iter()
                                            .collect();
                                        terraforming_cost.sort_by_key(|(resource_type, _)| resource_display_order(*resource_type));

                                        let can_afford = terraforming_cost.iter().all(|(resource_type, cost)| {
                                            props.empire_resources.get(resource_type).copied().unwrap_or(0) >= *cost
                                        });
                                        let formatted_value = format_modifier_value(modifier);
                                        let polarity_class = modifier_polarity_class(modifier.value);

                                        html! {
                                            <div class={format!("modifier-option {}", if can_afford { "affordable" } else { "insufficient" })}>
                                                <div class="modifier-header">
                                                    <span class="modifier-name">{ format!("{:?}", modifier.modifier_type) }</span>
                                                    <span class={classes!("modifier-value", polarity_class)}>{ formatted_value }</span>
                                                </div>
                                                <div class="modifier-cost">
                                                    <h6>{ "Terraforming Cost:" }</h6>
                                                    <div class="cost-list">
                                                        { for terraforming_cost.iter().map(|(resource_type, cost)| {
                                                            let available = props.empire_resources.get(resource_type).copied().unwrap_or(0);
                                                            let can_afford_resource = available >= *cost;
                                                            html! {
                                                                <div class={format!("cost-item {}", if can_afford_resource { "affordable" } else { "insufficient" })}>
                                                                    <span class="resource-name">{ format!("{:?}", resource_type) }</span>
                                                                    <span class="cost-amount">{ format!("{}/{}", cost, available) }</span>
                                                                </div>
                                                            }
                                                        }) }
                                                    </div>
                                                </div>
                                                <button
                                                    class="terraform-btn"
                                                    disabled={!can_afford}
                                                    onclick={
                                                        let on_start_terraforming = props.on_start_terraforming.clone();
                                                        let planet_id = planet.id;
                                                        let modifier_type = modifier.modifier_type;
                                                        move |_| on_start_terraforming.emit((planet_id, modifier_type))
                                                    }
                                                >
                                                    { if can_afford { "Start Terraforming" } else { "Insufficient Resources" } }
                                                </button>
                                            </div>
                                        }
                                    }) }
                                </div>
                            }
                        }}
                    </div>
                </div>
            }
        } else {
            html! {}
        }
    } else {
        html! {}
    }
}

/// Calculate terraforming cost for a modifier type
fn calculate_terraforming_cost(modifier_type: ModifierType) -> HashMap<ResourceType, u64> {
    let mut cost = HashMap::new();

    // Base terraforming cost
    cost.insert(ResourceType::Energy, 500);
    cost.insert(ResourceType::Minerals, 300);
    cost.insert(ResourceType::Population, 100);
    cost.insert(ResourceType::Technology, 150);

    // Additional costs based on modifier type
    match modifier_type {
        ModifierType::ResourcePenalty => {
            cost.insert(ResourceType::Energy, 400);
            cost.insert(ResourceType::Minerals, 200);
        }
        ModifierType::DefensiveBonus => {
            cost.insert(ResourceType::Population, 150);
            cost.insert(ResourceType::Technology, 100);
        }
        _ => {
            // Default cost for other modifiers
            cost.insert(ResourceType::Energy, 300);
            cost.insert(ResourceType::Minerals, 250);
        }
    }

    cost
}

fn resource_display_order(resource_type: ResourceType) -> usize {
    match resource_type {
        ResourceType::Energy => 0,
        ResourceType::Minerals => 1,
        ResourceType::Population => 2,
        ResourceType::Technology => 3,
        ResourceType::Food => 4,
        ResourceType::Alloys => 5,
        ResourceType::Electronics => 6,
        ResourceType::Medicine => 7,
        ResourceType::Starships => 8,
        ResourceType::AdvancedWeapons => 9,
        ResourceType::AISystems => 10,
        ResourceType::DysonSpheres => 11,
        ResourceType::GalacticNetworks => 12,
    }
}

/// Conquest cost display component
#[derive(Properties, PartialEq, Clone)]
pub struct ConquestCostProps {
    pub planet: Option<Planet>,
    pub empire_resources: HashMap<ResourceType, u64>,
}

#[function_component]
pub fn ConquestCost(props: &ConquestCostProps) -> Html {
    if let Some(planet) = &props.planet {
        if planet.state == PlanetState::Unexplored {
            // Calculate conquest cost (simplified version)
            let mut cost = HashMap::new();
            cost.insert(ResourceType::Energy, 100);
            cost.insert(ResourceType::Minerals, 50);
            cost.insert(ResourceType::Population, 25);

            let can_afford = cost.iter().all(|(resource_type, required)| {
                props
                    .empire_resources
                    .get(resource_type)
                    .copied()
                    .unwrap_or(0)
                    >= *required
            });

            html! {
                <div class="conquest-cost">
                    <h4>{ "Conquest Cost" }</h4>
                    <div class="cost-list">
                        {{
                            let mut ordered_cost: Vec<_> = cost.iter().collect();
                            ordered_cost.sort_by_key(|(resource_type, _)| resource_display_order(**resource_type));

                            ordered_cost
                                .into_iter()
                                .map(|(resource_type, amount)| {
                                    let available =
                                        props.empire_resources.get(resource_type).copied().unwrap_or(0);
                                    let can_afford_resource = available >= *amount;
                                    html! {
                                        <div class={format!("cost-item {}", if can_afford_resource { "affordable" } else { "insufficient" })}>
                                            <span class="resource-name">{ format!("{:?}", resource_type) }</span>
                                            <span class="cost-amount">{ format!("{}/{}", amount, available) }</span>
                                        </div>
                                    }
                                })
                                .collect::<Html>()
                        }}
                    </div>
                    <div class={format!("conquest-status {}", if can_afford { "ready" } else { "insufficient" })}>
                        { if can_afford { "Ready to Conquer!" } else { "Insufficient Resources" } }
                    </div>
                </div>
            }
        } else {
            html! {}
        }
    } else {
        html! {}
    }
}

/// Game statistics component
#[derive(Properties, PartialEq, Clone)]
pub struct GameStatsProps {
    pub stats: GameStatistics,
    pub planet_count: usize,
    pub on_prestige_card_click: Callback<()>,
    pub on_speed_card_click: Callback<()>,
    pub on_status_card_click: Callback<()>,
}

#[function_component]
pub fn GameStats(props: &GameStatsProps) -> Html {
    let stats = &props.stats;
    let prestige_card_click = props.on_prestige_card_click.clone();
    let prestige_card_keyboard = prestige_card_click.clone();
    let speed_card_click = props.on_speed_card_click.clone();
    let speed_card_keyboard = speed_card_click.clone();
    let status_card_click = props.on_status_card_click.clone();
    let status_card_keyboard = status_card_click.clone();

    html! {
        <div class="game-stats">
            <h3>{ "Game Statistics" }</h3>
            <div class="stats-grid">
                <div class="stat-item">
                    <span class="stat-label">{ "Current Tick" }</span>
                    <span class="stat-value">{ stats.current_tick }</span>
                </div>
                <div class="stat-item">
                    <span class="stat-label">{ "Conquered Planets" }</span>
                    <span class="stat-value">{ format!("{}/{}", stats.conquered_planets, stats.total_planets) }</span>
                </div>
                <div class="stat-item">
                    <span class="stat-label">{ "Total Factories" }</span>
                    <span class="stat-value">{ stats.total_factories }</span>
                </div>
                <div class="stat-item">
                    <span class="stat-label">{ "Total Resources" }</span>
                    <span class="stat-value">{ stats.total_resources }</span>
                </div>
                <div
                    class={classes!("stat-item", "clickable", "prestige-card")}
                    role="button"
                    tabindex={0}
                    onclick={Callback::from(move |_| prestige_card_click.emit(()))}
                    onkeydown={Callback::from(move |event: KeyboardEvent| {
                        let key = event.key();
                        if key == "Enter" || key == " " {
                            event.prevent_default();
                            prestige_card_keyboard.emit(());
                        }
                    })}
                    aria-label="View prestige system"
                >
                    <span class="stat-label">{ "Prestige Points" }</span>
                    <span class="stat-value">{ stats.prestige_points }</span>
                </div>
                <div
                    class={classes!("stat-item", "clickable", "speed-card")}
                    role="button"
                    tabindex={0}
                    onclick={Callback::from(move |_| speed_card_click.emit(()))}
                    onkeydown={Callback::from(move |event: KeyboardEvent| {
                        let key = event.key();
                        if key == "Enter" || key == " " {
                            event.prevent_default();
                            speed_card_keyboard.emit(());
                        }
                    })}
                    aria-label="Adjust game speed"
                >
                    <span class="stat-label">{ "Game Speed" }</span>
                    <span class="stat-value">{ format!("{}x", stats.game_speed as u64) }</span>
                </div>
                <div
                    class={classes!("stat-item", "clickable", "status-card")}
                    role="button"
                    tabindex={0}
                    onclick={Callback::from(move |_| status_card_click.emit(()))}
                    onkeydown={Callback::from(move |event: KeyboardEvent| {
                        let key = event.key();
                        if key == "Enter" || key == " " {
                            event.prevent_default();
                            status_card_keyboard.emit(());
                        }
                    })}
                    aria-label="Manage game status and speed"
                >
                    <span class="stat-label">{ "Status" }</span>
                    <span class="stat-value">{ if stats.is_paused { "Paused" } else { "Running" } }</span>
                </div>
                <div class="stat-item">
                    <span class="stat-label">{ "Total Planets" }</span>
                    <span class="stat-value">{ props.planet_count }</span>
                </div>
            </div>
        </div>
    }
}
