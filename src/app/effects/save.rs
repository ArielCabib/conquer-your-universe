use crate::{constants::STORAGE_KEY, types::GameState};
use gloo::storage::{LocalStorage, Storage};
use gloo_timers::callback::Interval;
use std::{cell::RefCell, rc::Rc};
use yew::prelude::*;

#[hook]
pub fn use_periodic_save(game_state: Rc<RefCell<GameState>>) {
    use_effect_with((), move |_| {
        let game_state_handle = game_state.clone();

        Interval::new(1_000, move || {
            if let Ok(state) = game_state_handle.try_borrow() {
                let snapshot = state.clone();
                drop(state);
                if let Err(error) = LocalStorage::set(STORAGE_KEY, &snapshot) {
                    log::warn!("Failed to persist game state: {:?}", error);
                }
            }
        })
        .forget();

        || ()
    });
}
