use yew::prelude::*;

/// The Tab component has the following props:
///
/// Required props:
///
/// - `children`: The children to be rendered inside the tab button.
/// - `panel`: The content to be rendered in the panel when this tab is selected.
///
/// Optional props:
///
/// - `disabled`: Whether the tab is disabled.
/// - `class`: `yew::Classes`
/// - `style`: The style attribute.
#[derive(Debug, PartialEq, Properties)]
pub struct TabProperties {
    #[prop_or_default]
    pub children: Children,
    pub panel: Html,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// A component to represent a single tab in a Tabs component.
///
/// Usage:
/// ```ignore
/// html! {
///     <Tabs>
///         <Tab panel={html!{<div>{"Panel 1"}</div>}}>{"Tab 1"}</Tab>
///         <Tab panel={html!{<div>{"Panel 2"}</div>}}>{"Tab 2"}</Tab>
///     </Tabs>
/// }
/// ```
#[derive(Debug)]
pub struct Tab;

impl Component for Tab {
    type Message = ();
    type Properties = TabProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self::Properties {
            children,
            disabled,
            class,
            style,
            ..
        } = ctx.props();

        html! {
            <div class={classes!("tab", class.clone(), disabled.then_some("disabled"))} {style}>
                { children.clone() }
            </div>
        }
    }
}
