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
    /// The value of the default tab to be selected.
    #[prop_or_default]
    pub default_value: Option<AttrValue>,
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
///     <Tabs on_change={Callback::from(|value| log::info!("Tab changed to: {}", value))}>
///         <TabList>
///             <Tab value="1">{"Tab 1"}</Tab>
///             <Tab value="2">{"Tab 2"}</Tab>
///         </TabList>
///         <TabPanel value="1">
///             <div>{ "TabPanel 1" }</div>
///         </TabPanel>
///         <TabPanel value="2">
///             <div>{ "TabPanel 2" }</div>
///         </TabPanel>
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
            default_value,
            class,
            style,
            on_change,
            ..
        } = ctx.props();

        html! {
            <TabsProvider {default_value} {on_change}>
                <div class={classes!("tabs", class.clone())} {style}>{ children.clone() }</div>
            </TabsProvider>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_tabs() {
        let _ = html! {
            <Tabs>
                <TabList>
                    <Tab value="1">{ "Tab 1" }</Tab>
                    <Tab value="2">{ "Tab 2" }</Tab>
                </TabList>
                <TabPanel value="1">
                    <div>{ "TabPanel 1" }</div>
                </TabPanel>
                <TabPanel value="2">
                    <div>{ "TabPanel 2" }</div>
                </TabPanel>
            </Tabs>
        };
    }
}
