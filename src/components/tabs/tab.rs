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
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[derive(Debug)]
pub enum TabMessage {
    ContextUpdated(TabsContext),
}

/// A component to represent a single tab in a TabList component.
#[derive(Debug)]
pub struct Tab {
    tabs_context: TabsContext,
    _ctx_handle: ContextHandle<TabsContext>,
}

impl Component for Tab {
    type Message = TabMessage;
    type Properties = TabProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let (tabs_context, ctx_handle) = ctx
            .link()
            .context::<TabsContext>(ctx.link().callback(Self::Message::ContextUpdated))
            .expect("No tabs context provided");

        Self {
            tabs_context,
            _ctx_handle: ctx_handle,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::ContextUpdated(new_ctx) => {
                self.tabs_context = new_ctx;

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self::Properties {
            value,
            children,
            disabled,
            node_ref,
            class,
            style,
            ..
        } = ctx.props();

        let is_selected = self.tabs_context.selected_tab == value.clone();

        html! {
            <button
                ref={node_ref}
                disabled={*disabled}
                class={classes!("tab", is_selected.then_some("selected"), disabled.then_some("disabled"), class.clone())}
                {style}
                onclick={let value = value.clone();
                    let tabs_context = self.tabs_context.clone();
                    Callback::from(move |_| {
                        if !is_selected {
                            tabs_context.dispatch(TabsAction::Select(value.clone()));
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
    fn test_render_tab() {
        let _ = html! { <Tab value="1">{ "Tab 1" }</Tab> };
    }
}
