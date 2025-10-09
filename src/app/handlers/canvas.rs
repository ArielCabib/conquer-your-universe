use super::super::ContextMenuState;
use crate::{
    app::{current_time_ms, point_within_planet, random_range},
    types::{GameState, SettlerState},
};
use std::{cell::RefCell, rc::Rc};
use web_sys::{HtmlCanvasElement, MouseEvent};
use yew::{prelude::*, Callback, NodeRef};

pub fn click_handler(
    game_state: Rc<RefCell<GameState>>,
    canvas_ref: NodeRef,
    is_paused: UseStateHandle<bool>,
    context_menu_state: UseStateHandle<Option<ContextMenuState>>,
    alive_count: UseStateHandle<usize>,
) -> Callback<MouseEvent> {
    Callback::from(move |event: MouseEvent| {
        if *is_paused {
            return;
        }

        context_menu_state.set(None);

        if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
            let rect = canvas.get_bounding_client_rect();
            let width = rect.width();
            let height = rect.height();

            if width == 0.0 || height == 0.0 {
                return;
            }

            let client_x = event.client_x() as f64;
            let client_y = event.client_y() as f64;
            let scale_x = (canvas.width() as f64) / width;
            let scale_y = (canvas.height() as f64) / height;
            let canvas_x = (client_x - rect.left()) * scale_x;
            let canvas_y = (client_y - rect.top()) * scale_y;

            if point_within_planet(canvas_x, canvas_y) {
                if let Ok(mut state) = game_state.try_borrow_mut() {
                    let base_capacity = state.settlers_base_capacity as usize;
                    let settlers_per_house = state.settlers_per_house as usize;
                    let house_capacity = state.houses.len().saturating_mul(settlers_per_house);
                    let settlers_capacity_limit = base_capacity.saturating_add(house_capacity);
                    let alive_now = *alive_count;

                    if settlers_capacity_limit > 0 && alive_now >= settlers_capacity_limit {
                        return;
                    }

                    let id = state.next_settler_id;
                    state.next_settler_id += 1;
                    let now = current_time_ms();
                    let lifespan =
                        random_range(state.settler_min_lifespan_ms, state.settler_max_lifespan_ms);
                    state
                        .settlers
                        .push(SettlerState::new(id, canvas_x, canvas_y, now, lifespan));
                    alive_count.set(alive_now.saturating_add(1));
                }
            }
        }
    })
}

pub fn context_menu_handler(
    canvas_ref: NodeRef,
    is_paused: UseStateHandle<bool>,
    context_menu_state: UseStateHandle<Option<ContextMenuState>>,
) -> Callback<MouseEvent> {
    Callback::from(move |event: MouseEvent| {
        event.prevent_default();

        if *is_paused {
            return;
        }

        if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
            let rect = canvas.get_bounding_client_rect();
            let width = rect.width();
            let height = rect.height();

            if width == 0.0 || height == 0.0 {
                context_menu_state.set(None);
                return;
            }

            let client_x = event.client_x() as f64;
            let client_y = event.client_y() as f64;
            let scale_x = (canvas.width() as f64) / width;
            let scale_y = (canvas.height() as f64) / height;
            let canvas_x = (client_x - rect.left()) * scale_x;
            let canvas_y = (client_y - rect.top()) * scale_y;

            if !point_within_planet(canvas_x, canvas_y) {
                context_menu_state.set(None);
                return;
            }

            let offset_x = client_x - rect.left();
            let offset_y = client_y - rect.top();

            context_menu_state.set(Some(ContextMenuState {
                canvas_x,
                canvas_y,
                offset_x,
                offset_y,
            }));
        }
    })
}
