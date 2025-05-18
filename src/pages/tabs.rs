use crate::components::tabs::Tabs;
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
                    <span>{"Dandelion"}</span>
                    <span>{"Heather"}</span>
                    <span>{"Lavender"}</span>
                    <span>{"Lilac"}</span>
                    <span>{"Marigold"}</span>
                    <span>{"Narcissus"}</span>
                    <span>{"Orchid"}</span>
                    <span>{"Poppy"}</span>
                    <span>{"Rose"}</span>
                </Tabs>
            </div>
        </div>
    }
}
