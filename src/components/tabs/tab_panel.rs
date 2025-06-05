use yew::prelude::*;

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

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let TabPanelProperties {
            children,
            class,
            style,
            ..
        } = ctx.props();

        html! {
            <div class={classes!("tab-panel", class.clone())} {style}>{ children.clone() }</div>
        }
    }
}
