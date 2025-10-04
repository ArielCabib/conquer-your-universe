use crate::game_engine::GameStatistics;
use crate::types::*;
use std::collections::HashMap;
use yew::prelude::*;

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
                        html! {
                            <span class={format!("modifier {}", if modifier.value > 0.0 { "positive" } else { "negative" })}>
                                { format!("{:?}: {:.1}%", modifier.modifier_type, modifier.value) }
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
    pub on_close: Callback<()>,
    pub on_terraform: Callback<(u64, ModifierType)>,
    pub on_add_factory: Callback<(u64, FactoryType)>,
}

#[function_component]
pub fn PlanetPanel(props: &PlanetPanelProps) -> Html {
    if let Some(planet) = &props.planet {
        let planet_id = planet.id;
        let on_close = props.on_close.clone();
        let on_terraform = props.on_terraform.clone();
        let on_add_factory = props.on_add_factory.clone();

        html! {
            <div class="planet-panel">
                <div class="panel-header">
                    <h3>{ &planet.name }</h3>
                    <button onclick={move |_| on_close.emit(())} class="close-btn">{"×"}</button>
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
                                    html! {
                                        <div class={format!("modifier-item {}", if modifier.value > 0.0 { "positive" } else { "negative" })}>
                                            <span class="modifier-name">{ format!("{:?}", modifier.modifier_type) }</span>
                                            <span class="modifier-value">{ format!("{:.1}%", modifier.value) }</span>
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
        html! {}
    }
}

/// Resource dashboard component
#[derive(Properties, PartialEq, Clone)]
pub struct ResourceDashboardProps {
    pub empire_resources: HashMap<ResourceType, u64>,
    pub resource_generation: HashMap<ResourceType, u64>,
}

#[function_component]
pub fn ResourceDashboard(props: &ResourceDashboardProps) -> Html {
    html! {
        <div class="resource-dashboard">
            <h3>{ "Empire Resources" }</h3>
            <div class="resource-grid">
                { for props.empire_resources.iter().map(|(resource_type, amount)| {
                    let generation = props.resource_generation.get(resource_type).copied().unwrap_or(0);
                    html! {
                        <div class="resource-card">
                            <div class="resource-header">
                                <h4>{ format!("{:?}", resource_type) }</h4>
                            </div>
                            <div class="resource-amount">{ *amount }</div>
                            <div class="resource-generation">
                                { format!("+{}/tick", generation) }
                            </div>
                        </div>
                    }
                }) }
            </div>
        </div>
    }
}

/// Game statistics component
#[derive(Properties, PartialEq, Clone)]
pub struct GameStatsProps {
    pub stats: GameStatistics,
}

#[function_component]
pub fn GameStats(props: &GameStatsProps) -> Html {
    let stats = &props.stats;

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
                <div class="stat-item">
                    <span class="stat-label">{ "Prestige Points" }</span>
                    <span class="stat-value">{ stats.prestige_points }</span>
                </div>
                <div class="stat-item">
                    <span class="stat-label">{ "Game Speed" }</span>
                    <span class="stat-value">{ format!("{}x", stats.game_speed as u64) }</span>
                </div>
                <div class="stat-item">
                    <span class="stat-label">{ "Status" }</span>
                    <span class="stat-value">{ if stats.is_paused { "Paused" } else { "Running" } }</span>
                </div>
            </div>
        </div>
    }
}
