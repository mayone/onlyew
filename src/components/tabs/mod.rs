use yew::prelude::*;

mod tab;
mod tab_list;
mod tab_panel;

pub use tab::Tab;
pub use tab_list::TabList;
pub use tab_panel::TabPanel;

use crate::contexts::TabsProvider;

/// Properties for the [`Tabs`].
#[derive(Debug, PartialEq, Properties)]
pub struct TabsProperties {
    #[prop_or_default]
    pub children: Children,
    /// The index of the default tab to be selected.
    #[prop_or_default]
    pub default_tab: Option<AttrValue>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<AttrValue>,
    /// A callback function that is called when the selected tab changes.
    #[prop_or_default]
    pub on_change: Callback<AttrValue>,
}

/// A component to display tabs.
///
/// It has the following purposes:
///
/// - To provide the tabs in a Tabs component.
///
/// Usage:
/// ```ignore
///
/// html! {
///     <Tabs on_change={Callback::from(|index| log::info!("Tab changed to: {}", index))}>
///         <Tab panel={html!{<div>{"Panel 1"}</div>}}>{"Tab 1"}</Tab>
///         <Tab panel={html!{<div>{"Panel 2"}</div>}}>{"Tab 2"}</Tab>
///     </Tabs>
/// }
/// ```
#[derive(Debug)]
pub struct Tabs;

impl Component for Tabs {
    type Message = ();
    type Properties = TabsProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self::Properties {
            children,
            default_tab,
            class,
            style,
            ..
        } = ctx.props();

        html! {
            <TabsProvider default_tab={default_tab.clone()}>
                <div class={classes!("tabs", class.clone())} {style}>{ children.clone() }</div>
            </TabsProvider>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_tabs() {
        let _ = html! {
            <Tabs>
                <TabList>
                    <Tab>{ "Tab 1" }</Tab>
                    <Tab>{ "Tab 2" }</Tab>
                </TabList>
                <TabPanel>
                    <div>{ "TabPanel 1" }</div>
                </TabPanel>
                <TabPanel>
                    <div>{ "TabPanel 2" }</div>
                </TabPanel>
            </Tabs>
        };
    }
}
