use super::render_houses;
use crate::app::{random_range, random_target_for_settler};
use crate::{
    app::ease_out_quad,
    constants::*,
    types::{GameState, SettlerPhase, SettlerState},
};
use web_sys::CanvasRenderingContext2d;
use yew::UseStateHandle;

pub fn handle_active_state(
    context: &CanvasRenderingContext2d,
    state: &mut GameState,
    now: f64,
    alive_count: &UseStateHandle<usize>,
) {
    let mut alive_total = 0_usize;

    {
        let settlers_vec = &mut state.settlers;
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
                    if now - settler.last_direction_change_ms >= MOVE_INTERVAL_MS {
                        let (target_x, target_y) = random_target_for_settler(current_x, current_y);
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
                    true
                }
                SettlerPhase::Fading { started_ms } => {
                    let elapsed = now - started_ms;
                    if elapsed >= FADING_DURATION_MS {
                        false
                    } else {
                        let progress = (elapsed / FADING_DURATION_MS).clamp(0.0, 1.0);
                        let opacity = 1.0 - progress;
                        let radius = SETTLER_RADIUS * (1.0 + 0.6 * progress);

                        context.set_global_alpha(opacity);
                        context.set_fill_style_str(ORBIT_02);
                        context.begin_path();
                        let _ =
                            context.arc(current_x, current_y, radius, 0.0, std::f64::consts::TAU);
                        context.fill();
                        context.set_global_alpha(1.0);
                        true
                    }
                }
            }
        });
    }

    let base_capacity = state.settlers_base_capacity as usize;
    let settlers_per_house = state.settlers_per_house as usize;
    let houses_len = state.houses.len();
    let settlers_capacity_limit =
        base_capacity.saturating_add(houses_len.saturating_mul(settlers_per_house));
    let capacity_limit = if settlers_capacity_limit == 0 {
        None
    } else {
        Some(settlers_capacity_limit)
    };
    let min_lifespan = state.settler_min_lifespan_ms;
    let max_lifespan = state.settler_max_lifespan_ms;
    let mut new_settlers = Vec::new();
    let mut next_settler_id = state.next_settler_id;

    for house in &mut state.houses {
        if let Some(limit) = capacity_limit {
            if alive_total >= limit {
                break;
            }
        }

        if now - house.last_spawn_ms >= 5_000.0 {
            let (spawn_x, spawn_y) = (house.x, house.y);
            let id = next_settler_id;
            next_settler_id = next_settler_id.saturating_add(1);
            let lifespan = random_range(min_lifespan, max_lifespan);
            new_settlers.push(SettlerState::new(id, spawn_x, spawn_y, now, lifespan));
            house.last_spawn_ms = now;
            alive_total = alive_total.saturating_add(1);
        }
    }

    state.next_settler_id = next_settler_id;

    if !new_settlers.is_empty() {
        state.settlers.extend(new_settlers);
    }

    render_houses(context, state, now);
    alive_count.set(alive_total);
}
