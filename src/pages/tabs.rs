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
                    tab_list={html_nested!{<TabList>
                        <Tab>{ "Dandelion" }</Tab>
                        <Tab>{ "Heather" }</Tab>
                        <Tab>{ "Lavender" }</Tab>
                        <Tab>{ "Lilac" }</Tab>
                        <Tab>{ "Marigold" }</Tab>
                        <Tab>{ "Narcissus" }</Tab>
                        <Tab>{ "Orchid" }</Tab>
                        <Tab disabled=true>{ "Poppy" }</Tab>
                        <Tab>{ "Rose" }</Tab>
                    </TabList>}}
                    default_tab=1
                    on_change={Callback::from(|index| log::info!("Tab changed to: {}", index))}
                >
                    <TabPanel>
                        <div>{ "Dandelion" }</div>
                    </TabPanel>
                    <TabPanel>
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
                    <TabPanel>
                        <div>{ "Poppy" }</div>
                    </TabPanel>
                    <TabPanel>
                        <div>{ "Rose" }</div>
                    </TabPanel>
                </Tabs>
            </div>
        </div>
    }
}
