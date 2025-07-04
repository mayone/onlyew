use yew::prelude::*;

/// Properties for the [`SidebarHeader`].
#[derive(Debug, PartialEq, Properties)]
pub struct SidebarHeaderProperties {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[derive(Debug)]
pub struct SidebarHeader;

impl Component for SidebarHeader {
    type Message = ();
    type Properties = SidebarHeaderProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self::Properties {
            children,
            class,
            style,
        } = ctx.props();

        html! { <div class={classes!(class.clone())} {style}>{ children.clone() }</div> }
    }
}
