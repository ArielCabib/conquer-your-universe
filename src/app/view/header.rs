use crate::constants::*;
use yew::{events::MouseEvent, html, Callback, Html};

pub fn header_section(open_settings: &Callback<MouseEvent>) -> Html {
    html! {
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
    }
}
