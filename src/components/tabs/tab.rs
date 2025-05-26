use yew::prelude::*;

/// The Tab component has the following props:
///
/// Required props:
///
/// - `children`: The children to be rendered inside.
///
/// Optional props:
///
/// - `class`: `yew::Classes`
/// - `style`: The style attribute.
#[derive(Debug, PartialEq, Properties)]
pub struct TabProperties {
    #[prop_or_default]
    pub children: Children,
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
///         <Tab>{"Tab 1"}</Tab>
///         <Tab>{"Tab 2"}</Tab>
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
        } = ctx.props();

        html! {
            <div class={classes!("tab", class.clone())} {style}>
                { children.clone() }
            </div>
        }
    }
}
