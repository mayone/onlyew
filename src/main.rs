use yew::prelude::*;

use pagination::Pagination;
mod pagination;

#[function_component(App)]
fn app() -> Html {
    let on_change = Callback::from(|page: usize| {
        log::info!("Page changed to: {}", page);
    });

    html! {
        <>
            <Pagination total_pages={6} on_change={on_change} />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
