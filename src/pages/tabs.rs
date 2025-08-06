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
                    default_value="723"
                    on_change={Callback::from(|value| log::info!("Tab changed to: {value}"))}
                >
                    <TabList>
                        <Tab value="1">{ "Dandelion" }</Tab>
                        <Tab value="723">{ "Heather" }</Tab>
                        <Tab value="3">{ "Lavender" }</Tab>
                        <Tab value="4">{ "Lilac" }</Tab>
                        <Tab value="5">{ "Marigold" }</Tab>
                        <Tab value="6">{ "Narcissus" }</Tab>
                        <Tab value="7">{ "Orchid" }</Tab>
                        <Tab value="8" disabled=true>{ "Poppy" }</Tab>
                        <Tab value="9">{ "Rose" }</Tab>
                        <Tab value="9">{ "Rose" }</Tab>
                        <Tab value="9">{ "Rose" }</Tab>
                        <Tab value="9">{ "Rose" }</Tab>
                        <Tab value="9">{ "Rose" }</Tab>
                    </TabList>
                    <TabPanel value="1">
                        <div>{ "Dandelion" }</div>
                    </TabPanel>
                    <TabPanel value="723">
                        <div>{ "Heather" }</div>
                    </TabPanel>
                    <TabPanel value="3">
                        <div>{ "Lavender" }</div>
                    </TabPanel>
                    <TabPanel value="4">
                        <div>{ "Lilac" }</div>
                    </TabPanel>
                    <TabPanel value="5">
                        <div>{ "Marigold" }</div>
                    </TabPanel>
                    <TabPanel value="6">
                        <div>{ "Narcissus" }</div>
                    </TabPanel>
                    // <TabPanel value="7">
                    //     <div>{ "Orchid" }</div>
                    // </TabPanel>
                    <TabPanel value="723">
                        <div>{ "Poppy" }</div>
                    </TabPanel>
                    <TabPanel value="9">
                        <div>{ "Rose" }</div>
                    </TabPanel>
                </Tabs>
            </div>
        </div>
    }
}
