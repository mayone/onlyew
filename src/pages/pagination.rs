use crate::components::pagination::Pagination;
use yew::prelude::*;

#[function_component(PaginationPage)]
pub fn pagination() -> Html {
    let total_pages = 10;
    let current_page = use_state(|| if total_pages > 0 { 1 } else { 0 });
    let on_change = {
        let current_page = current_page.clone();
        Callback::from(move |page: usize| current_page.set(page))
    };
    html! {
        <div style="display: flex; flex-direction: column; gap: 20px; padding: 20px;">
            <h1>{"Pagination Showcase"}</h1>
            <div style="display: flex; flex-direction: column; gap: 20px">
                <Pagination total_pages=0 />
                <Pagination total_pages=1 />
                <Pagination total_pages=3 />
                <Pagination total_pages=5 />
                <Pagination total_pages={total_pages} on_change={on_change} />
                <div>{ format!("Page {} of {}", *current_page, total_pages) }</div>
            </div>
        </div>
    }
}
