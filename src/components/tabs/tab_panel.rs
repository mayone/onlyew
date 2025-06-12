use yew::prelude::*;

use crate::contexts::TabsContext;

/// Properties for the [`TabPanel`].
#[derive(Debug, PartialEq, Properties)]
pub struct TabPanelProperties {
    #[prop_or_default]
    pub children: Children,
    pub value: AttrValue,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[derive(Debug)]
pub enum TabPanelMessage {
    ContextUpdated(TabsContext),
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
    tabs_context: TabsContext,
    _ctx_handle: ContextHandle<TabsContext>,
}

impl Component for TabPanel {
    type Message = TabPanelMessage;
    type Properties = TabPanelProperties;

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
        let TabPanelProperties {
            children,
            class,
            style,
            value,
            ..
        } = ctx.props();

        let is_selected = value.clone() == self.tabs_context.state.selected_tab;

        html! {
            <div
                class={classes!("tab-panel", (!is_selected).then_some("hidden"), class.clone())}
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
    fn test_render_tab_panel() {
        let _ = html! {
            <TabPanel value="1">
                <div>{ "Tab 1" }</div>
            </TabPanel>
        };
    }
}
