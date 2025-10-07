use web_sys::SvgsvgElement;
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

#[derive(Clone, PartialEq)]
struct Settler {
    x: f64,
    y: f64,
}

#[function_component]
fn App() -> Html {
    let settlers = use_state(|| Vec::<Settler>::new());
    let svg_ref = use_node_ref();

    let handle_click = {
        let settlers = settlers.clone();
        let svg_ref = svg_ref.clone();

        Callback::from(move |event: MouseEvent| {
            if let Some(svg) = svg_ref.cast::<SvgsvgElement>() {
                let rect = svg.get_bounding_client_rect();
                let width = rect.width();
                let height = rect.height();

                if width == 0_f64 || height == 0_f64 {
                    return;
                }

                let client_x = event.client_x() as f64;
                let client_y = event.client_y() as f64;
                let svg_x = (client_x - rect.left()) * (VIEWBOX_WIDTH / width);
                let svg_y = (client_y - rect.top()) * (VIEWBOX_HEIGHT / height);
                let dx = svg_x - PLANET_CENTER_X;
                let dy = svg_y - PLANET_CENTER_Y;

                if dx * dx + dy * dy <= PLANET_RADIUS * PLANET_RADIUS {
                    let mut next = (*settlers).clone();
                    next.push(Settler { x: svg_x, y: svg_y });
                    settlers.set(next);
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
                <svg
                    viewBox={format!("0 0 {} {}", VIEWBOX_WIDTH, VIEWBOX_HEIGHT)}
                    style="width: min(80vw, 540px); height: auto;"
                    role="img"
                    aria-hidden="true"
                    ref={svg_ref}
                    onclick={handle_click}
                >
                    <circle
                        cx={PLANET_CENTER_X.to_string()}
                        cy={PLANET_CENTER_Y.to_string()}
                        r={PLANET_RADIUS.to_string()}
                        fill={ORBIT_05}
                    />
                    {for settlers.iter().enumerate().map(|(idx, settler)| {
                        html! {
                            <circle
                                key={idx.to_string()}
                                cx={settler.x.to_string()}
                                cy={settler.y.to_string()}
                                r={SETTLER_RADIUS.to_string()}
                                fill={ORBIT_04}
                                opacity="0.9"
                            />
                        }
                    })}
                </svg>
            </section>
        </main>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
