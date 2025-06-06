use std::{
    any::{Any, TypeId},
    rc::Rc,
};

use yew::prelude::*;

mod tab;
mod tab_list;
mod tab_panel;

pub use tab::Tab;
pub use tab_list::TabList;
pub use tab_panel::TabPanel;

// use crate::contexts::TabsContext;

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

#[derive(Debug)]
pub enum TabsMessage {
    Select(AttrValue),
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
pub struct Tabs {
    selected_tab: AttrValue,
}

impl Component for Tabs {
    type Message = TabsMessage;
    type Properties = TabsProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let selected_tab = ctx.props().default_tab.clone().unwrap_or("0".into());

        Self { selected_tab }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TabsMessage::Select(value) => {
                if self.selected_tab != value {
                    self.selected_tab = value.clone();
                    ctx.props().on_change.emit(value);
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self::Properties {
            children,
            class,
            style,
            ..
        } = ctx.props();

        // let (tabs_ctx, _) = ctx
        //     .link()
        //     .context::<TabsContext>(Callback::noop())
        //     .expect("No tabs context provided");

        let tab_list = children
            .iter()
            .filter(|child| child.type_id() == TypeId::of::<TabList>())
            .map(|mut tab_list| {
                let on_select = ctx.link().callback(TabsMessage::Select);
                let props = Rc::make_mut(&mut tab_list.props);
                props.selected_tab = self.selected_tab.clone();
                props.on_select = on_select;

                tab_list
            });

        let children = children
            .iter()
            .filter(|child| child.type_id() == TypeId::of::<Tab>())
            .enumerate()
            .filter(|(index, child)| {
                child
                    .props
                    .value
                    .clone()
                    .unwrap_or(index.to_string().into())
                    == self.selected_tab
            })
            .map(|(_, child)| child);

        html! {
            <div class={classes!("tabs", class.clone())} {style}>
                { tab_list.collect::<Html>() }
                { children.collect::<Html>() }
            </div>
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
