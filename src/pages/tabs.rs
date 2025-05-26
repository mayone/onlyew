use crate::components::{Tab, Tabs};
use yew::prelude::*;

#[function_component(TabsPage)]
pub fn tabs() -> Html {
    html! {
        <div style="display: flex; flex-direction: column; gap: 20px; padding: 20px">
            <h1>{"Tabs Showcase"}</h1>
            <div style="display: flex; flex-direction: column; gap: 20px">
                <Tabs
                    default_tab={1}
                    on_change={Callback::from(|index| log::info!("Tab changed to: {}", index))}
                >
                    <Tab panel={html!{<div>{"Dandelion"}</div>}}>{"Dandelion"}</Tab>
                    <Tab panel={html!{<div>{"Heather"}</div>}}>{"Heather"}</Tab>
                    <Tab panel={html!{<div>{"Lavender"}</div>}}>{"Lavender"}</Tab>
                    <Tab panel={html!{<div>{"Lilac"}</div>}}>{"Lilac"}</Tab>
                    <Tab panel={html!{<div>{"Marigold"}</div>}}>{"Marigold"}</Tab>
                    <Tab panel={html!{<div>{"Narcissus"}</div>}}>{"Narcissus"}</Tab>
                    <Tab panel={html!{<div>{"Orchid"}</div>}}>{"Orchid"}</Tab>
                    <Tab disabled={true} panel={html!{<div>{"Poppy"}</div>}}>{"Poppy"}</Tab>
                    <Tab panel={html!{<div>{"Rose"}</div>}}>{"Rose"}</Tab>
                </Tabs>
            </div>
        </div>
    }
}
