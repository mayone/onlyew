use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::dialog::DialogPage;
use crate::pages::home::Home;
use crate::pages::pagination::PaginationPage;
use crate::pages::segmented_control::SegmentedControlPage;
use crate::pages::tabs::TabsPage;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/pagination")]
    Pagination,
    #[at("/dialog")]
    Dialog,
    #[at("/tabs")]
    Tabs,
    #[at("/segmented_control")]
    SegmentedControl,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Pagination => html! { <PaginationPage /> },
        Route::Dialog => html! { <DialogPage /> },
        Route::Tabs => html! { <TabsPage /> },
        Route::SegmentedControl => html! { <SegmentedControlPage /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
