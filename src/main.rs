use std::cell::RefCell;
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
        html! {
            <GameStats stats={stats} />
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

    let galaxy_map = {
        let galaxies = game_engine.borrow().game_state.galaxies.clone();
        let solar_systems = game_engine.borrow().game_state.solar_systems.clone();
        let planets = game_engine.borrow().game_state.planets.clone();
        let current_galaxy = game_engine.borrow().game_state.current_galaxy;
        let planets_clone = planets.clone();
        let on_planet_click = {
            let selected_planet = selected_planet.clone();
            move |planet_id: u64| {
                if let Some(planet) = planets_clone.get(&planet_id) {
                    selected_planet.set(Some(planet.clone()));
                }
            }
        };

        html! {
            <GalaxyMap
                galaxies={galaxies}
                solar_systems={solar_systems}
                planets={planets}
                current_galaxy={current_galaxy}
                on_planet_click={Callback::from(on_planet_click)}
            />
        }
    };

    let planet_panel = {
        let planet = (*selected_planet).clone();
        let on_close = {
            let selected_planet = selected_planet.clone();
            move |_| selected_planet.set(None)
        };
        let on_terraform = {
            let _game_engine = game_engine.clone();
            move |(planet_id, modifier_type)| {
                // This would be implemented to start terraforming
                log::info!(
                    "Starting terraforming on planet {} for modifier {:?}",
                    planet_id,
                    modifier_type
                );
            }
        };
        let on_add_factory = {
            let _game_engine = game_engine.clone();
            move |(planet_id, factory_type)| {
                // This would be implemented to add factory
                log::info!("Adding factory {:?} to planet {}", factory_type, planet_id);
            }
        };

        html! {
            <PlanetPanel
                planet={planet}
                on_close={Callback::from(on_close)}
                on_terraform={Callback::from(on_terraform)}
                on_add_factory={Callback::from(on_add_factory)}
            />
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
                    <div class="left-panel">
                        { game_stats }
                        { empire_resources }
                    </div>

                    <div class="center-panel">
                        { galaxy_map }
                    </div>

                    <div class="right-panel">
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
