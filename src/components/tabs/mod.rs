use std::rc::Rc;

use yew::prelude::*;

mod tab;
mod tab_list;
mod tab_panel;

pub use tab::Tab;
pub use tab_list::TabList;
pub use tab_panel::TabPanel;

/// Properties for the [`Tabs`].
#[derive(Debug, PartialEq, Properties)]
pub struct TabsProperties {
    #[prop_or_default]
    pub children: ChildrenWithProps<TabPanel>,
    pub tab_list: ChildrenWithProps<TabList>,
    /// The index of the default tab to be selected.
    #[prop_or_default]
    pub default_tab: Option<usize>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<AttrValue>,
    /// A callback function that is called when the selected tab changes.
    #[prop_or_default]
    pub on_change: Callback<usize>,
}

#[derive(Debug)]
pub enum TabsMessage {
    Select(usize),
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
    selected_tab: usize,
}

impl Component for Tabs {
    type Message = TabsMessage;
    type Properties = TabsProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let selected_tab = ctx.props().default_tab.unwrap_or(0);

        Self { selected_tab }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TabsMessage::Select(value) => {
                if self.selected_tab != value {
                    self.selected_tab = value;
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
            tab_list,
            children,
            class,
            style,
            ..
        } = ctx.props();

        let tab_list = tab_list.iter().map(|mut tab_list| {
            let on_select = ctx.link().callback(TabsMessage::Select);
            let props = Rc::make_mut(&mut tab_list.props);
            props.selected_tab = self.selected_tab;
            props.on_select = on_select;

            tab_list
        });

        let children = children
            .iter()
            .enumerate()
            .filter(|(index, child)| child.props.value.unwrap_or(*index) == self.selected_tab)
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
            <Tabs
                tab_list={html_nested!{
                    <TabList>
                        <Tab>{ "Tab 1" }</Tab>
                        <Tab>{ "Tab 2" }</Tab>
                    </TabList>
                }}
            />
        };
    }
}
