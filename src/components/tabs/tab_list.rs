use std::{collections::HashMap, rc::Rc};

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

/// A component to contain a list of tabs.
#[derive(Debug)]
pub struct TabList {
    indicator_ref: NodeRef,
    tab_refs: HashMap<String, NodeRef>,
    _ctx_handle: ContextHandle<TabsContext>,
}

impl Component for TabList {
    type Message = ();
    type Properties = TabListProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let (tabs_context, ctx_handle) = ctx
            .link()
            .context::<TabsContext>(ctx.link().callback(|_| ()))
            .expect("No tabs context provided");

        let tab_refs = ctx
            .props()
            .children
            .iter()
            .enumerate()
            .map(|(index, child)| {
                let value = child.props.value.clone();
                if index == 0 && tabs_context.state.selected_tab.is_empty() {
                    tabs_context
                        .state
                        .dispatch(TabsAction::Select(value.clone()));
                }

                (value.to_string(), NodeRef::default())
            })
            .collect::<HashMap<_, _>>();

        Self {
            indicator_ref: NodeRef::default(),
            tab_refs,
            _ctx_handle: ctx_handle,
        }
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
            ..
        } = ctx.props();

        let children = children
            .iter()
            .map(|mut child| {
                let props = Rc::make_mut(&mut child.props);
                let value = props.value.clone();

                props.is_selected = value == tabs_context.state.selected_tab;
                props.node_ref = self.tab_refs[&value.to_string()].clone();

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

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        let (tabs_context, _) = ctx
            .link()
            .context::<TabsContext>(Callback::noop())
            .expect("No tabs context provided");

        let selected = tabs_context.state.selected_tab.clone();

        update_indicator_position(&self.tab_refs, &self.indicator_ref, &selected, first_render);
    }
}

fn update_indicator_position(
    tab_refs: &HashMap<String, NodeRef>,
    indicator_ref: &NodeRef,
    selected: &str,
    first_render: bool,
) {
    if let Some(tab_ref) = tab_refs.get(&selected.to_string()) {
        if let Some(tab) = tab_ref.cast::<HtmlElement>() {
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

            if let Some(indicator) = indicator_ref.cast::<HtmlElement>() {
                let _ = indicator.set_attribute("style", &indicator_style);
            }
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
