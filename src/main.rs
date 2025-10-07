use gloo_timers::callback::Interval;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::{events::MouseEvent, prelude::*};

const ORBIT_01: &str = "#2A1A11";
const ORBIT_02: &str = "#5C3A27";
const ORBIT_03: &str = "#F8E1C8";
const ORBIT_04: &str = "#F2A7A0";
const ORBIT_05: &str = "#88AFC6";
const VIEWBOX_WIDTH: f64 = 600.0;
const VIEWBOX_HEIGHT: f64 = 400.0;
const PLANET_RADIUS: f64 = 200.0;
const PLANET_CENTER_X: f64 = 300.0;
const PLANET_CENTER_Y: f64 = 200.0;
const SETTLER_RADIUS: f64 = 10.0;
const SETTLER_LIFESPAN_MS: f64 = 5_000.0;
const MOVE_INTERVAL_MS: f64 = 1_000.0;
const MOVE_DISTANCE_MIN: f64 = 12.0;
const MOVE_DISTANCE_MAX: f64 = 45.0;
const FADING_DURATION_MS: f64 = 600.0;

#[derive(Clone, PartialEq)]
enum SettlerPhase {
    Alive,
    Fading { started_ms: f64 },
}

#[derive(Clone, PartialEq)]
struct SettlerState {
    id: u64,
    anchor_x: f64,
    anchor_y: f64,
    target_x: f64,
    target_y: f64,
    move_start_ms: f64,
    last_direction_change_ms: f64,
    birth_ms: f64,
    phase: SettlerPhase,
}

impl SettlerState {
    fn new(id: u64, x: f64, y: f64, now_ms: f64) -> Self {
        Self {
            id,
            anchor_x: x,
            anchor_y: y,
            target_x: x,
            target_y: y,
            move_start_ms: now_ms,
            last_direction_change_ms: now_ms - MOVE_INTERVAL_MS,
            birth_ms: now_ms,
            phase: SettlerPhase::Alive,
        }
    }

    fn position_at(&self, now_ms: f64) -> (f64, f64) {
        let elapsed = (now_ms - self.move_start_ms).max(0.0);
        let progress = (elapsed / MOVE_INTERVAL_MS).clamp(0.0, 1.0);
        let eased = ease_out_quad(progress);
        let x = self.anchor_x + (self.target_x - self.anchor_x) * eased;
        let y = self.anchor_y + (self.target_y - self.anchor_y) * eased;
        (x, y)
    }
}

fn ease_out_quad(t: f64) -> f64 {
    1.0 - (1.0 - t).powi(2)
}

fn current_time_ms() -> f64 {
    js_sys::Date::now()
}

fn point_within_planet(x: f64, y: f64) -> bool {
    let dx = x - PLANET_CENTER_X;
    let dy = y - PLANET_CENTER_Y;
    (dx * dx + dy * dy).sqrt() <= (PLANET_RADIUS - SETTLER_RADIUS)
}

fn random_range(min: f64, max: f64) -> f64 {
    let normalized = js_sys::Math::random();
    min + normalized * (max - min)
}

fn random_angle() -> f64 {
    random_range(0.0, std::f64::consts::TAU)
}

fn random_target_near(x: f64, y: f64) -> (f64, f64) {
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

    // Fallback to the planet's center if we couldn't find a suitable nearby spot.
    (PLANET_CENTER_X, PLANET_CENTER_Y)
}

#[function_component]
fn App() -> Html {
    let settlers = use_mut_ref(|| Vec::<SettlerState>::new());
    let next_id = use_state(|| 0_u64);
    let alive_count = use_state(|| 0_usize);
    let canvas_ref = use_node_ref();

    {
        let canvas_ref = canvas_ref.clone();
        let settlers = settlers.clone();
        let alive_count_state = alive_count.clone();

        use_effect_with((), move |_| {
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                canvas.set_width(VIEWBOX_WIDTH as u32);
                canvas.set_height(VIEWBOX_HEIGHT as u32);

                if let Some(context) = canvas
                    .get_context("2d")
                    .ok()
                    .flatten()
                    .and_then(|ctx| ctx.dyn_into::<CanvasRenderingContext2d>().ok())
                {
                    let draw_context = context.clone();
                    let settlers_handle = settlers.clone();

                    Interval::new(16, move || {
                        let now = current_time_ms();
                        draw_context.clear_rect(0.0, 0.0, VIEWBOX_WIDTH, VIEWBOX_HEIGHT);

                        draw_context.set_fill_style_str(ORBIT_05);
                        draw_context.begin_path();
                        let _ = draw_context.arc(
                            PLANET_CENTER_X,
                            PLANET_CENTER_Y,
                            PLANET_RADIUS,
                            0.0,
                            std::f64::consts::TAU,
                        );
                        draw_context.fill();

                        let mut settlers_vec = settlers_handle.borrow_mut();
                        let mut alive_total = 0_usize;
                        settlers_vec.retain_mut(|settler| {
                            let (current_x, current_y) = settler.position_at(now);

                            if matches!(settler.phase, SettlerPhase::Alive)
                                && now - settler.birth_ms >= SETTLER_LIFESPAN_MS
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
                                        let (target_x, target_y) =
                                            random_target_near(current_x, current_y);
                                        settler.anchor_x = current_x;
                                        settler.anchor_y = current_y;
                                        settler.target_x = target_x;
                                        settler.target_y = target_y;
                                        settler.move_start_ms = now;
                                        settler.last_direction_change_ms = now;
                                    }

                                    draw_context.set_global_alpha(0.92);
                                    draw_context.set_fill_style_str(ORBIT_04);
                                    draw_context.begin_path();
                                    let _ = draw_context.arc(
                                        current_x,
                                        current_y,
                                        SETTLER_RADIUS,
                                        0.0,
                                        std::f64::consts::TAU,
                                    );
                                    draw_context.fill();
                                    draw_context.set_global_alpha(1.0);
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

                                        draw_context.set_global_alpha(opacity);
                                        draw_context.set_fill_style_str(ORBIT_02);
                                        draw_context.begin_path();
                                        let _ = draw_context.arc(
                                            current_x,
                                            current_y,
                                            radius,
                                            0.0,
                                            std::f64::consts::TAU,
                                        );
                                        draw_context.fill();
                                        draw_context.set_global_alpha(1.0);
                                        true
                                    }
                                }
                            }
                        });

                        alive_count_state.set(alive_total);
                    })
                    .forget();
                }
            }

            || ()
        });
    }

    let handle_click = {
        let settlers = settlers.clone();
        let canvas_ref = canvas_ref.clone();
        let next_id = next_id.clone();

        Callback::from(move |event: MouseEvent| {
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                let rect = canvas.get_bounding_client_rect();
                let width = rect.width();
                let height = rect.height();

                if width == 0.0 || height == 0.0 {
                    return;
                }

                let client_x = event.client_x() as f64;
                let client_y = event.client_y() as f64;
                let scale_x = (canvas.width() as f64) / width;
                let scale_y = (canvas.height() as f64) / height;
                let canvas_x = (client_x - rect.left()) * scale_x;
                let canvas_y = (client_y - rect.top()) * scale_y;

                if point_within_planet(canvas_x, canvas_y) {
                    let id = *next_id;
                    next_id.set(id + 1);
                    let now = current_time_ms();
                    settlers.borrow_mut().push(SettlerState::new(id, canvas_x, canvas_y, now));
                }
            }
        })
    };

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
                <h1
                    style={format!(
                        "color: {}; font-family: Orbitron, 'Trebuchet MS', sans-serif; font-size: clamp(2.5rem, 4vw, 3.5rem); letter-spacing: 0.12em; text-transform: uppercase; margin: 0;",
                        ORBIT_03
                    )}
                >
                    {"Your Planet"}
                </h1>
                <canvas
                    ref={canvas_ref}
                    width={VIEWBOX_WIDTH.to_string()}
                    height={VIEWBOX_HEIGHT.to_string()}
                    style="width: min(80vw, 540px); height: auto; max-width: 600px; cursor: pointer; touch-action: manipulation;"
                    onclick={handle_click}
                >
                    {"Your browser does not support HTML canvas."}
                </canvas>
                <div
                    style={format!(
                        "margin-top: 1.5rem; padding: 0.75rem 1.25rem; border: 1px solid {}; border-radius: 0.75rem; background-color: rgba(0,0,0,0.25); color: {}; font-size: clamp(1rem, 2vw, 1.15rem); letter-spacing: 0.05em;",
                        ORBIT_02,
                        ORBIT_03
                    )}
                >
                    {format!("Settlers alive: {}", *alive_count)}
                </div>
            </section>
        </main>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
