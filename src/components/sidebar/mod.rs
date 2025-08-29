pub mod sidebar_content;
pub mod sidebar_footer;
pub mod sidebar_header;
pub mod sidebar_item;
pub mod sidebar_toggle;

pub use sidebar_content::SidebarContent;
pub use sidebar_footer::SidebarFooter;
pub use sidebar_header::SidebarHeader;
pub use sidebar_item::SidebarItem;
pub use sidebar_toggle::SidebarToggle;
use tailwind_fuse::tw_merge;
use yew::prelude::*;

use crate::contexts::SidebarContext;

#[derive(Debug, strum::Display)]
#[strum(serialize_all = "lowercase")]
pub enum CollapsedMode {
    Icon,
    None,
    Open,
}

/// Properties for the [`Sidebar`].
#[derive(Debug, PartialEq, Properties)]
pub struct SidebarProperties {
    pub children: Children,
    #[prop_or_default]
    pub class: AttrValue,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// A container component to display content in a Sidebar.
///
/// It has the following purposes:
///
/// - To provide the content in a Sidebar.
/// - Can only be used inside <SidebarProvider>
///
/// Usage:
/// ```ignore
/// <SidebarProvider default_open={true}>
///     <Sidebar>
///         <SidebarHeader>
///             { "..." }
///         </SidebarHeader>
///         <SidebarContent>
///             <SidebarMenuButton />
///             <SidebarMenuButton />
///         </SidebarContent>
///         <SidebarFooter>
///             { "..." }
///         </SidebarFooter>
///     </Sidebar>
/// </SidebarProvider>
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

        let Self::Properties {
            children,
            class,
            style,
            ..
        } = ctx.props();

        let collapsed_mode = if sidebar_context.is_open {
            CollapsedMode::Open
        } else {
            CollapsedMode::Icon
        };

        html! {
            <aside
                class={tw_merge!(
                    "group w-65 data-[collapsed-mode=icon]:w-16 bg-neutral-800 shrink-0 min-h-svh relative whitespace-nowrap transition-[width] duration-300 overflow-hidden",
                    class.as_ref()
                )}
                {style}
                data-collapsed-mode={collapsed_mode.to_string()}
            >
                <div class="flex absolute top-0 left-0 flex-col justify-between w-full h-dvh">
                    { children.clone() }
                </div>
            </aside>
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn html_with_all_props() {
        let _ = html! {
            <Sidebar class={tw_merge!("text-black", "text-white")} style="background-color: gray">
                <SidebarHeader>{ "Header" }</SidebarHeader>
                <SidebarContent />
                <SidebarFooter>{ "Footer" }</SidebarFooter>
            </Sidebar>
        };
    }
}
