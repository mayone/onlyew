mod pagination;

use pagination::Pagination;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let total_pages = 6;
    let current_page = use_state(|| if total_pages > 0 { 1 } else { 0 });
    let on_change = {
        let current_page = current_page.clone();
        Callback::from(move |page: usize| current_page.set(page))
    };

    html! {
        <>
            <div>{ format!("Page {} of {}", *current_page, total_pages) }</div>
            <Pagination total_pages={total_pages} on_change={on_change} />
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Starting Yew app");
    yew::Renderer::<App>::new().render();
}
