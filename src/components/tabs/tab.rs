use yew::prelude::*;

/// Properties for the [`Tab`].
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct TabProperties {
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub value: Option<usize>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub is_selected: bool,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<AttrValue>,
    #[prop_or_default]
    pub on_click: Callback<usize>,
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
            node_ref,
            children,
            value,
            disabled,
            is_selected,
            class,
            style,
            on_click,
            ..
        } = ctx.props();

        html! {
            <button
                ref={node_ref}
                disabled={*disabled}
                class={classes!("tab", is_selected.then_some("selected"), disabled.then_some("disabled"), class.clone())}
                {style}
                onclick={let on_click = on_click.clone();
                    let value = value.clone();
                    Callback::from(move |_| on_click.emit(value.unwrap().clone()))}
            >
                { children.clone() }
            </button>
        }
    }
}
