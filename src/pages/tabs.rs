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
                    <Tab>{"Dandelion"}</Tab>
                    <Tab>{"Heather"}</Tab>
                    <Tab>{"Lavender"}</Tab>
                    <Tab>{"Lilac"}</Tab>
                    <Tab>{"Marigold"}</Tab>
                    <Tab>{"Narcissus"}</Tab>
                    <Tab>{"Orchid"}</Tab>
                    <Tab disabled={true}>{"Poppy"}</Tab>
                    <Tab>{"Rose"}</Tab>
                </Tabs>
            </div>
        </div>
    }
}
