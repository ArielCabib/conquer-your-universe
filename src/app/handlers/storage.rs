use crate::{
    app::{ensure_house_registry, ensure_settler_lifespans},
    constants::STORAGE_KEY,
    types::{GameState, SettlerPhase},
};
use gloo::storage::{LocalStorage, Storage};
use js_sys::Array;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{
    Blob, BlobPropertyBag, FileReader, HtmlAnchorElement, HtmlInputElement, MouseEvent, Url,
};
use yew::{prelude::*, Callback, NodeRef};

pub fn open_file_dialog_handler(file_input_ref: NodeRef) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        if let Some(input) = file_input_ref.cast::<HtmlInputElement>() {
            input.click();
        }
    })
}

pub fn save_game_handler(game_state: Rc<RefCell<GameState>>) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        if let Ok(state_ref) = game_state.try_borrow() {
            match serde_json::to_string(&*state_ref) {
                Ok(serialized) => {
                    let parts = Array::new();
                    parts.push(&JsValue::from(serialized));

                    let bag = BlobPropertyBag::new();
                    bag.set_type("application/json");

                    match Blob::new_with_str_sequence_and_options(&parts, &bag) {
                        Ok(blob) => {
                            if let Some(window) = web_sys::window() {
                                if let Some(document) = window.document() {
                                    if let Ok(element) = document.create_element("a") {
                                        if let Ok(anchor) = element.dyn_into::<HtmlAnchorElement>()
                                        {
                                            match Url::create_object_url_with_blob(&blob) {
                                                Ok(url) => {
                                                    anchor.set_href(&url);
                                                    anchor.set_download(
                                                        "conquer-your-universe-save.json",
                                                    );
                                                    if let Some(body) = document.body() {
                                                        let _ = body.append_child(&anchor);
                                                        anchor.click();
                                                        let _ = body.remove_child(&anchor);
                                                    }
                                                    let _ = Url::revoke_object_url(&url);
                                                }
                                                Err(error) => {
                                                    log::warn!(
                                                        "Unable to generate download URL for save: {:?}",
                                                        error
                                                    );
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Err(error) => {
                            log::warn!("Failed to create save blob: {:?}", error);
                        }
                    }
                }
                Err(error) => {
                    log::warn!("Failed to serialize game state for save: {:?}", error);
                }
            }
        }
    })
}

pub fn file_change_handler(
    game_state: Rc<RefCell<GameState>>,
    alive_count: UseStateHandle<usize>,
    is_modal_open: UseStateHandle<bool>,
    is_paused: UseStateHandle<bool>,
    pause_time: Rc<RefCell<Option<f64>>>,
) -> Callback<web_sys::Event> {
    Callback::from(move |event: web_sys::Event| {
        let target = event
            .target()
            .and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

        if let Some(input) = target {
            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                    match FileReader::new() {
                        Ok(reader) => {
                            let reader_clone = reader.clone();
                            let game_state_handle = game_state.clone();
                            let alive_count_handle = alive_count.clone();
                            let is_modal_open_handle = is_modal_open.clone();
                            let pause_time_handle = pause_time.clone();
                            let is_paused_handle = is_paused.clone();

                            let onload = Closure::<dyn FnMut(_)>::wrap(Box::new(
                                move |_event: web_sys::Event| match reader_clone.result() {
                                    Ok(result) => {
                                        if let Some(text) = result.as_string() {
                                            match serde_json::from_str::<GameState>(&text) {
                                                Ok(mut loaded_state) => {
                                                    ensure_settler_lifespans(&mut loaded_state);
                                                    ensure_house_registry(&mut loaded_state);

                                                    let alive_total = loaded_state
                                                        .settlers
                                                        .iter()
                                                        .filter(|settler| {
                                                            matches!(
                                                                settler.phase,
                                                                SettlerPhase::Alive
                                                            )
                                                        })
                                                        .count();

                                                    if let Ok(mut state_ref) =
                                                        game_state_handle.try_borrow_mut()
                                                    {
                                                        *state_ref = loaded_state;
                                                        if let Err(error) = LocalStorage::set(
                                                            STORAGE_KEY,
                                                            &*state_ref,
                                                        ) {
                                                            log::warn!(
                                                                "Failed to persist loaded game state: {:?}",
                                                                error
                                                            );
                                                        }
                                                    }

                                                    alive_count_handle.set(alive_total);
                                                    *pause_time_handle.borrow_mut() = None;
                                                    is_paused_handle.set(false);
                                                    is_modal_open_handle.set(false);
                                                }
                                                Err(error) => {
                                                    log::warn!(
                                                        "Unable to parse game file: {:?}",
                                                        error
                                                    );
                                                }
                                            }
                                        }
                                    }
                                    Err(error) => {
                                        log::warn!("Failed to read file contents: {:?}", error);
                                    }
                                },
                            )
                                as Box<dyn FnMut(_)>);

                            reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                            onload.forget();

                            let onerror =
                                Closure::<dyn FnMut(_)>::wrap(Box::new(|_event: web_sys::Event| {
                                    log::warn!("An error occurred while reading the game file.");
                                })
                                    as Box<dyn FnMut(_)>);
                            reader.set_onerror(Some(onerror.as_ref().unchecked_ref()));
                            onerror.forget();

                            if let Err(error) = reader.read_as_text(&file) {
                                log::warn!("Failed to initiate game file read: {:?}", error);
                            }
                        }
                        Err(error) => {
                            log::warn!("Unable to create file reader: {:?}", error);
                        }
                    }
                }
            }

            input.set_value("");
        }
    })
}
