use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
    rc::Rc,
};

use crate::contexts::{TabsAction, TabsContext, tabs_context};

use super::Tab;
use web_sys::HtmlElement;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct TabListProperties {
    #[prop_or_default]
    pub children: ChildrenWithProps<Tab>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<AttrValue>,
    /// A callback function that is called when the selected tab changes.
    #[prop_or_default]
    pub on_select: Callback<AttrValue>,
}

#[derive(Debug)]
pub enum TabListMessage {
    Select(AttrValue),
}

/// A component to contain a list of tabs.
///
/// Usage:
/// ```ignore
/// html! {
///     <TabList selected_tab={0} on_select={Callback::from(|_| {})}>
///         <Tab>{"Tab 1"}</Tab>
///         <Tab>{"Tab 2"}</Tab>
///     </TabList>
/// }
/// ```
#[derive(Debug)]
pub struct TabList {
    indicator_ref: NodeRef,
    tab_refs: HashMap<u64, NodeRef>,
}

impl Component for TabList {
    type Message = TabListMessage;
    type Properties = TabListProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let tab_refs = ctx
            .props()
            .children
            .iter()
            .map(|child| {
                let mut hasher = DefaultHasher::new();
                child.props.value.clone().hash(&mut hasher);
                let id = hasher.finish();

                (id, NodeRef::default())
            })
            .collect::<HashMap<_, _>>();

        Self {
            indicator_ref: NodeRef::default(),
            tab_refs,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TabListMessage::Select(value) => {
                let (tabs_context, _) = ctx
                    .link()
                    .context::<TabsContext>(Callback::noop())
                    .expect("No tabs context provided");

                tabs_context.dispatch(TabsAction::Select(value));
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let (tabs_context, _) = ctx
            .link()
            .context::<TabsContext>(Callback::noop())
            .expect("No tabs context provided");

        let TabListProperties {
            children,
            class,
            style,
            on_select,
            ..
        } = ctx.props();

        let children = children.iter().map(|mut child| {
            let props = Rc::make_mut(&mut child.props);
            let mut hasher = DefaultHasher::new();
            let value = props.value.clone();
            value.hash(&mut hasher);
            let id = hasher.finish();
            props.node_ref = self.tab_refs[&id].clone();
            props.value = value.clone();
            props.is_selected = value == *tabs_context.selected_tab;
            props.on_click = {
                ctx.link()
                    .callback(move |value: AttrValue| Self::Message::Select(value.clone()))
            };

            child
        });

        html! {
            <div class={classes!("tab-list", class.clone())} {style}>
                { children.collect::<Html>() }
                <span class={classes!("tabs-indicator")} ref={self.indicator_ref.clone()} />
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        let (tabs_context, _) = ctx
            .link()
            .context::<TabsContext>(Callback::noop())
            .expect("No tabs context provided");

        let selected = tabs_context.selected_tab.clone();

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
                "width: {}px; transform: translateX({}px)",
                tab.client_width(),
                tab.offset_left(),
            );
            let _ = indicator.set_attribute("style", &indicator_style);
        }
    }
}
