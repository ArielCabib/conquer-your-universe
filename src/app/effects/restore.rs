use super::super::helpers::{ensure_house_registry, ensure_settler_lifespans};
use crate::{constants::STORAGE_KEY, types::GameState};
use gloo::storage::{LocalStorage, Storage};
use std::{cell::RefCell, rc::Rc};
use yew::prelude::*;

#[hook]
pub fn use_restore_state(game_state: Rc<RefCell<GameState>>) {
    use_effect_with((), move |_| {
        match LocalStorage::get::<GameState>(STORAGE_KEY) {
            Ok(mut stored_state) => {
                ensure_settler_lifespans(&mut stored_state);
                ensure_house_registry(&mut stored_state);
                *game_state.borrow_mut() = stored_state;
            }
            Err(_) => {
                if let Ok(state) = game_state.try_borrow() {
                    let snapshot = state.clone();
                    drop(state);
                    let _ = LocalStorage::set(STORAGE_KEY, &snapshot);
                }
            }
        }
        || ()
    });
}
