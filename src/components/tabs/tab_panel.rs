use yew::prelude::*;

use crate::contexts::TabsContext;

/// Properties for the [`TabPanel`].
#[derive(Debug, PartialEq, Properties)]
pub struct TabPanelProperties {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<AttrValue>,
    #[prop_or_default]
    pub value: Option<AttrValue>,
}

/// A component to display the selected tab's content.
///
/// Usage:
/// ```ignore
/// html! {
///     <TabPanel selected_tab={0}>
///         <Tab panel={html!{<div>{"Panel 1"}</div>}}>{"Tab 1"}</Tab>
///         <Tab panel={html!{<div>{"Panel 2"}</div>}}>{"Tab 2"}</Tab>
///     </TabPanel>
/// }
/// ```
#[derive(Debug)]
pub struct TabPanel;

impl Component for TabPanel {
    type Message = ();
    type Properties = TabPanelProperties;

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let (tabs_context, _) = ctx
            .link()
            .context::<TabsContext>(Callback::noop())
            .expect("No tabs context provided");

        let TabPanelProperties {
            children,
            class,
            style,
            value,
            ..
        } = ctx.props();

        let is_selected = value.clone().unwrap_or("0".into()) == *tabs_context.selected_tab;

        html! {
            <div class={classes!("tab-panel", (!is_selected).then_some("hidden"), class.clone())} {style}>{ children.clone() }</div>
        }
    }
}
