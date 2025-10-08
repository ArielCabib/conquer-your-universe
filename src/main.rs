use gloo::storage::{LocalStorage, Storage};
use gloo_timers::callback::Interval;
use js_sys::Array;
use log::info;
use std::{cell::Cell, rc::Rc};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{
    Blob, BlobPropertyBag, CanvasRenderingContext2d, FileReader, HtmlAnchorElement,
    HtmlCanvasElement, HtmlInputElement, Url,
};
use yew::{events::MouseEvent, prelude::*};

mod constants;
mod types;

use constants::{
    BIRTH_ANIMATION_MS, FADING_DURATION_MS, MOVE_DISTANCE_MAX, MOVE_DISTANCE_MIN, MOVE_INTERVAL_MS,
    ORBIT_01, ORBIT_02, ORBIT_03, ORBIT_04, ORBIT_05, PLANET_CENTER_X, PLANET_CENTER_Y,
    PLANET_RADIUS, SETTLER_RADIUS, STORAGE_KEY, VIEWBOX_HEIGHT, VIEWBOX_WIDTH,
};
use types::{GameState, HouseState, SettlerPhase, SettlerState};

fn ease_out_quad(t: f64) -> f64 {
    1.0 - (1.0 - t).powi(2)
}

fn current_time_ms() -> f64 {
    js_sys::Date::now()
}

fn point_within_planet(x: f64, y: f64) -> bool {
    let dx = x - PLANET_CENTER_X;
    let dy = y - PLANET_CENTER_Y;
    (dx * dx + dy * dy).sqrt() <= (PLANET_RADIUS - SETTLER_RADIUS)
}

fn random_range(min: f64, max: f64) -> f64 {
    let normalized = js_sys::Math::random();
    min + normalized * (max - min)
}

fn random_angle() -> f64 {
    random_range(0.0, std::f64::consts::TAU)
}

fn random_target_near(x: f64, y: f64) -> (f64, f64) {
    const ATTEMPTS: usize = 8;
    for _ in 0..ATTEMPTS {
        let angle = random_angle();
        let distance = random_range(MOVE_DISTANCE_MIN, MOVE_DISTANCE_MAX);
        let candidate_x = x + distance * angle.cos();
        let candidate_y = y + distance * angle.sin();
        if point_within_planet(candidate_x, candidate_y) {
            return (candidate_x, candidate_y);
        }
    }

    // Fallback to the planet's center if we couldn't find a suitable nearby spot.
    (PLANET_CENTER_X, PLANET_CENTER_Y)
}

fn ensure_settler_lifespans(state: &mut GameState) {
    let min_lifespan = state.settler_min_lifespan_ms;
    let max_lifespan = state.settler_max_lifespan_ms;

    for settler in &mut state.settlers {
        if settler.lifespan_ms <= 0.0 {
            settler.lifespan_ms = random_range(min_lifespan, max_lifespan);
        }
    }
}

fn ensure_house_registry(state: &mut GameState) {
    if let Some(highest_id) = state.houses.iter().map(|house| house.id).max() {
        let next_id = highest_id.saturating_add(1);
        if state.next_house_id <= highest_id {
            state.next_house_id = next_id;
        }
    }
}

fn random_planet_position() -> (f64, f64) {
    let safe_radius = (PLANET_RADIUS - 28.0).max(0.0);
    if safe_radius <= 0.0 {
        return (PLANET_CENTER_X, PLANET_CENTER_Y);
    }

    let angle = random_angle();
    let radius = safe_radius * js_sys::Math::random().sqrt();
    let x = PLANET_CENTER_X + radius * angle.cos();
    let y = PLANET_CENTER_Y + radius * angle.sin();
    (x, y)
}

fn draw_house(context: &CanvasRenderingContext2d, house: &HouseState) {
    let base_width = 28.0;
    let base_height = 18.0;
    let roof_height = 14.0;

    let base_x = house.x - base_width / 2.0;
    let base_y = house.y - base_height / 2.0;

    let _ = context.save();
    context.set_fill_style_str(ORBIT_01);
    context.fill_rect(base_x, base_y, base_width, base_height);

    context.set_fill_style_str(ORBIT_02);
    context.begin_path();
    context.move_to(base_x - 2.0, base_y);
    context.line_to(house.x, base_y - roof_height);
    context.line_to(base_x + base_width + 2.0, base_y);
    context.close_path();
    context.fill();

    context.set_fill_style_str(ORBIT_05);
    let window_size = base_width * 0.22;
    let window_y = base_y + base_height * 0.28;
    context.fill_rect(
        base_x + base_width * 0.16,
        window_y,
        window_size,
        window_size,
    );
    context.fill_rect(
        base_x + base_width - window_size - base_width * 0.16,
        window_y,
        window_size,
        window_size,
    );

    context.set_fill_style_str(ORBIT_04);
    let door_width = base_width * 0.28;
    let door_height = base_height * 0.62;
    context.fill_rect(
        house.x - door_width / 2.0,
        base_y + base_height - door_height,
        door_width,
        door_height,
    );

    let _ = context.restore();
}

#[function_component]
fn App() -> Html {
    let game_state = use_mut_ref(GameState::new);
    let alive_count = use_state(|| 0_usize);
    let canvas_ref = use_node_ref();
    let file_input_ref = use_node_ref();
    let is_modal_open = use_state(|| false);
    let is_paused = use_state(|| false);
    let pause_time = use_mut_ref(|| None::<f64>);

    {
        let game_state = game_state.clone();

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

    {
        let canvas_ref = canvas_ref.clone();
        let game_state = game_state.clone();
        let alive_count_state = alive_count.clone();
        let pause_time_state = pause_time.clone();

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
                    let alive_count_handle = alive_count_state.clone();
                    let pause_time_handle = pause_time_state.clone();
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

                        draw_context.set_fill_style_str(ORBIT_05);
                        draw_context.begin_path();
                        let _ = draw_context.arc(
                            PLANET_CENTER_X,
                            PLANET_CENTER_Y,
                            PLANET_RADIUS,
                            0.0,
                            std::f64::consts::TAU,
                        );
                        draw_context.fill();

                        if paused {
                            if let Ok(state) = game_state_handle.try_borrow() {
                                let mut alive_total = 0_usize;

                                for settler in &state.settlers {
                                    let (current_x, current_y) = settler.position_at(now);

                                    match &settler.phase {
                                        SettlerPhase::Alive => {
                                            let age_ms = now - settler.birth_ms;
                                            let mut display_radius = SETTLER_RADIUS;
                                            let mut base_alpha = 0.92;

                                            if age_ms < BIRTH_ANIMATION_MS {
                                                let progress =
                                                    (age_ms / BIRTH_ANIMATION_MS).clamp(0.0, 1.0);
                                                let eased = ease_out_quad(progress);
                                                display_radius = (SETTLER_RADIUS * eased).max(1.2);
                                                let halo_radius =
                                                    display_radius * (1.0 + 0.6 * (1.0 - eased));

                                                draw_context
                                                    .set_global_alpha((1.0 - progress) * 0.45);
                                                draw_context.set_fill_style_str(ORBIT_05);
                                                draw_context.begin_path();
                                                let _ = draw_context.arc(
                                                    current_x,
                                                    current_y,
                                                    halo_radius,
                                                    0.0,
                                                    std::f64::consts::TAU,
                                                );
                                                draw_context.fill();

                                                base_alpha = 0.74 + 0.18 * progress;
                                            }

                                            draw_context.set_global_alpha(base_alpha);
                                            draw_context.set_fill_style_str(ORBIT_04);
                                            draw_context.begin_path();
                                            let _ = draw_context.arc(
                                                current_x,
                                                current_y,
                                                display_radius,
                                                0.0,
                                                std::f64::consts::TAU,
                                            );
                                            draw_context.fill();
                                            draw_context.set_global_alpha(1.0);
                                            alive_total += 1;
                                        }
                                        SettlerPhase::Fading { started_ms } => {
                                            let elapsed = now - started_ms;
                                            if elapsed < FADING_DURATION_MS {
                                                let progress =
                                                    (elapsed / FADING_DURATION_MS).clamp(0.0, 1.0);
                                                let opacity = 1.0 - progress;
                                                let radius =
                                                    SETTLER_RADIUS * (1.0 + 0.6 * progress);

                                                draw_context.set_global_alpha(opacity);
                                                draw_context.set_fill_style_str(ORBIT_02);
                                                draw_context.begin_path();
                                                let _ = draw_context.arc(
                                                    current_x,
                                                    current_y,
                                                    radius,
                                                    0.0,
                                                    std::f64::consts::TAU,
                                                );
                                                draw_context.fill();
                                                draw_context.set_global_alpha(1.0);
                                            }
                                        }
                                    }
                                }

                                for house in &state.houses {
                                    draw_house(&draw_context, house);
                                }

                                alive_count_handle.set(alive_total);
                            }
                        } else if let Ok(mut state) = game_state_handle.try_borrow_mut() {
                            let settlers_vec = &mut state.settlers;
                            let mut alive_total = 0_usize;

                            settlers_vec.retain_mut(|settler| {
                                let (current_x, current_y) = settler.position_at(now);

                                if matches!(settler.phase, SettlerPhase::Alive)
                                    && now - settler.birth_ms >= settler.lifespan_ms
                                {
                                    settler.anchor_x = current_x;
                                    settler.anchor_y = current_y;
                                    settler.target_x = current_x;
                                    settler.target_y = current_y;
                                    settler.move_start_ms = now;
                                    settler.phase = SettlerPhase::Fading { started_ms: now };
                                }

                                match settler.phase {
                                    SettlerPhase::Alive => {
                                        if now - settler.last_direction_change_ms
                                            >= MOVE_INTERVAL_MS
                                        {
                                            let (target_x, target_y) =
                                                random_target_near(current_x, current_y);
                                            settler.anchor_x = current_x;
                                            settler.anchor_y = current_y;
                                            settler.target_x = target_x;
                                            settler.target_y = target_y;
                                            settler.move_start_ms = now;
                                            settler.last_direction_change_ms = now;
                                        }

                                        let age_ms = now - settler.birth_ms;
                                        let mut display_radius = SETTLER_RADIUS;
                                        let mut base_alpha = 0.92;

                                        if age_ms < BIRTH_ANIMATION_MS {
                                            let progress =
                                                (age_ms / BIRTH_ANIMATION_MS).clamp(0.0, 1.0);
                                            let eased = ease_out_quad(progress);
                                            display_radius = (SETTLER_RADIUS * eased).max(1.2);
                                            let halo_radius =
                                                display_radius * (1.0 + 0.6 * (1.0 - eased));

                                            draw_context.set_global_alpha((1.0 - progress) * 0.45);
                                            draw_context.set_fill_style_str(ORBIT_05);
                                            draw_context.begin_path();
                                            let _ = draw_context.arc(
                                                current_x,
                                                current_y,
                                                halo_radius,
                                                0.0,
                                                std::f64::consts::TAU,
                                            );
                                            draw_context.fill();

                                            base_alpha = 0.74 + 0.18 * progress;
                                        }

                                        draw_context.set_global_alpha(base_alpha);
                                        draw_context.set_fill_style_str(ORBIT_04);
                                        draw_context.begin_path();
                                        let _ = draw_context.arc(
                                            current_x,
                                            current_y,
                                            display_radius,
                                            0.0,
                                            std::f64::consts::TAU,
                                        );
                                        draw_context.fill();
                                        draw_context.set_global_alpha(1.0);
                                        alive_total += 1;
                                        true
                                    }
                                    SettlerPhase::Fading { started_ms } => {
                                        let elapsed = now - started_ms;
                                        if elapsed >= FADING_DURATION_MS {
                                            false
                                        } else {
                                            let progress =
                                                (elapsed / FADING_DURATION_MS).clamp(0.0, 1.0);
                                            let opacity = 1.0 - progress;
                                            let radius = SETTLER_RADIUS * (1.0 + 0.6 * progress);

                                            draw_context.set_global_alpha(opacity);
                                            draw_context.set_fill_style_str(ORBIT_02);
                                            draw_context.begin_path();
                                            let _ = draw_context.arc(
                                                current_x,
                                                current_y,
                                                radius,
                                                0.0,
                                                std::f64::consts::TAU,
                                            );
                                            draw_context.fill();
                                            draw_context.set_global_alpha(1.0);
                                            true
                                        }
                                    }
                                }
                            });

                            for house in &state.houses {
                                draw_house(&draw_context, house);
                            }

                            alive_count_handle.set(alive_total);
                        }
                    })
                    .forget();
                }
            }
            || ()
        });
    }

    {
        let game_state = game_state.clone();

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

    let handle_click = {
        let game_state = game_state.clone();
        let canvas_ref = canvas_ref.clone();
        let is_paused_state = is_paused.clone();

        Callback::from(move |event: MouseEvent| {
            if *is_paused_state {
                return;
            }

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
                        let id = state.next_settler_id;
                        state.next_settler_id += 1;
                        let now = current_time_ms();
                        let lifespan = random_range(
                            state.settler_min_lifespan_ms,
                            state.settler_max_lifespan_ms,
                        );
                        state
                            .settlers
                            .push(SettlerState::new(id, canvas_x, canvas_y, now, lifespan));
                    }
                }
            }
        })
    };

    let pause_button_label = if *is_paused {
        "Resume Time"
    } else {
        "Pause Time"
    };

    let open_settings = {
        let is_modal_open = is_modal_open.clone();
        Callback::from(move |_| is_modal_open.set(true))
    };

    let close_modal = {
        let is_modal_open = is_modal_open.clone();
        Callback::from(move |_| is_modal_open.set(false))
    };

    let restart_game = {
        let game_state = game_state.clone();
        let alive_count_handle = alive_count.clone();
        let is_modal_open = is_modal_open.clone();
        let is_paused = is_paused.clone();
        let pause_time = pause_time.clone();

        Callback::from(move |_| {
            if let Ok(mut state_ref) = game_state.try_borrow_mut() {
                *state_ref = GameState::new();
                if let Err(error) = LocalStorage::set(STORAGE_KEY, &*state_ref) {
                    log::warn!("Failed to persist game state after restart: {:?}", error);
                }
            }

            alive_count_handle.set(0);
            *pause_time.borrow_mut() = None;
            is_paused.set(false);
            is_modal_open.set(false);
        })
    };

    let toggle_pause = {
        let is_paused = is_paused.clone();
        let pause_time = pause_time.clone();
        let game_state = game_state.clone();

        Callback::from(move |_| {
            let currently_paused = *is_paused;
            info!(
                "toggle_pause clicked; currently_paused={}",
                currently_paused
            );

            if currently_paused {
                let paused_at_opt = *pause_time.borrow();
                if let Some(paused_at) = paused_at_opt {
                    let resume_now = current_time_ms();
                    let offset = resume_now - paused_at;

                    info!(
                        "resuming time; paused_at={:.2} resume_now={:.2} offset={:.2}",
                        paused_at, resume_now, offset
                    );

                    if offset > 0.0 {
                        if let Ok(mut state_ref) = game_state.try_borrow_mut() {
                            // Shift internal timers so animations resume without skipping ahead
                            for settler in &mut state_ref.settlers {
                                settler.move_start_ms += offset;
                                settler.last_direction_change_ms += offset;
                                settler.birth_ms += offset;
                                if let SettlerPhase::Fading { started_ms } = &mut settler.phase {
                                    *started_ms += offset;
                                }
                            }
                        }
                    }
                }

                *pause_time.borrow_mut() = None;
                is_paused.set(false);
                info!("time resumed");
            } else {
                let now = current_time_ms();
                *pause_time.borrow_mut() = Some(now);
                is_paused.set(true);
                info!("time paused");
            }
        })
    };

    let open_file_dialog = {
        let file_input_ref = file_input_ref.clone();
        Callback::from(move |_| {
            if let Some(input) = file_input_ref.cast::<HtmlInputElement>() {
                input.click();
            }
        })
    };

    let save_game = {
        let game_state = game_state.clone();

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
                                            if let Ok(anchor) =
                                                element.dyn_into::<HtmlAnchorElement>()
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
    };

    let build_house = {
        let game_state = game_state.clone();
        let alive_count_handle = alive_count.clone();

        Callback::from(move |_| {
            if *alive_count_handle < 1 {
                return;
            }

            if let Ok(mut state_ref) = game_state.try_borrow_mut() {
                let house_id = state_ref.next_house_id;
                state_ref.next_house_id = state_ref.next_house_id.saturating_add(1);
                let (x, y) = random_planet_position();
                let built_at = current_time_ms();
                state_ref
                    .houses
                    .push(HouseState::new(house_id, x, y, built_at));

                if let Err(error) = LocalStorage::set(STORAGE_KEY, &*state_ref) {
                    log::warn!(
                        "Failed to persist game state after building house: {:?}",
                        error
                    );
                }
            }
        })
    };

    let on_file_change = {
        let game_state = game_state.clone();
        let alive_count = alive_count.clone();
        let is_modal_open = is_modal_open.clone();
        let is_paused = is_paused.clone();
        let pause_time = pause_time.clone();

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
                                                                log::warn!("Failed to persist loaded game state: {:?}", error);
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

                                let onerror = Closure::<dyn FnMut(_)>::wrap(Box::new(
                                    |_event: web_sys::Event| {
                                        log::warn!(
                                            "An error occurred while reading the game file."
                                        );
                                    },
                                )
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
    };

    let is_paused_now = *is_paused;
    let pause_status_text = if is_paused_now {
        "Time is currently paused."
    } else {
        "Time is currently running."
    };

    let canvas_cursor = if is_paused_now {
        "not-allowed"
    } else {
        "pointer"
    };
    let canvas_pointer_events = if is_paused_now { "none" } else { "auto" };
    let canvas_style = format!(
        "width: min(80vw, 540px); height: auto; max-width: 600px; cursor: {}; touch-action: manipulation; pointer-events: {};",
        canvas_cursor, canvas_pointer_events
    );

    let alive_now = *alive_count;
    let can_build_house = alive_now >= 1;
    let build_button_label: &str = if can_build_house {
        "Build House"
    } else {
        "Build House (need settlers)"
    };
    let build_button_style = format!(
        "padding: 0.75rem 1rem; border-radius: 0.75rem; border: 1px solid rgba(248,225,200,0.35); background: rgba(0,0,0,0.35); color: {}; font-size: 1rem; letter-spacing: 0.06em; cursor: {}; opacity: {}; transition: opacity 0.2s ease;",
        ORBIT_03,
        if can_build_house { "pointer" } else { "not-allowed" },
        if can_build_house { "1" } else { "0.6" }
    );

    let houses_built = game_state
        .try_borrow()
        .map(|state| state.houses.len())
        .unwrap_or(0);

    let is_modal_active = *is_modal_open;

    html! {
        <main
            style={format!(
                "background-color: {}; min-height: 100vh; display: flex; align-items: center; justify-content: center;",
                ORBIT_01
            )}
        >
            <section
                style="display: flex; flex-direction: column; align-items: center; gap: 2.5rem; text-align: center;"
            >
                <div
                    style="display: flex; align-items: center; justify-content: space-between; max-width: 600px; gap: 1rem;"
                >
                    <div>
                    <div style="display: flex; align-items: center;">
                    <span></span>
                    <button
                        type="button"
                        aria-label="Open settings"
                        onclick={open_settings.clone()}
                        style={format!(
                            "display: inline-flex; align-items: center; justify-content: center; width: 2.25rem; height: 2.25rem; border-radius: 50%; border: 1px solid rgba(248, 225, 200, 0.4); background: rgba(0,0,0,0.35); color: {}; cursor: pointer; margin-left: auto; margin-right: 0;",
                            ORBIT_03
                        )}
                    >
                        <svg
                            width="28"
                            height="28"
                            viewBox="0 0 24 24"
                            fill="currentColor"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <path d="M11.983 2a1 1 0 0 1 .993.883l.007.117v1.19a5.52 5.52 0 0 1 1.45.6l.84-.84a1 1 0 0 1 1.497 1.32l-.083.094-.84.84a5.52 5.52 0 0 1 .6 1.451h1.19a1 1 0 0 1 .117 1.993l-.117.007h-1.19a5.52 5.52 0 0 1-.6 1.45l.84.841a1 1 0 0 1-1.32 1.497l-.094-.083-.84-.84a5.52 5.52 0 0 1-1.451.6v1.19a1 1 0 0 1-1.993.117l-.007-.117v-1.19a5.52 5.52 0 0 1-1.45-.6l-.84.84a1 1 0 0 1-1.497-1.32l.083-.094.84-.84a5.52 5.52 0 0 1-.6-1.451h-1.19a1 1 0 0 1-.117-1.993l.117-.007h1.19a5.52 5.52 0 0 1 .6-1.45l-.84-.841a1 1 0 0 1 1.32-1.497l.094.083.84.84a5.52 5.52 0 0 1 1.451-.6v-1.19A1 1 0 0 1 11.983 2Zm.017 5a3 3 0 1 0 0 6a3 3 0 0 0 0-6Z" />
                        </svg>
                    </button>
                    </div>
                    <h1
                        style={format!(
                            "color: {}; font-family: Orbitron, 'Trebuchet MS', sans-serif; font-size: clamp(2.5rem, 3vw, 3.5rem); letter-spacing: 0.12em; text-transform: uppercase; margin: 0;",
                            ORBIT_03
                        )}
                    >
                        {"Your Planet"}
                    </h1>
                    <h3 style={format!(
                        "color: {}; font-family: 'Trebuchet MS', sans-serif; font-size: clamp(1rem, 1vw, 1rem); letter-spacing: 0.05em; margin: 0.25rem 0 0 0;",
                        ORBIT_05
                    )}>
                        {"Click around and find out."}
                    </h3>


                    </div>
                </div>
                <div
                    style="position: relative; width: min(80vw, 540px); max-width: 600px;"
                >
                    <canvas
                        ref={canvas_ref}
                        width={VIEWBOX_WIDTH.to_string()}
                        height={VIEWBOX_HEIGHT.to_string()}
                        style={canvas_style.clone()}
                        onclick={handle_click}
                    >
                        {"Your browser does not support HTML canvas."}
                    </canvas>
                    {
                        if is_paused_now {
                            html! {
                                <div
                                    style={format!(
                                        "position: absolute; inset: 0; display: flex; align-items: center; justify-content: center; background: rgba(18, 11, 8, 0.55); color: {}; font-family: Orbitron, 'Trebuchet MS', sans-serif; letter-spacing: 0.08em; text-transform: uppercase; font-size: 1.1rem; pointer-events: none;",
                                        ORBIT_03
                                    )}
                                >
                                    {"Paused"}
                                </div>
                            }
                        } else {
                            Html::default()
                        }
                    }
                </div>
                <div
                    style={format!(
                        "margin-top: 1.5rem; padding: 0.75rem 1.25rem; border: 1px solid {}; border-radius: 0.75rem; background-color: rgba(0,0,0,0.25); color: {}; font-size: clamp(1rem, 2vw, 1.15rem); letter-spacing: 0.05em;",
                        ORBIT_02,
                        ORBIT_03
                    )}
                >
                    {format!("Settlers alive: {} · Houses built: {}", alive_now, houses_built)}
                </div>
            </section>
            <input
                ref={file_input_ref.clone()}
                type="file"
                accept="application/json"
                style="display: none;"
                onchange={on_file_change}
            />
            {
                if is_modal_active {
                    html! {
                        <div
                            style="position: fixed; inset: 0; background-color: rgba(0,0,0,0.65); display: flex; align-items: center; justify-content: center; padding: 1.5rem; z-index: 100;"
                        >
                            <div
                                style={format!(
                                    "background: rgba(28, 18, 14, 0.96); border: 1px solid rgba(248, 225, 200, 0.35); border-radius: 1rem; padding: 1.5rem; width: min(90vw, 420px); display: flex; flex-direction: column; gap: 1.25rem; color: {};",
                                    ORBIT_03
                                )}
                            >
                                <div
                                    style="display: flex; align-items: center; justify-content: space-between; gap: 0.75rem;"
                                >
                                    <h2
                                        style={format!(
                                            "margin: 0; font-size: 1.35rem; letter-spacing: 0.08em; text-transform: uppercase; font-family: Orbitron, 'Trebuchet MS', sans-serif; color: {};",
                                            ORBIT_03
                                        )}
                                    >
                                        {"Command Center"}
                                    </h2>
                                    <button
                                        type="button"
                                        onclick={close_modal.clone()}
                                        style="border: none; background: rgba(0,0,0,0.25); color: inherit; padding: 0.35rem 0.75rem; border-radius: 0.5rem; cursor: pointer; font-size: 0.9rem; letter-spacing: 0.06em;"
                                    >
                                        {"Close"}
                                    </button>
                                </div>
                                <p
                                    style="margin: 0; text-align: left; font-size: 0.95rem; letter-spacing: 0.04em; color: rgba(248, 225, 200, 0.85);"
                                >
                                    {pause_status_text}
                                </p>
                                <div
                                    style="display: flex; flex-direction: column; gap: 0.75rem;"
                                >
                                    <button
                                        type="button"
                                        onclick={restart_game.clone()}
                                        style={format!(
                                            "padding: 0.75rem 1rem; border-radius: 0.75rem; border: 1px solid rgba(248,225,200,0.35); background: rgba(0,0,0,0.35); color: {}; font-size: 1rem; letter-spacing: 0.06em; cursor: pointer;",
                                            ORBIT_03
                                        )}
                                    >
                                        {"Restart Game"}
                                    </button>
                                    <button
                                        type="button"
                                        onclick={toggle_pause.clone()}
                                        style={format!(
                                            "padding: 0.75rem 1rem; border-radius: 0.75rem; border: 1px solid rgba(248,225,200,0.35); background: rgba(0,0,0,0.35); color: {}; font-size: 1rem; letter-spacing: 0.06em; cursor: pointer;",
                                            ORBIT_03
                                        )}
                                    >
                                        {pause_button_label}
                                    </button>
                                    <button
                                        type="button"
                                        onclick={build_house.clone()}
                                        disabled={!can_build_house}
                                        style={build_button_style.clone()}
                                    >
                                        {build_button_label}
                                    </button>
                                    <button
                                        type="button"
                                        onclick={save_game.clone()}
                                        style={format!(
                                            "padding: 0.75rem 1rem; border-radius: 0.75rem; border: 1px solid rgba(248,225,200,0.35); background: rgba(0,0,0,0.35); color: {}; font-size: 1rem; letter-spacing: 0.06em; cursor: pointer;",
                                            ORBIT_03
                                        )}
                                    >
                                        {"Save Game"}
                                    </button>
                                    <button
                                        type="button"
                                        onclick={open_file_dialog.clone()}
                                        style={format!(
                                            "padding: 0.75rem 1rem; border-radius: 0.75rem; border: 1px solid rgba(248,225,200,0.35); background: rgba(0,0,0,0.35); color: {}; font-size: 1rem; letter-spacing: 0.06em; cursor: pointer;",
                                            ORBIT_03
                                        )}
                                    >
                                        {"Load Game"}
                                    </button>
                                </div>
                            </div>
                        </div>
                    }
                } else {
                    Html::default()
                }
            }
        </main>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
