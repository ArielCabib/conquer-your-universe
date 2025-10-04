use std::cell::RefCell;
use std::rc::Rc;
use yew::prelude::*;

// Import all our game modules
mod galaxy_system;
mod game_engine;
mod planet_system;
mod prestige_system;
mod resource_system;
mod supply_chain;
mod transport_system;
mod types;

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

    let game_stats = {
        let stats = game_engine.borrow().get_game_statistics();
        html! {
            <div class="game-stats">
                <h2>{ "Game Statistics" }</h2>
                <p>{ format!("Tick: {}", stats.current_tick) }</p>
                <p>{ format!("Conquered Planets: {}/{}", stats.conquered_planets, stats.total_planets) }</p>
                <p>{ format!("Total Factories: {}", stats.total_factories) }</p>
                <p>{ format!("Total Resources: {}", stats.total_resources) }</p>
                <p>{ format!("Prestige Points: {}", stats.prestige_points) }</p>
                <p>{ format!("Speed: {}x", stats.game_speed as u64) }</p>
                <p>{ format!("Status: {}", if stats.is_paused { "Paused" } else { "Running" }) }</p>
            </div>
        }
    };

    let empire_resources = {
        let resources = &game_engine.borrow().game_state.empire_resources;
        html! {
            <div class="empire-resources">
                <h3>{ "Empire Resources" }</h3>
                { for resources.iter().map(|(resource_type, amount)| {
                    html! {
                        <p>{ format!("{:?}: {}", resource_type, amount) }</p>
                    }
                }) }
            </div>
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
                <div class="game-info">
                    { game_stats }
                    { empire_resources }
                </div>

                <div class="game-content">
                    <p>{ "Game infrastructure is ready! All systems are initialized and running." }</p>
                    <p>{ "The game engine includes:" }</p>
                    <ul>
                        <li>{ "Resource generation and management" }</li>
                        <li>{ "Planet generation with modifiers and terraforming" }</li>
                        <li>{ "Supply chain system with product DAG" }</li>
                        <li>{ "Transport system for resource flow" }</li>
                        <li>{ "Galaxy and solar system structure" }</li>
                        <li>{ "Prestige system for galaxy progression" }</li>
                        <li>{ "Speed control (1x to 1000x)" }</li>
                        <li>{ "Save/load functionality" }</li>
                    </ul>
                </div>
            </main>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
