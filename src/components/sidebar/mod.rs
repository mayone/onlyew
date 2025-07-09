pub mod sidebar_content;
pub mod sidebar_footer;
pub mod sidebar_header;
pub mod sidebar_toggle;

pub use sidebar_content::SidebarContent;
pub use sidebar_footer::SidebarFooter;
pub use sidebar_header::SidebarHeader;
pub use sidebar_toggle::SidebarToggle;

use yew::prelude::*;

use crate::contexts::SidebarContext;

/// Properties for the [`Sidebar`].
#[derive(Debug, PartialEq, Properties)]
pub struct SidebarProperties {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// A container component to display content in a Sidebar.
///
/// It has the following purposes:
///
/// - To provide the content in a Sidebar.
///
/// Usage:
/// ```ignore
/// <Sidebar>
///     <SidebarHeader>
///         { "..." }
///     </SidebarHeader>
///     <SidebarContent>
///         { "..." }
///     </SidebarContent>
///     // Optional
///     <SidebarFooter>
///         { "..." }
///     </SidebarFooter>
/// </Sidebar>
/// ```
#[derive(Debug)]
pub struct Sidebar {
    _ctx_handle: ContextHandle<SidebarContext>,
}

impl Component for Sidebar {
    type Message = ();
    type Properties = SidebarProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let (_, _ctx_handle) = ctx
            .link()
            .context::<SidebarContext>(ctx.link().callback(|_| ()))
            .expect("No sidebar context provided");

        Self { _ctx_handle }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let (sidebar_context, _) = ctx
            .link()
            .context::<SidebarContext>(Callback::noop())
            .expect("No sidebar context provided");

        let open = &sidebar_context.state.open;

        let Self::Properties {
            children,
            class,
            style,
            ..
        } = ctx.props();

        html! {
            <aside
                class={classes!("sidebar",
                    open.then_some("open"),
                    class.clone())}
                {style}
            >
                <div class="sidebar-container">{ children.clone() }</div>
            </aside>
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_render_sidebar() {
        let _ = yew::html! {
            <Sidebar>
                <SidebarHeader>{ "Header" }</SidebarHeader>
                <SidebarContent>{ "Content" }</SidebarContent>
                <SidebarFooter>{ "Footer" }</SidebarFooter>
            </Sidebar>
        };
    }
}
