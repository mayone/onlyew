use yew::prelude::*;
use yew_router::prelude::Link;

use crate::{
    components::{ControlItem, SegmentedControl},
    route::Route,
};

#[function_component(SegmentedControlPage)]
pub fn segmented_control() -> Html {
    html! {
        <div style="display: flex; flex-direction: column; gap: 20px; padding: 20px">
            <h1>{ "Segmented Control Showcase" }</h1>
            <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
            <div style="display: flex; flex-direction: column; gap: 20px">
                <SegmentedControl
                    default_value="Heather"
                    on_change={Callback::from(|value| log::info!("Segmented Control changed to: {value}"))}
                >
                    <ControlItem value="Dandelion">{ "Dandelion" }</ControlItem>
                    <ControlItem value="Wayne">{ "Wayne" }</ControlItem>
                    <ControlItem value="Heather">{ "Heather" }</ControlItem>
                </SegmentedControl>
            </div>
        </div>
    }
}
