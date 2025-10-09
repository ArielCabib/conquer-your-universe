use crate::constants::*;
use yew::{events::MouseEvent, html, Callback, Html};

pub fn control_modal(
    is_active: bool,
    close_modal: &Callback<MouseEvent>,
    pause_status_text: &str,
    restart_game: &Callback<MouseEvent>,
    save_game: &Callback<MouseEvent>,
    open_file_dialog: &Callback<MouseEvent>,
) -> Html {
    if is_active {
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
