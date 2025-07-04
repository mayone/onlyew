use yew::prelude::*;

/// Properties for the [`SidebarContent`].
#[derive(Debug, PartialEq, Properties)]
pub struct SidebarContentProperties {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[derive(Debug)]
pub struct SidebarContent;

impl Component for SidebarContent {
    type Message = ();
    type Properties = SidebarContentProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self::Properties {
            children,
            class,
            style,
        } = ctx.props();

        html! {
            <div
                class={classes!("sidebar-content",
                    class.clone()
                )}
                {style}
            >
                { children.clone() }
            </div>
        }
    }
}
