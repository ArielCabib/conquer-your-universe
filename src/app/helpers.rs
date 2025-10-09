use crate::{
    constants::*,
    types::{GameState, HouseState},
};
use web_sys::CanvasRenderingContext2d;

pub fn ease_out_quad(t: f64) -> f64 {
    1.0 - (1.0 - t).powi(2)
}

pub fn current_time_ms() -> f64 {
    js_sys::Date::now()
}

pub fn point_within_planet(x: f64, y: f64) -> bool {
    let dx = x - PLANET_CENTER_X;
    let dy = y - PLANET_CENTER_Y;
    (dx * dx + dy * dy).sqrt() <= (PLANET_RADIUS - SETTLER_RADIUS)
}

pub fn random_range(min: f64, max: f64) -> f64 {
    let normalized = js_sys::Math::random();
    min + normalized * (max - min)
}

pub fn random_angle() -> f64 {
    random_range(0.0, std::f64::consts::TAU)
}

pub fn random_target_near(x: f64, y: f64) -> (f64, f64) {
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

    (PLANET_CENTER_X, PLANET_CENTER_Y)
}

pub fn ensure_settler_lifespans(state: &mut GameState) {
    let min_lifespan = state.settler_min_lifespan_ms;
    let max_lifespan = state.settler_max_lifespan_ms;

    for settler in &mut state.settlers {
        if settler.lifespan_ms <= 0.0 {
            settler.lifespan_ms = random_range(min_lifespan, max_lifespan);
        }
    }
}

pub fn ensure_house_registry(state: &mut GameState) {
    if let Some(highest_id) = state.houses.iter().map(|house| house.id).max() {
        let next_id = highest_id.saturating_add(1);
        if state.next_house_id <= highest_id {
            state.next_house_id = next_id;
        }
    }
}

pub fn draw_house(context: &CanvasRenderingContext2d, house: &HouseState, now_ms: f64) {
    let spawn_elapsed = (now_ms - house.last_spawn_ms).max(0.0);
    let highlight_factor = if spawn_elapsed < HOUSE_SPAWN_ANIMATION_MS {
        1.0 - ease_out_quad((spawn_elapsed / HOUSE_SPAWN_ANIMATION_MS).clamp(0.0, 1.0))
    } else {
        0.0
    };

    if highlight_factor > 0.0 {
        let halo_radius = 36.0 + 18.0 * highlight_factor;
        context.set_global_alpha(0.45 * highlight_factor);
        context.set_fill_style_str(ORBIT_04);
        context.begin_path();
        let _ = context.arc(
            house.x,
            house.y + 4.0,
            halo_radius,
            0.0,
            std::f64::consts::TAU,
        );
        context.fill();
        context.set_global_alpha(1.0);
    }

    let base_width = 28.0;
    let base_height = 18.0;
    let roof_height = 14.0;

    let base_x = house.x - base_width / 2.0;
    let base_y = house.y - base_height / 2.0;

    let _ = context.save();
    context.set_fill_style_str(ORBIT_01);
    context.fill_rect(base_x, base_y, base_width, base_height);

    if highlight_factor > 0.0 {
        context.set_global_alpha(0.35 * highlight_factor + 0.2);
        context.set_fill_style_str(ORBIT_05);
        context.fill_rect(base_x, base_y, base_width, base_height);
        context.set_global_alpha(1.0);
    }

    context.set_fill_style_str(ORBIT_02);
    context.begin_path();
    context.move_to(base_x - 2.0, base_y);
    context.line_to(house.x, base_y - roof_height);
    context.line_to(base_x + base_width + 2.0, base_y);
    context.close_path();
    context.fill();

    if highlight_factor > 0.0 {
        context.set_global_alpha(0.45 * highlight_factor + 0.15);
        context.begin_path();
        context.move_to(base_x - 2.0, base_y);
        context.line_to(house.x, base_y - roof_height - 4.0 * highlight_factor);
        context.line_to(base_x + base_width + 2.0, base_y);
        context.close_path();
        context.set_fill_style_str(ORBIT_03);
        context.fill();
        context.set_global_alpha(1.0);
    }

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

pub fn random_target_for_settler(x: f64, y: f64) -> (f64, f64) {
    random_target_near(x, y)
}
