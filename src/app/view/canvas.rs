use super::super::ContextMenuState;
use crate::constants::*;
use yew::{events::MouseEvent, html, Callback, Html, NodeRef};

#[derive(Clone, PartialEq)]
pub struct CanvasAreaProps {
    pub canvas_ref: NodeRef,
    pub canvas_style: String,
    pub handle_click: Callback<MouseEvent>,
    pub handle_context_menu: Callback<MouseEvent>,
    pub is_paused_now: bool,
    pub context_menu_state: Option<ContextMenuState>,
    pub build_house_from_menu: Callback<MouseEvent>,
    pub can_build_house: bool,
}

pub fn canvas_area(props: CanvasAreaProps) -> Html {
    let paused_overlay = if props.is_paused_now {
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
    };

    let context_menu = props
        .context_menu_state
        .clone()
        .map(|menu_state| {
            let menu_container_style = format!(
                "position: absolute; left: {:.2}px; top: {:.2}px; transform: translate(-50%, 0); min-width: 160px; background: rgba(28, 18, 14, 0.94); border: 1px solid rgba(248, 225, 200, 0.4); border-radius: 0.6rem; box-shadow: 0 12px 24px rgba(0, 0, 0, 0.35); padding: 0.35rem; z-index: 10;",
                menu_state.offset_x,
                menu_state.offset_y
            );
            let button_style = format!(
                "width: 100%; text-align: left; padding: 0.5rem 0.75rem; border: none; background: transparent; color: {}; font-family: 'Trebuchet MS', sans-serif; font-size: 0.95rem; letter-spacing: 0.04em; border-radius: 0.5rem; cursor: {};",
                ORBIT_03,
                if props.can_build_house { "pointer" } else { "not-allowed" }
            );
            html! {
                <div style={menu_container_style}>
                    <button
                        type="button"
                        style={button_style}
                        onclick={props.build_house_from_menu.clone()}
                        disabled={!props.can_build_house}
                    >
                        {"Build House"}
                    </button>
                </div>
            }
        })
        .unwrap_or_default();

    html! {
        <div
            style="position: relative; width: min(80vw, 540px); max-width: 600px;"
        >
            <canvas
                ref={props.canvas_ref}
                width={VIEWBOX_WIDTH.to_string()}
                height={VIEWBOX_HEIGHT.to_string()}
                style={props.canvas_style}
                onclick={props.handle_click}
                oncontextmenu={props.handle_context_menu}
            >
                {"Your browser does not support HTML canvas."}
            </canvas>
            {paused_overlay}
            {context_menu}
        </div>
    }
}
