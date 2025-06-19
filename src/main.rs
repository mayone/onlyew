mod components;
mod contexts;
mod pages;
mod route;

use yew::prelude::*;
use yew_router::prelude::*;

use route::Route;
use route::switch;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <div id="modal-root" />
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Starting Yew app");
    yew::Renderer::<App>::new().render();
}
