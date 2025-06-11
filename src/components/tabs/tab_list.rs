use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
    rc::Rc,
};

use crate::contexts::{TabsAction, TabsContext};

use super::Tab;

use web_sys::HtmlElement;
use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct TabListProperties {
    #[prop_or_default]
    pub children: ChildrenWithProps<Tab>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[derive(Debug)]
pub enum TabListMessage {
    ContextUpdated(TabsContext),
}

/// A component to contain a list of tabs.
#[derive(Debug)]
pub struct TabList {
    indicator_ref: NodeRef,
    tab_refs: HashMap<u64, NodeRef>,
    tabs_context: TabsContext,
    _ctx_handle: ContextHandle<TabsContext>,
}

impl Component for TabList {
    type Message = TabListMessage;
    type Properties = TabListProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let (tabs_context, ctx_handle) = ctx
            .link()
            .context::<TabsContext>(ctx.link().callback(Self::Message::ContextUpdated))
            .expect("No tabs context provided");

        let tab_refs = ctx
            .props()
            .children
            .iter()
            .enumerate()
            .map(|(index, child)| {
                let value = child.props.value.clone();
                if index == 0 && tabs_context.selected_tab.is_empty() {
                    tabs_context.dispatch(TabsAction::Select(value.clone()));
                }

                let mut hasher = DefaultHasher::new();
                value.hash(&mut hasher);
                let id = hasher.finish();

                (id, NodeRef::default())
            })
            .collect::<HashMap<_, _>>();

        Self {
            indicator_ref: NodeRef::default(),
            tab_refs,
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
        let TabListProperties {
            children,
            class,
            style,
            ..
        } = ctx.props();

        let children = children
            .iter()
            .map(|mut child| {
                let props = Rc::make_mut(&mut child.props);
                let value = props.value.clone();

                let mut hasher = DefaultHasher::new();
                value.hash(&mut hasher);
                let id = hasher.finish();

                props.node_ref = self.tab_refs[&id].clone();

                child
            })
            .collect::<Html>();

        html! {
            <div class={classes!("tab-list", class.clone())} {style}>
                { children }
                <span class={classes!("tabs-indicator")} ref={self.indicator_ref.clone()} />
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        let selected = self.tabs_context.selected_tab.clone();

        let mut hasher = DefaultHasher::new();
        selected.hash(&mut hasher);
        let id = hasher.finish();

        let indicator = self.indicator_ref.cast::<HtmlElement>().unwrap();

        if let Some(tab) = self
            .tab_refs
            .get(&id)
            .and_then(|tab| tab.cast::<HtmlElement>())
        {
            let indicator_style = format!(
                "width: {}px; transform: translateX({}px);{}",
                tab.client_width(),
                tab.offset_left(),
                if first_render {
                    " transition: none;"
                } else {
                    ""
                }
            );
            let _ = indicator.set_attribute("style", &indicator_style);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_tab_list() {
        let _ = html! {
            <TabList>
                <Tab value="1">{ "Tab 1" }</Tab>
                <Tab value="2">{ "Tab 2" }</Tab>
            </TabList>
        };
    }
}
