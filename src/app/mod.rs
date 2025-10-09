mod effects;
mod handlers;
mod helpers;
mod view;

use effects::{use_canvas_renderer, use_periodic_save, use_restore_state};
use handlers::{
    build_house_menu_handler, click_handler, context_menu_handler, file_change_handler,
    modal_close_handler, modal_open_handler, open_file_dialog_handler, restart_game_handler,
    save_game_handler,
};
use view::AppView;

use crate::types::GameState;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ContextMenuState {
    pub canvas_x: f64,
    pub canvas_y: f64,
    pub offset_x: f64,
    pub offset_y: f64,
}

pub use helpers::ease_out_quad;
pub use helpers::{
    current_time_ms, ensure_house_registry, ensure_settler_lifespans, point_within_planet,
    random_range, random_target_for_settler,
};

#[function_component]
pub fn App() -> Html {
    let game_state = use_mut_ref(GameState::new);
    let alive_count = use_state(|| 0_usize);
    let canvas_ref = use_node_ref();
    let file_input_ref = use_node_ref();
    let is_modal_open = use_state(|| false);
    let is_paused = use_state(|| false);
    let pause_time = use_mut_ref(|| None::<f64>);
    let context_menu_state = use_state(|| None::<ContextMenuState>);

    use_restore_state(game_state.clone());
    use_canvas_renderer(
        canvas_ref.clone(),
        game_state.clone(),
        alive_count.clone(),
        pause_time.clone(),
    );
    use_periodic_save(game_state.clone());

    let handle_click = click_handler(
        game_state.clone(),
        canvas_ref.clone(),
        is_paused.clone(),
        context_menu_state.clone(),
        alive_count.clone(),
    );

    let handle_context_menu = context_menu_handler(
        canvas_ref.clone(),
        is_paused.clone(),
        context_menu_state.clone(),
    );

    let open_settings = modal_open_handler(is_modal_open.clone());
    let close_modal = modal_close_handler(is_modal_open.clone());

    let restart_game = restart_game_handler(
        game_state.clone(),
        alive_count.clone(),
        is_modal_open.clone(),
        is_paused.clone(),
        pause_time.clone(),
    );

    let open_file_dialog = open_file_dialog_handler(file_input_ref.clone());
    let save_game = save_game_handler(game_state.clone());

    let build_house_from_menu = build_house_menu_handler(
        game_state.clone(),
        alive_count.clone(),
        context_menu_state.clone(),
    );

    let on_file_change = file_change_handler(
        game_state.clone(),
        alive_count.clone(),
        is_modal_open.clone(),
        is_paused.clone(),
        pause_time.clone(),
    );

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

    let (houses_built, settlers_base_capacity, houses_capacity_limit, settlers_per_house) =
        game_state
            .try_borrow()
            .map(|state| {
                (
                    state.houses.len(),
                    state.settlers_base_capacity as usize,
                    state.houses_base_capacity as usize,
                    state.settlers_per_house as usize,
                )
            })
            .unwrap_or((0, 0, 0, 0));

    let alive_now = *alive_count;
    let has_house_capacity = houses_capacity_limit == 0 || houses_built < houses_capacity_limit;
    let can_build_house = alive_now >= 1 && has_house_capacity;

    let settlers_capacity_limit =
        settlers_base_capacity.saturating_add(houses_built.saturating_mul(settlers_per_house));

    let should_show_build_prompt = alive_now >= 1 && houses_built == 0;
    let current_menu = (*context_menu_state).clone();

    let is_modal_active = *is_modal_open;

    html! {
        <AppView
            alive_now={alive_now}
            build_house_from_menu={build_house_from_menu}
            can_build_house={can_build_house}
            canvas_ref={canvas_ref}
            canvas_style={canvas_style}
            close_modal={close_modal}
            context_menu_state={current_menu}
            file_input_ref={file_input_ref}
            handle_click={handle_click}
            handle_context_menu={handle_context_menu}
            houses_built={houses_built}
            houses_capacity_limit={houses_capacity_limit}
            is_modal_active={is_modal_active}
            is_paused_now={is_paused_now}
            on_file_change={on_file_change}
            open_file_dialog={open_file_dialog}
            open_settings={open_settings}
            pause_status_text={pause_status_text.to_string()}
            restart_game={restart_game}
            save_game={save_game}
            settlers_capacity_limit={settlers_capacity_limit}
            should_show_build_prompt={should_show_build_prompt}
        />
    }
}
