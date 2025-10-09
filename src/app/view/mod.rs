mod canvas;
mod header;
mod modal;
mod prompt;
mod stats;

use crate::constants::*;
use canvas::CanvasAreaProps;
use header::header_section;
use modal::control_modal;
use prompt::build_prompt;
use stats::stats_panel;
use yew::{events::MouseEvent, function_component, html, Callback, Html, NodeRef, Properties};

use super::ContextMenuState;

#[derive(Properties, PartialEq)]
pub struct AppViewProps {
    pub alive_now: usize,
    pub build_house_from_menu: Callback<MouseEvent>,
    pub can_build_house: bool,
    pub canvas_ref: NodeRef,
    pub canvas_style: String,
    pub close_modal: Callback<MouseEvent>,
    pub context_menu_state: Option<ContextMenuState>,
    pub file_input_ref: NodeRef,
    pub handle_click: Callback<MouseEvent>,
    pub handle_context_menu: Callback<MouseEvent>,
    pub houses_built: usize,
    pub houses_capacity_limit: usize,
    pub is_modal_active: bool,
    pub is_paused_now: bool,
    pub on_file_change: Callback<web_sys::Event>,
    pub open_file_dialog: Callback<MouseEvent>,
    pub open_settings: Callback<MouseEvent>,
    pub pause_status_text: String,
    pub restart_game: Callback<MouseEvent>,
    pub save_game: Callback<MouseEvent>,
    pub settlers_capacity_limit: usize,
    pub should_show_build_prompt: bool,
}

#[function_component(AppView)]
pub fn app_view(props: &AppViewProps) -> Html {
    let header = header_section(&props.open_settings);
    let prompt = build_prompt(props.should_show_build_prompt);
    let canvas_area = canvas::canvas_area(CanvasAreaProps {
        canvas_ref: props.canvas_ref.clone(),
        canvas_style: props.canvas_style.clone(),
        handle_click: props.handle_click.clone(),
        handle_context_menu: props.handle_context_menu.clone(),
        is_paused_now: props.is_paused_now,
        context_menu_state: props.context_menu_state.clone(),
        build_house_from_menu: props.build_house_from_menu.clone(),
        can_build_house: props.can_build_house,
    });
    let stats = stats_panel(
        props.alive_now,
        props.settlers_capacity_limit,
        props.houses_built,
        props.houses_capacity_limit,
    );
    let modal = control_modal(
        props.is_modal_active,
        &props.close_modal,
        &props.pause_status_text,
        &props.restart_game,
        &props.save_game,
        &props.open_file_dialog,
    );

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
                {header}
                {prompt}
                {canvas_area}
                {stats}
            </section>
            <input
                ref={props.file_input_ref.clone()}
                type="file"
                accept="application/json"
                style="display: none;"
                onchange={props.on_file_change.clone()}
            />
            {modal}
        </main>
    }
}
