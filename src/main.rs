use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::MouseEvent;
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

const TICKS_PER_SECOND: u32 = 10; // Base ticks per second

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
    let refresh_trigger = use_state(|| 0);

    // Game update loop with dynamic speed
    let game_engine_clone = game_engine.clone();
    let game_tick_clone = game_tick.clone();
    let is_paused_clone = is_paused.clone();
    let game_speed_clone = game_speed.clone();

    use_effect(move || {
        let interval =
            gloo_timers::callback::Interval::new(1000 / TICKS_PER_SECOND as u32, move || {
                if !*is_paused_clone {
                    let speed_multiplier = *game_speed_clone as u64;

                    // Run multiple updates based on speed multiplier
                    for _ in 0..speed_multiplier {
                        game_engine_clone.borrow_mut().update();
                    }

                    game_tick_clone.set(*game_tick_clone + speed_multiplier);
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
                }} class={classes!(
                    "control-btn",
                    "speed-btn",
                    (*game_speed == GameSpeed::Normal).then_some("active")
                )}>
                    { "1x" }
                </button>
                <button onclick={move |_| {
                    game_engine_10x.borrow_mut().set_game_speed(GameSpeed::Fast);
                    game_speed_10x.set(GameSpeed::Fast);
                }} class={classes!(
                    "control-btn",
                    "speed-btn",
                    (*game_speed == GameSpeed::Fast).then_some("active")
                )}>
                    { "10x" }
                </button>
                <button onclick={move |_| {
                    game_engine_100x.borrow_mut().set_game_speed(GameSpeed::VeryFast);
                    game_speed_100x.set(GameSpeed::VeryFast);
                }} class={classes!(
                    "control-btn",
                    "speed-btn",
                    (*game_speed == GameSpeed::VeryFast).then_some("active")
                )}>
                    { "100x" }
                </button>
                <button onclick={move |_| {
                    game_engine_1000x.borrow_mut().set_game_speed(GameSpeed::UltraFast);
                    game_speed_1000x.set(GameSpeed::UltraFast);
                }} class={classes!(
                    "control-btn",
                    "speed-btn",
                    (*game_speed == GameSpeed::UltraFast).then_some("active")
                )}>
                    { "1000x" }
                </button>
            </div>
        }
    };

    let pause_button = {
        let game_engine = game_engine.clone();
        let is_paused_clone = is_paused.clone();

        html! {
            <button
                class={classes!(
                    "control-btn",
                    "pause-btn",
                    (*is_paused).then_some("active")
                )}
                onclick={move |_| {
                game_engine.borrow_mut().toggle_pause();
                is_paused_clone.set(!*is_paused_clone);
            }}
            >
                { if *is_paused { "Resume" } else { "Pause" } }
            </button>
        }
    };

    let selected_planet = use_state(|| None::<Planet>);
    let show_prestige_modal = use_state(|| false);
    let show_controls_modal = use_state(|| false);
    let game_stats = {
        let stats = game_engine.borrow().get_game_statistics();
        let planet_count = game_engine.borrow().get_planet_count();
        let on_prestige_card_click = {
            let show_prestige_modal = show_prestige_modal.clone();
            Callback::from(move |_| show_prestige_modal.set(true))
        };
        let on_speed_card_click = {
            let show_controls_modal = show_controls_modal.clone();
            Callback::from(move |_| show_controls_modal.set(true))
        };
        let on_status_card_click = {
            let show_controls_modal = show_controls_modal.clone();
            Callback::from(move |_| show_controls_modal.set(true))
        };
        html! {
            <GameStats
                stats={stats}
                planet_count={planet_count}
                on_prestige_card_click={on_prestige_card_click}
                on_speed_card_click={on_speed_card_click}
                on_status_card_click={on_status_card_click}
            />
        }
    };

    let on_mine_resource = {
        let game_engine = game_engine.clone();
        let refresh_trigger = refresh_trigger.clone();
        let selected_planet_handle = selected_planet.clone();
        Callback::from(move |resource_type: ResourceType| {
            let mined = {
                let mut engine = game_engine.borrow_mut();
                engine.mine_resource(resource_type, 1)
            };

            if mined {
                refresh_trigger.set(*refresh_trigger + 1);

                let current_planet = (*selected_planet_handle).clone();
                if let Some(current_planet) = current_planet {
                    let updated_planet = {
                        let engine_ref = game_engine.borrow();
                        engine_ref
                            .game_state
                            .planets
                            .get(&current_planet.id)
                            .cloned()
                    };

                    if let Some(updated_planet) = updated_planet {
                        selected_planet_handle.set(Some(updated_planet));
                    }
                }
            }
        })
    };

    let empire_resources = {
        let resources = game_engine.borrow().game_state.empire_resources.clone();
        let generation = game_engine
            .borrow()
            .game_state
            .last_resource_generation
            .clone();
        let storage_limits = game_engine.borrow().resource_system.get_storage_limits();
        html! {
            <ResourceDashboard
                empire_resources={resources}
                resource_generation={generation}
                storage_limits={storage_limits}
                on_mine_resource={on_mine_resource.clone()}
            />
        }
    };

    let on_perform_prestige_modal = {
        let game_engine = game_engine.clone();
        let show_prestige_modal = show_prestige_modal.clone();
        Callback::from(move |_| {
            let success = game_engine.borrow_mut().perform_prestige();
            if success {
                log::info!("Successfully prestiged to new galaxy!");
                show_prestige_modal.set(false);
            } else {
                log::warn!("Failed to prestige - requirements not met");
            }
        })
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
    // Use refresh_trigger to ensure UI updates after JSON load
    let _refresh_trigger = *refresh_trigger;
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
                    // Clone the system and planets once to avoid continuous re-renders
                    let system_clone = system.clone();
                    let planets = game_engine.borrow().game_state.planets.clone();
                    let on_planet_click_callback = Callback::from(on_planet_click);

                    html! {
                        <div class="solar-system-view">
                            <div class="view-controls">
                                <button onclick={on_back_to_galaxy} class="back-button">{ "← Back to Galaxy" }</button>
                            </div>
                            <SolarSystemGrid
                                solar_system={system_clone}
                                planets={planets}
                                on_planet_click={on_planet_click_callback}
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
                let storage_limits = game_engine.borrow().resource_system.get_storage_limits();
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

                let on_add_building = {
                    let game_engine = game_engine.clone();
                    move |(planet_id, building_type): (u64, BuildingType)| {
                        let result = game_engine
                            .borrow_mut()
                            .add_building(planet_id, building_type);
                        if result.is_some() {
                            log::info!("Added building to planet {}", planet_id);
                        } else {
                            log::warn!("Failed to add building");
                        }
                    }
                };

                html! {
                    <div class="planet-view">
                        <div class="view-controls">
                            <button onclick={on_back_to_system} class="back-button">{ "← Back to Solar System" }</button>
                        </div>

                        <div class="planet-details-section">
                            <PlanetDetailGrid
                                planet={planet.clone()}
                                empire_resources={empire_resources.clone()}
                                on_terraform={Callback::from(on_terraform.clone())}
                                on_add_building={Callback::from(on_add_building.clone())}
                                on_mine_resource={on_mine_resource.clone()}
                                storage_limits={storage_limits.clone()}
                            />
                        </div>
                    </div>
                }
            } else {
                html! { <div class="error">{ "No planet selected" }</div> }
            }
        }
    };

    // Save/Load JSON callbacks
    let on_save_json = {
        let game_engine = game_engine.clone();
        move |_| {
            game_engine.borrow().save_to_json_file();
        }
    };

    let on_load_json = {
        let game_engine = game_engine.clone();
        let refresh_trigger = refresh_trigger.clone();
        move |e: web_sys::Event| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                if let Some(files) = input.files() {
                    if let Some(file) = files.get(0) {
                        let reader = web_sys::FileReader::new().unwrap();
                        let game_engine = game_engine.clone();

                        let refresh_trigger = refresh_trigger.clone();
                        let onload = Closure::wrap(Box::new(move |e: web_sys::Event| {
                            if let Some(reader) = e.target_dyn_into::<web_sys::FileReader>() {
                                if let Ok(result) = reader.result() {
                                    if let Some(content) = result.as_string() {
                                        let success =
                                            game_engine.borrow_mut().load_from_json_file(&content);
                                        if success {
                                            log::info!("Game loaded from JSON file");
                                            // Trigger UI refresh
                                            refresh_trigger.set(*refresh_trigger + 1);
                                            // Clear the file input to prevent reloading the same file
                                            input.set_value("");
                                        } else {
                                            log::warn!("Failed to load game from JSON file");
                                        }
                                    }
                                }
                            }
                        }) as Box<dyn FnMut(_)>);

                        reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                        onload.forget();

                        let _ = reader.read_as_text(&file);
                    }
                }
            }
        }
    };

    let on_reset_game = {
        let game_engine = game_engine.clone();
        let refresh_trigger = refresh_trigger.clone();
        let current_view = current_view.clone();
        let selected_solar_system = selected_solar_system.clone();
        let selected_planet = selected_planet.clone();
        move |_| {
            // Confirm reset
            if let Some(window) = web_sys::window() {
                if let Ok(confirmed) = window.confirm_with_message("Are you sure you want to reset the game? This will delete all progress and cannot be undone.") {
                    if confirmed {
                        game_engine.borrow_mut().reset_game();
                        // Reset UI state
                        current_view.set(ViewMode::Galaxy);
                        selected_solar_system.set(None);
                        selected_planet.set(None);
                        // Trigger UI refresh
                        refresh_trigger.set(*refresh_trigger + 1);
                        log::info!("Game reset confirmed");
                    }
                }
            }
        }
    };

    let prestige_modal = {
        if *show_prestige_modal {
            let overlay_close = {
                let show_prestige_modal = show_prestige_modal.clone();
                Callback::from(move |_| show_prestige_modal.set(false))
            };
            let close_button_click = overlay_close.clone();
            let modal_click = Callback::from(|event: MouseEvent| event.stop_propagation());
            let (current_prestige, galaxy_conquest_progress, can_prestige, prestige_requirements) = {
                let engine = game_engine.borrow();
                let current_prestige = engine.game_state.total_prestige_points;
                let current_galaxy_id = engine.game_state.current_galaxy;
                let galaxy_conquest_progress = engine
                    .game_state
                    .galaxies
                    .get(&current_galaxy_id)
                    .map(|galaxy| {
                        engine.galaxy_system.get_galaxy_conquest_progress(
                            galaxy,
                            &engine.game_state.solar_systems,
                            &engine.game_state.planets,
                        )
                    })
                    .unwrap_or(0.0);
                let prestige_requirements = engine
                    .prestige_system
                    .calculate_prestige_requirements(current_prestige);
                let can_prestige = engine.check_prestige_eligibility();
                (
                    current_prestige,
                    galaxy_conquest_progress,
                    can_prestige,
                    prestige_requirements,
                )
            };

            html! {
                <div class="modal-overlay" onclick={overlay_close}>
                    <div class="modal-content prestige-modal" onclick={modal_click}>
                        <button class="modal-close" onclick={close_button_click} aria-label="Close prestige details">{ "×" }</button>
                        <PrestigeSystem
                            current_prestige={current_prestige}
                            galaxy_conquest_progress={galaxy_conquest_progress}
                            can_prestige={can_prestige}
                            prestige_requirements={prestige_requirements}
                            on_perform_prestige={on_perform_prestige_modal.clone()}
                        />
                    </div>
                </div>
            }
        } else {
            html! {}
        }
    };

    let controls_modal = {
        if *show_controls_modal {
            let overlay_close = {
                let show_controls_modal = show_controls_modal.clone();
                Callback::from(move |_| show_controls_modal.set(false))
            };
            let close_button_click = overlay_close.clone();
            let modal_click = Callback::from(|event: MouseEvent| event.stop_propagation());

            html! {
                <div class="modal-overlay" onclick={overlay_close}>
                    <div class="modal-content status-modal" onclick={modal_click}>
                        <button class="modal-close" onclick={close_button_click} aria-label="Close game controls">{ "×" }</button>
                        <div class="modal-section">
                            <h4>{ "Game Speed" }</h4>
                            { speed_controls.clone() }
                        </div>
                        <div class="modal-section">
                            <h4>{ "Session" }</h4>
                            <div class="session-controls">
                                { pause_button.clone() }
                            </div>
                        </div>
                    </div>
                </div>
            }
        } else {
            html! {}
        }
    };

    html! {
        <div class="game-container">
            <header class="game-header">
                <div class="header-title">
                    <h1>{ "Conquer Your Universe" }</h1>
                    <p class="header-subtitle">{ "Galactic command console" }</p>
                </div>
                <div class="header-actions">
                    <div class="control-stack">
                        <span class="control-label">{ "Data" }</span>
                        <div class="save-load-controls">
                            <button onclick={on_save_json} class="save-btn">{ "Save Game" }</button>
                            <input type="file" accept=".json" onchange={on_load_json} class="load-input" id="load-json-input" />
                            <label for="load-json-input" class="load-btn">{ "Load Game" }</label>
                            <button onclick={on_reset_game} class="reset-btn">{ "Reset" }</button>
                        </div>
                    </div>
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
                </div>
            </main>
            { prestige_modal }
            { controls_modal }
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
