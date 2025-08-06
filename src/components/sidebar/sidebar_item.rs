use yew::prelude::*;

use crate::contexts::SidebarContext;

use super::CollapsedMode;

/// Properties for the [`SidebarItem`].
#[derive(Debug, PartialEq, Properties)]
pub struct SidebarItemProperties {
    pub children: Children,
    #[prop_or_default]
    pub collapsible: CollapsedMode,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[derive(Debug)]
pub struct SidebarItem {
    _ctx_handle: ContextHandle<SidebarContext>,
}

impl Component for SidebarItem {
    type Message = ();
    type Properties = SidebarItemProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let (_, ctx_handle) = ctx
            .link()
            .context::<SidebarContext>(ctx.link().callback(|_| ()))
            .expect("No sidebar context provided");

        Self {
            _ctx_handle: ctx_handle,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let (sidebar_context, _) = ctx
            .link()
            .context::<SidebarContext>(Callback::noop())
            .expect("No sidebar context provided");

        let open = &sidebar_context.state.open;

        let Self::Properties {
            children,
            collapsible,
            class,
            style,
            ..
        } = ctx.props();

        let footer_class = classes!(
            (*collapsible == CollapsedMode::Hidden).then_some("collapsed-hidden"),
            if *open { "expanded" } else { "collapsed" },
            class.clone()
        );

        html! { <div class={footer_class} {style}>{ children.clone() }</div> }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn html_with_all_props() {
        let _ = html! {
            <SidebarItem
                class={classes!("test-class")}
                style="background-color: red"
                collapsible={CollapsedMode::Hidden}
            >
                { "Item" }
            </SidebarItem>
        };
    }
}
