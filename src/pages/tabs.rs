use yew::prelude::*;
use yew_router::prelude::Link;

use crate::{
    components::{Tab, TabList, TabPanel, Tabs},
    route::Route,
};

#[function_component(TabsPage)]
pub fn tabs() -> Html {
    html! {
        <div style="display: flex; flex-direction: column; gap: 20px; padding: 20px">
            <h1>{ "Tabs Showcase" }</h1>
            <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
            <div style="display: flex; flex-direction: column; gap: 20px">
                <Tabs
                    default_tab="723"
                    on_change={Callback::from(|value| log::info!("Tab changed to: {}", value))}
                >
                    <TabList>
                        <Tab>{ "Dandelion" }</Tab>
                        <Tab value="723">{ "Heather" }</Tab>
                        <Tab>{ "Lavender" }</Tab>
                        <Tab>{ "Lilac" }</Tab>
                        <Tab>{ "Marigold" }</Tab>
                        <Tab>{ "Narcissus" }</Tab>
                        <Tab>{ "Orchid" }</Tab>
                        <Tab disabled=true>{ "Poppy" }</Tab>
                        <Tab>{ "Rose" }</Tab>
                    </TabList>
                    <TabPanel>
                        <div>{ "Dandelion" }</div>
                    </TabPanel>
                    <TabPanel value="723">
                        <div>{ "Heather" }</div>
                    </TabPanel>
                    <TabPanel>
                        <div>{ "Lavender" }</div>
                    </TabPanel>
                    <TabPanel>
                        <div>{ "Lilac" }</div>
                    </TabPanel>
                    <TabPanel>
                        <div>{ "Marigold" }</div>
                    </TabPanel>
                    <TabPanel>
                        <div>{ "Narcissus" }</div>
                    </TabPanel>
                    <TabPanel>
                        <div>{ "Orchid" }</div>
                    </TabPanel>
                    <TabPanel value="723">
                        <div>{ "Poppy" }</div>
                    </TabPanel>
                    // <TabPanel>
                    //     <div>{ "Rose" }</div>
                    // </TabPanel>
                </Tabs>
            </div>
        </div>
    }
}
