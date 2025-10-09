use crate::{constants::STORAGE_KEY, types::GameState};
use gloo::storage::{LocalStorage, Storage};
use std::{cell::RefCell, rc::Rc};
use yew::{events::MouseEvent, prelude::*, Callback};

pub fn modal_open_handler(is_modal_open: UseStateHandle<bool>) -> Callback<MouseEvent> {
    Callback::from(move |_| is_modal_open.set(true))
}

pub fn modal_close_handler(is_modal_open: UseStateHandle<bool>) -> Callback<MouseEvent> {
    Callback::from(move |_| is_modal_open.set(false))
}

pub fn restart_game_handler(
    game_state: Rc<RefCell<GameState>>,
    alive_count: UseStateHandle<usize>,
    is_modal_open: UseStateHandle<bool>,
    is_paused: UseStateHandle<bool>,
    pause_time: Rc<RefCell<Option<f64>>>,
) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        if let Ok(mut state_ref) = game_state.try_borrow_mut() {
            *state_ref = GameState::new();
            if let Err(error) = LocalStorage::set(STORAGE_KEY, &*state_ref) {
                log::warn!("Failed to persist game state after restart: {:?}", error);
            }
        }

        alive_count.set(0);
        *pause_time.borrow_mut() = None;
        is_paused.set(false);
        is_modal_open.set(false);
    })
}
