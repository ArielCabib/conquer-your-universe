use crate::constants::*;
use yew::{html, Html};

pub fn build_prompt(should_show: bool) -> Html {
    if should_show {
        html! {
            <div
                style={format!(
                    "padding: 0.6rem 1rem; border-radius: 0.75rem; background: rgba(0,0,0,0.35); border: 1px solid rgba(248,225,200,0.35); color: {}; font-family: 'Trebuchet MS', sans-serif; font-size: 0.95rem; letter-spacing: 0.04em; text-transform: uppercase;",
                    ORBIT_03
                )}
            >
                {"Right click the planet to build a house"}
            </div>
        }
    } else {
        Html::default()
    }
}
