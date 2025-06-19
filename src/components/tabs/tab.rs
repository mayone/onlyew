use yew::prelude::*;

use crate::contexts::{TabsAction, TabsContext};

/// Properties for the [`Tab`].
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct TabProperties {
    pub value: AttrValue,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub is_selected: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// A component to represent a single tab in a [`TabList`] component.
#[derive(Debug)]
pub struct Tab;

impl Component for Tab {
    type Message = ();
    type Properties = TabProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let (tabs_context, _) = ctx
            .link()
            .context::<TabsContext>(Callback::noop())
            .expect("No tabs context provided");

        let Self::Properties {
            value,
            children,
            disabled,
            is_selected,
            node_ref,
            class,
            style,
            ..
        } = ctx.props();

        html! {
            <button
                ref={node_ref}
                disabled={*disabled}
                class={classes!("tab", is_selected.then_some("selected"), disabled.then_some("disabled"), class.clone())}
                {style}
                onclick={let value = value.clone();
                    let is_selected = *is_selected;
                    let tabs_context = tabs_context.clone();
                    Callback::from(move |_| {
                        if !is_selected {
                            tabs_context.state.dispatch(TabsAction::Select(value.clone()));
                            tabs_context.on_change.emit(value.clone());
                        }
                    })}
            >
                { children.clone() }
            </button>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_tab() {
        let _ = html! { <Tab value="1">{ "Tab 1" }</Tab> };
    }
}
