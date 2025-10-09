use super::render_houses;
use crate::{
    app::ease_out_quad,
    constants::*,
    types::{GameState, SettlerPhase},
};
use web_sys::CanvasRenderingContext2d;
use yew::UseStateHandle;

pub fn render_paused_state(
    context: &CanvasRenderingContext2d,
    state: &GameState,
    now: f64,
    alive_count: &UseStateHandle<usize>,
) {
    let mut alive_total = 0_usize;

    for settler in &state.settlers {
        let (current_x, current_y) = settler.position_at(now);

        match &settler.phase {
            SettlerPhase::Alive => {
                let age_ms = now - settler.birth_ms;
                let mut display_radius = SETTLER_RADIUS;
                let mut base_alpha = 0.92;

                if age_ms < BIRTH_ANIMATION_MS {
                    let progress = (age_ms / BIRTH_ANIMATION_MS).clamp(0.0, 1.0);
                    let eased = ease_out_quad(progress);
                    display_radius = (SETTLER_RADIUS * eased).max(1.2);
                    let halo_radius = display_radius * (1.0 + 0.6 * (1.0 - eased));

                    context.set_global_alpha((1.0 - progress) * 0.45);
                    context.set_fill_style_str(ORBIT_05);
                    context.begin_path();
                    let _ = context.arc(
                        current_x,
                        current_y,
                        halo_radius,
                        0.0,
                        std::f64::consts::TAU,
                    );
                    context.fill();

                    base_alpha = 0.74 + 0.18 * progress;
                }

                context.set_global_alpha(base_alpha);
                context.set_fill_style_str(ORBIT_04);
                context.begin_path();
                let _ = context.arc(
                    current_x,
                    current_y,
                    display_radius,
                    0.0,
                    std::f64::consts::TAU,
                );
                context.fill();
                context.set_global_alpha(1.0);
                alive_total += 1;
            }
            SettlerPhase::Fading { started_ms } => {
                let elapsed = now - started_ms;
                if elapsed < FADING_DURATION_MS {
                    let progress = (elapsed / FADING_DURATION_MS).clamp(0.0, 1.0);
                    let opacity = 1.0 - progress;
                    let radius = SETTLER_RADIUS * (1.0 + 0.6 * progress);

                    context.set_global_alpha(opacity);
                    context.set_fill_style_str(ORBIT_02);
                    context.begin_path();
                    let _ = context.arc(current_x, current_y, radius, 0.0, std::f64::consts::TAU);
                    context.fill();
                    context.set_global_alpha(1.0);
                }
            }
        }
    }

    render_houses(context, state, now);
    alive_count.set(alive_total);
}
