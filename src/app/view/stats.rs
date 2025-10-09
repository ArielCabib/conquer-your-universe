use crate::constants::*;
use yew::{html, Html};

pub fn stats_panel(
    alive_now: usize,
    settlers_capacity_limit: usize,
    houses_built: usize,
    houses_capacity_limit: usize,
) -> Html {
    html! {
        <div style="display: flex; flex-direction: row; align-items: center; gap: 1.5rem; max-width: 600px; width: min(80vw, 540px);">
            <div
                style={format!(
                    "margin-top: 1.5rem; padding: 0.75rem 1.25rem; border: 1px solid {}; border-radius: 0.75rem; background-color: rgba(0,0,0,0.25); color: {}; font-size: clamp(1rem, 2vw, 1.15rem); letter-spacing: 0.05em;",
                    ORBIT_02,
                    ORBIT_03
                )}
            >
                {format!("Settlers alive: {}/{}", alive_now, settlers_capacity_limit)}
            </div>
            <div
                style={format!(
                    "margin-top: 1.5rem; padding: 0.75rem 1.25rem; border: 1px solid {}; border-radius: 0.75rem; background-color: rgba(0,0,0,0.25); color: {}; font-size: clamp(1rem, 2vw, 1.15rem); letter-spacing: 0.05em;",
                    ORBIT_02,
                    ORBIT_03
                )}
            >
                {format!("Houses built: {}/{}", houses_built, houses_capacity_limit)}
            </div>
        </div>
    }
}
