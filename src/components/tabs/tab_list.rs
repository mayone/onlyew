use std::rc::Rc;

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
    #[prop_or_default]
    pub selected_tab: usize,
    /// A callback function that is called when the selected tab changes.
    #[prop_or_default]
    pub on_select: Callback<usize>,
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
    tab_refs: Vec<NodeRef>,
}

impl Component for TabList {
    type Message = ();
    type Properties = TabListProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let tab_refs = ctx
            .props()
            .children
            .iter()
            .map(|_| NodeRef::default())
            .collect::<Vec<_>>();

        Self {
            indicator_ref: NodeRef::default(),
            tab_refs,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let TabListProperties {
            children,
            class,
            style,
            selected_tab,
            on_select,
        } = ctx.props();

        let children = children.iter().enumerate().map(|(index, mut child)| {
            let props = Rc::make_mut(&mut child.props);
            props.node_ref = self.tab_refs[index].clone();
            props.value = format!("{}", index).into();
            props.is_selected = index == *selected_tab;
            props.on_click = {
                let on_select = on_select.clone();
                Callback::from(move |_| on_select.emit(index))
            };

            child
        });

        html! {
            <div class={classes!("tab-list", class.clone())} {style}>
                { for children }
                <span class={classes!("tabs-indicator")} ref={self.indicator_ref.clone()} />
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        let selected_tab = ctx.props().selected_tab;
        let indicator = self.indicator_ref.cast::<HtmlElement>().unwrap();

        if let Some(tab) = self.tab_refs[selected_tab].cast::<HtmlElement>() {
            let indicator_style = format!(
                "width: {}px; transform: translateX({}px)",
                tab.client_width(),
                tab.offset_left(),
            );
            let _ = indicator.set_attribute("style", &indicator_style);
        }
    }
}
