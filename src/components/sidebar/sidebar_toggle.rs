use yew::prelude::*;

use crate::contexts::{SidebarAction, SidebarContext};

/// Properties for the [`SidebarToggle`].
#[derive(Debug, PartialEq, Properties)]
pub struct SidebarToggleProperties {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: AttrValue,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// A component to represent a toggle button for [`Sidebar`] component.
#[derive(Debug)]
pub struct SidebarToggle;

impl Component for SidebarToggle {
    type Message = ();
    type Properties = SidebarToggleProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let (sidebar_context, _) = ctx
            .link()
            .context::<SidebarContext>(Callback::noop())
            .expect("No sidebar context provided");

        let Self::Properties {
            children,
            class,
            style,
            ..
        } = ctx.props();

        let on_click = { Callback::from(move |_| sidebar_context.dispatch(SidebarAction::Toggle)) };

        html! { <button {class} {style} onclick={on_click}>{ children.clone() }</button> }
    }
}

#[cfg(test)]
mod tests {
    use tailwind_fuse::tw_merge;

    use super::*;

    #[test]
    fn html_with_all_props() {
        let _ = html! {
            <SidebarToggle
                class={tw_merge!("text-black", "text-white")}
                style="background-color: gray"
            >
                { "Toggle" }
            </SidebarToggle>
        };
    }
}
