use yew::prelude::*;
use yew_router::prelude::Link;

use crate::route::Route;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <div style="display: flex; flex-direction: column; gap: 20px; padding: 20px">
                <h1>{ "Home" }</h1>
                <Link<Route> to={Route::Pagination}>{ "Pagination" }</Link<Route>>
                <Link<Route> to={Route::Dialog}>{ "Dialog" }</Link<Route>>
                <Link<Route> to={Route::Tabs}>{ "Tabs" }</Link<Route>>
                <Link<Route> to={Route::SegmentedControl}>{ "Segmented Control" }</Link<Route>>
            </div>
        </div>
    }
}
