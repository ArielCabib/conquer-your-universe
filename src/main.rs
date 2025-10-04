use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use yew::prelude::*;

// Import all our game modules
mod components;
mod galaxy_system;
mod game_engine;
mod planet_system;
mod prestige_system;
mod resource_system;
mod supply_chain;
mod transport_system;
mod types;

use components::*;
use game_engine::GameEngine;
use types::*;

#[function_component]
fn App() -> Html {
    let game_engine = use_state(|| {
        let mut engine = GameEngine::new();
        engine.initialize_game();
        Rc::new(RefCell::new(engine))
    });

    let game_tick = use_state(|| 0);
    let game_speed = use_state(|| GameSpeed::Normal);
    let is_paused = use_state(|| false);
    let current_view = use_state(|| ViewMode::Galaxy);
    let selected_solar_system = use_state(|| None::<u64>);
    let selected_planet = use_state(|| None::<Planet>);

    // Game update loop
    let game_engine_clone = game_engine.clone();
    let game_tick_clone = game_tick.clone();
    let is_paused_clone = is_paused.clone();

    use_effect(move || {
        let interval = gloo_timers::callback::Interval::new(100, move || {
            if !*is_paused_clone {
                game_engine_clone.borrow_mut().update();
                game_tick_clone.set(*game_tick_clone + 1);
            }
        });

        move || drop(interval)
    });

    let speed_controls = {
        let game_engine_1x = game_engine.clone();
        let game_engine_10x = game_engine.clone();
        let game_engine_100x = game_engine.clone();
        let game_engine_1000x = game_engine.clone();
        let game_speed_1x = game_speed.clone();
        let game_speed_10x = game_speed.clone();
        let game_speed_100x = game_speed.clone();
        let game_speed_1000x = game_speed.clone();

        html! {
            <div class="speed-controls">
                <button onclick={move |_| {
                    game_engine_1x.borrow_mut().set_game_speed(GameSpeed::Normal);
                    game_speed_1x.set(GameSpeed::Normal);
                }} class={if *game_speed == GameSpeed::Normal { "active" } else { "" }}>
                    { "1x" }
                </button>
                <button onclick={move |_| {
                    game_engine_10x.borrow_mut().set_game_speed(GameSpeed::Fast);
                    game_speed_10x.set(GameSpeed::Fast);
                }} class={if *game_speed == GameSpeed::Fast { "active" } else { "" }}>
                    { "10x" }
                </button>
                <button onclick={move |_| {
                    game_engine_100x.borrow_mut().set_game_speed(GameSpeed::VeryFast);
                    game_speed_100x.set(GameSpeed::VeryFast);
                }} class={if *game_speed == GameSpeed::VeryFast { "active" } else { "" }}>
                    { "100x" }
                </button>
                <button onclick={move |_| {
                    game_engine_1000x.borrow_mut().set_game_speed(GameSpeed::UltraFast);
                    game_speed_1000x.set(GameSpeed::UltraFast);
                }} class={if *game_speed == GameSpeed::UltraFast { "active" } else { "" }}>
                    { "1000x" }
                </button>
            </div>
        }
    };

    let pause_button = {
        let game_engine = game_engine.clone();
        let is_paused_clone = is_paused.clone();

        html! {
            <button onclick={move |_| {
                game_engine.borrow_mut().toggle_pause();
                is_paused_clone.set(!*is_paused_clone);
            }}>
                { if *is_paused { "Resume" } else { "Pause" } }
            </button>
        }
    };

    let selected_planet = use_state(|| None::<Planet>);
    let game_stats = {
        let stats = game_engine.borrow().get_game_statistics();
        let planet_count = game_engine.borrow().get_planet_count();
        html! {
            <>
                <GameStats stats={stats} />
                <div class="debug-info">
                    <p>{ format!("Total Planets: {}", planet_count) }</p>
                </div>
            </>
        }
    };

    let empire_resources = {
        let resources = game_engine.borrow().game_state.empire_resources.clone();
        let generation = game_engine
            .borrow()
            .resource_system
            .calculate_empire_resource_generation(
                &game_engine.borrow().game_state.planets,
                &game_engine
                    .borrow()
                    .game_state
                    .planets
                    .values()
                    .filter(|planet| planet.state == PlanetState::Conquered)
                    .map(|planet| planet.id)
                    .collect::<Vec<_>>(),
            );
        html! {
            <ResourceDashboard
                empire_resources={resources}
                resource_generation={generation}
            />
        }
    };

    // Navigation callbacks
    let on_system_click = {
        let current_view = current_view.clone();
        let selected_solar_system = selected_solar_system.clone();
        move |system_id: u64| {
            selected_solar_system.set(Some(system_id));
            current_view.set(ViewMode::SolarSystem);
        }
    };

    let on_planet_click = {
        let current_view = current_view.clone();
        let selected_planet = selected_planet.clone();
        let planets = game_engine.borrow().game_state.planets.clone();
        move |planet_id: u64| {
            if let Some(planet) = planets.get(&planet_id) {
                selected_planet.set(Some(planet.clone()));
                current_view.set(ViewMode::Planet);
            }
        }
    };

    let on_back_to_galaxy = {
        let current_view = current_view.clone();
        move |_| {
            current_view.set(ViewMode::Galaxy);
        }
    };

    let on_back_to_system = {
        let current_view = current_view.clone();
        move |_| {
            current_view.set(ViewMode::SolarSystem);
        }
    };

    // Main content based on current view
    let main_content = match *current_view {
        ViewMode::Galaxy => {
            let galaxies = game_engine.borrow().game_state.galaxies.clone();
            let solar_systems = game_engine.borrow().game_state.solar_systems.clone();
            let planets = game_engine.borrow().game_state.planets.clone();
            let current_galaxy = game_engine.borrow().game_state.current_galaxy;

            html! {
                <GalaxyGrid
                    galaxies={galaxies}
                    solar_systems={solar_systems}
                    planets={planets}
                    current_galaxy={current_galaxy}
                    discovered_solar_systems={game_engine.borrow().get_discovered_solar_systems().clone()}
                    explored_solar_systems={game_engine.borrow().get_explored_solar_systems().clone()}
                    on_system_click={Callback::from(on_system_click)}
                />
            }
        }
        ViewMode::SolarSystem => {
            if let Some(system_id) = *selected_solar_system {
                if let Some(system) = game_engine
                    .borrow()
                    .game_state
                    .solar_systems
                    .get(&system_id)
                {
                    let planets = game_engine.borrow().game_state.planets.clone();
                    html! {
                        <div class="solar-system-view">
                            <div class="view-controls">
                                <button onclick={on_back_to_galaxy} class="back-button">{ "← Back to Galaxy" }</button>
                            </div>
                            <SolarSystemGrid
                                solar_system={system.clone()}
                                planets={planets}
                                on_planet_click={Callback::from(on_planet_click)}
                            />
                        </div>
                    }
                } else {
                    html! { <div class="error">{ "Solar system not found" }</div> }
                }
            } else {
                html! { <div class="error">{ "No solar system selected" }</div> }
            }
        }
        ViewMode::Planet => {
            if let Some(planet) = (*selected_planet).clone() {
                let empire_resources = game_engine.borrow().game_state.empire_resources.clone();
                let on_terraform = {
                    let game_engine = game_engine.clone();
                    move |(planet_id, modifier_type): (u64, ModifierType)| {
                        // Calculate terraforming cost (simplified for now)
                        let cost = HashMap::new(); // TODO: Implement proper cost calculation
                        let duration = 1000; // 1000 ticks
                        let energy_cost = 100; // 100 energy

                        let result = game_engine.borrow_mut().start_terraforming_project(
                            planet_id,
                            modifier_type,
                            cost,
                            duration,
                            energy_cost,
                        );

                        if result {
                            log::info!("Started terraforming project for planet {}", planet_id);
                        } else {
                            log::warn!("Failed to start terraforming project");
                        }
                    }
                };

                let on_add_factory = {
                    let game_engine = game_engine.clone();
                    move |(planet_id, factory_type): (u64, FactoryType)| {
                        let result = game_engine
                            .borrow_mut()
                            .add_factory(planet_id, factory_type);
                        if result.is_some() {
                            log::info!("Added factory to planet {}", planet_id);
                        } else {
                            log::warn!("Failed to add factory");
                        }
                    }
                };

                html! {
                    <div class="planet-view">
                        <div class="view-controls">
                            <button onclick={on_back_to_system} class="back-button">{ "← Back to Solar System" }</button>
                        </div>
                        <PlanetDetailGrid
                            planet={planet}
                            empire_resources={empire_resources}
                            on_terraform={Callback::from(on_terraform)}
                            on_add_factory={Callback::from(on_add_factory)}
                        />
                    </div>
                }
            } else {
                html! { <div class="error">{ "No planet selected" }</div> }
            }
        }
    };

    let planet_panel = {
        let planet = (*selected_planet).clone();
        let empire_resources = game_engine.borrow().game_state.empire_resources.clone();
        let on_terraform = {
            let game_engine = game_engine.clone();
            move |(planet_id, modifier_type)| {
                // Calculate terraforming cost
                let mut cost = HashMap::new();
                cost.insert(ResourceType::Energy, 500);
                cost.insert(ResourceType::Minerals, 300);
                cost.insert(ResourceType::Population, 100);
                cost.insert(ResourceType::Technology, 150);

                let result = game_engine.borrow_mut().start_terraforming_project(
                    planet_id,
                    modifier_type,
                    cost,
                    1000, // duration
                    200,  // energy cost
                );
                if result {
                    log::info!(
                        "Started terraforming project on planet {} for modifier {:?}",
                        planet_id,
                        modifier_type
                    );
                } else {
                    log::warn!(
                        "Failed to start terraforming project on planet {} for modifier {:?} - insufficient resources or planet not conquered",
                        planet_id,
                        modifier_type
                    );
                }
            }
        };
        let on_add_factory = {
            let game_engine = game_engine.clone();
            move |(planet_id, factory_type)| {
                let result = game_engine
                    .borrow_mut()
                    .add_factory(planet_id, factory_type);
                match result {
                    Some(factory_id) => {
                        log::info!(
                            "Successfully added factory {:?} (ID: {}) to planet {}",
                            factory_type,
                            factory_id,
                            planet_id
                        );
                    }
                    None => {
                        log::warn!("Failed to add factory {:?} to planet {} - insufficient resources or planet not conquered", factory_type, planet_id);
                    }
                }
            }
        };

        let on_start_transport = {
            let game_engine = game_engine.clone();
            move |(from_planet, to_planet, resource_type, amount)| {
                let result = game_engine.borrow_mut().start_resource_transport(
                    from_planet,
                    to_planet,
                    resource_type,
                    amount,
                );
                if result {
                    log::info!(
                        "Started transport of {} {} from planet {} to planet {}",
                        amount,
                        format!("{:?}", resource_type),
                        from_planet,
                        to_planet
                    );
                } else {
                    log::warn!(
                        "Failed to start transport of {} {} from planet {} to planet {} - insufficient resources or invalid planets",
                        amount,
                        format!("{:?}", resource_type),
                        from_planet,
                        to_planet
                    );
                }
            }
        };

        let on_perform_prestige = {
            let game_engine = game_engine.clone();
            move |_| {
                let success = game_engine.borrow_mut().perform_prestige();
                if success {
                    log::info!("Successfully prestiged to new galaxy!");
                } else {
                    log::warn!("Failed to prestige - requirements not met");
                }
            }
        };

        if let Some(planet) = planet {
            html! {
                <>
                    <PlanetPanel
                        planet={planet.clone()}
                        on_terraform={Callback::from(on_terraform.clone())}
                        on_add_factory={Callback::from(on_add_factory.clone())}
                    />
                    <ConquestCost
                        planet={planet.clone()}
                        empire_resources={empire_resources.clone()}
                    />
                    <FactoryManagement
                        planet={planet.clone()}
                        empire_resources={empire_resources.clone()}
                        on_add_factory={Callback::from(on_add_factory)}
                    />
                    <TransportSystem
                        planets={game_engine.borrow().game_state.planets.clone()}
                        empire_resources={empire_resources.clone()}
                        on_start_transport={Callback::from(on_start_transport)}
                    />
                    <PrestigeSystem
                        current_prestige={game_engine.borrow().game_state.total_prestige_points}
                        galaxy_conquest_progress={game_engine.borrow().galaxy_system.get_galaxy_conquest_progress(
                            &game_engine.borrow().game_state.galaxies.get(&game_engine.borrow().game_state.current_galaxy).unwrap(),
                            &game_engine.borrow().game_state.solar_systems,
                            &game_engine.borrow().game_state.planets,
                        )}
                        can_prestige={game_engine.borrow().check_prestige_eligibility()}
                        prestige_requirements={game_engine.borrow().prestige_system.calculate_prestige_requirements(game_engine.borrow().game_state.total_prestige_points)}
                        on_perform_prestige={Callback::from(on_perform_prestige)}
                    />
                    <components::TerraformingProject
                        planet={planet}
                        empire_resources={empire_resources}
                        on_start_terraforming={Callback::from(on_terraform)}
                    />
                </>
            }
        } else {
            html! {
                <div class="no-planet-selected">
                    <h3>{ "No Planet Selected" }</h3>
                    <p>{ "Click on a planet to view its details and manage it." }</p>
                </div>
            }
        }
    };

    html! {
        <div class="game-container">
            <header>
                <h1>{ "Conquer Your Universe" }</h1>
                <div class="controls">
                    { speed_controls }
                    { pause_button }
                </div>
            </header>

            <main>
                <div class="game-layout">
                    <div class="header-panel">
                        <div>
                            { game_stats }
                        </div>
        <div>
                            { empire_resources }
                        </div>
                    </div>

                    <div class="center-panel">
                        { main_content }
                    </div>

                    <div class="footer-panel">
                        { planet_panel }
                    </div>
                </div>
            </main>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
