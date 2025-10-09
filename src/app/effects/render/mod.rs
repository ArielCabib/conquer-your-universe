mod active;
mod paused;

use super::super::helpers::{current_time_ms, draw_house};
use crate::{constants::*, types::GameState};
use active::handle_active_state;
use gloo_timers::callback::Interval;
use log::info;
use paused::render_paused_state;
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

#[hook]
pub fn use_canvas_renderer(
    canvas_ref: NodeRef,
    game_state: Rc<RefCell<GameState>>,
    alive_count: UseStateHandle<usize>,
    pause_time: Rc<RefCell<Option<f64>>>,
) {
    use_effect_with((), move |_| {
        if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
            canvas.set_width(VIEWBOX_WIDTH as u32);
            canvas.set_height(VIEWBOX_HEIGHT as u32);

            if let Some(context) = canvas
                .get_context("2d")
                .ok()
                .flatten()
                .and_then(|ctx| ctx.dyn_into::<CanvasRenderingContext2d>().ok())
            {
                let draw_context = context.clone();
                let game_state_handle = game_state.clone();
                let alive_count_handle = alive_count.clone();
                let pause_time_handle = pause_time.clone();
                let pause_log_handle = Rc::new(Cell::new(None::<bool>));

                Interval::new(16, move || {
                    let pause_value = *pause_time_handle.borrow();
                    let now = pause_value.unwrap_or_else(current_time_ms);
                    let paused = pause_value.is_some();

                    if pause_log_handle.get() != Some(paused) {
                        pause_log_handle.set(Some(paused));
                        info!(
                            "frame_state_change paused={} now={:.2} alive_count={}",
                            paused, now, *alive_count_handle
                        );
                    }

                    draw_context.clear_rect(0.0, 0.0, VIEWBOX_WIDTH, VIEWBOX_HEIGHT);
                    render_planet(&draw_context);

                    if paused {
                        if let Ok(state) = game_state_handle.try_borrow() {
                            render_paused_state(&draw_context, &state, now, &alive_count_handle);
                        }
                    } else if let Ok(mut state) = game_state_handle.try_borrow_mut() {
                        handle_active_state(&draw_context, &mut state, now, &alive_count_handle);
                    }
                })
                .forget();
            }
        }
        || ()
    });
}

fn render_planet(context: &CanvasRenderingContext2d) {
    context.set_fill_style_str(ORBIT_05);
    context.begin_path();
    let _ = context.arc(
        PLANET_CENTER_X,
        PLANET_CENTER_Y,
        PLANET_RADIUS,
        0.0,
        std::f64::consts::TAU,
    );
    context.fill();
}

fn draw_houses(context: &CanvasRenderingContext2d, state: &GameState, now: f64) {
    for house in &state.houses {
        draw_house(context, house, now);
    }
}

pub(super) fn render_houses(context: &CanvasRenderingContext2d, state: &GameState, now: f64) {
    draw_houses(context, state, now);
}
