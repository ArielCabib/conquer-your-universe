use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {}
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
