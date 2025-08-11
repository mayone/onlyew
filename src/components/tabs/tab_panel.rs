use tailwind_fuse::tw_merge;
use yew::prelude::*;

use crate::contexts::TabsContext;

/// Properties for the [`TabPanel`].
#[derive(Debug, PartialEq, Properties)]
pub struct TabPanelProperties {
    pub value: AttrValue,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: AttrValue,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// A component to display the selected tab's content.
///
/// Usage:
/// ```ignore
/// html! {
///     <TabPanel value="1">
///         <div>{"Tab 1"}</div>
///     </TabPanel>
/// }
/// ```
#[derive(Debug)]
pub struct TabPanel {
    _ctx_handle: ContextHandle<TabsContext>,
}

impl Component for TabPanel {
    type Message = ();
    type Properties = TabPanelProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let (_tabs_context, _ctx_handle) = ctx
            .link()
            .context::<TabsContext>(ctx.link().callback(|_| ()))
            .expect("No tabs context provided");

        Self { _ctx_handle }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let (tabs_context, _) = ctx
            .link()
            .context::<TabsContext>(Callback::noop())
            .expect("No tabs context provided");

        let TabPanelProperties {
            value,
            children,
            class,
            style,
            ..
        } = ctx.props();

        let is_selected = value.clone() == tabs_context.state.selected_tab;

        html! {
            <div
                class={tw_merge!("py-4 px-6", (!is_selected).then_some("hidden"), class.as_ref())}
                {style}
            >
                { children.clone() }
            </div>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn html_with_all_props() {
        let _ = html! {
            <TabPanel
                value="1"
                class={tw_merge!("text-black", "text-white")}
                style="background-color: gray"
            >
                <div>{ "Tab 1" }</div>
            </TabPanel>
        };
    }
}
