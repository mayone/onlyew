use tailwind_fuse::tw_merge;
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
    pub class: AttrValue,
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

        let on_click = {
            let value = value.clone();
            let is_selected = *is_selected;
            let tabs_context = tabs_context.clone();

            Callback::from(move |_| {
                if !is_selected {
                    tabs_context
                        .state
                        .dispatch(TabsAction::Select(value.clone()));
                    tabs_context.on_change.emit(value.clone());
                }
            })
        };

        html! {
            <button
                ref={node_ref}
                disabled={*disabled}
                class={tw_merge!("border-none py-4 px-6 bg-transparent text-white/60 cursor-pointer transition-colors duration-300 hover:bg-neutral-200/10 active:bg-neutral-200/20", is_selected.then_some("text-white"), disabled.then_some("bg-transparent text-white/30 cursor-default"), class.as_ref())}
                {style}
                onclick={on_click}
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
    fn html_with_all_props() {
        let _ = html! {
            <Tab
                value="1"
                disabled=false
                is_selected=true
                node_ref={NodeRef::default()}
                class={tw_merge!("text-black", "text-white")}
                style="background-color: gray"
            >
                { "Tab 1" }
            </Tab>
        };
    }
}
