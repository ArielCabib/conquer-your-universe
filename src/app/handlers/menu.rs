use super::super::ContextMenuState;
use crate::{
    app::point_within_planet,
    constants::STORAGE_KEY,
    types::{GameState, HouseState},
};
use gloo::storage::{LocalStorage, Storage};
use std::{cell::RefCell, rc::Rc};
use yew::{events::MouseEvent, prelude::*, Callback};

pub fn build_house_menu_handler(
    game_state: Rc<RefCell<GameState>>,
    alive_count: UseStateHandle<usize>,
    context_menu_state: UseStateHandle<Option<ContextMenuState>>,
) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        let menu_state = (*context_menu_state).clone();

        if menu_state.is_none() {
            return;
        }

        if *alive_count < 1 {
            context_menu_state.set(None);
            return;
        }

        if let Some(menu) = menu_state {
            if !point_within_planet(menu.canvas_x, menu.canvas_y) {
                context_menu_state.set(None);
                return;
            }

            if let Ok(mut state_ref) = game_state.try_borrow_mut() {
                let house_limit = state_ref.houses_base_capacity as usize;
                if house_limit > 0 && state_ref.houses.len() >= house_limit {
                    context_menu_state.set(None);
                    return;
                }

                let house_id = state_ref.next_house_id;
                state_ref.next_house_id = state_ref.next_house_id.saturating_add(1);
                let built_at = crate::app::current_time_ms();
                state_ref.houses.push(HouseState::new(
                    house_id,
                    menu.canvas_x,
                    menu.canvas_y,
                    built_at,
                ));

                if let Err(error) = LocalStorage::set(STORAGE_KEY, &*state_ref) {
                    log::warn!(
                        "Failed to persist game state after building house: {:?}",
                        error
                    );
                }
            }

            context_menu_state.set(None);
        }
    })
}
