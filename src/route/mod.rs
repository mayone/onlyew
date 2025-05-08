use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::home::Home;
use crate::pages::modal::ModalPage;
use crate::pages::pagination::PaginationPage;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/pagination")]
    Pagination,
    #[at("/modal")]
    Modal,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Pagination => html! { <PaginationPage /> },
        Route::Modal => html! { <ModalPage /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
