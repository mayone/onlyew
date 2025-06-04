use web_sys::HtmlElement;
use yew::prelude::*;

mod tab;
pub use tab::Tab;

/// Properties for the [`Tabs`].
#[derive(Debug, PartialEq, Properties)]
pub struct TabsProperties {
    #[prop_or_default]
    pub children: ChildrenWithProps<Tab>,
    /// The index of the default tab to be selected.
    #[prop_or_default]
    pub default_tab: Option<usize>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<AttrValue>,
    /// A callback function that is called when the selected tab changes.
    #[prop_or_default]
    pub on_change: Callback<usize>,
}

#[derive(Debug)]
pub enum TabsMessage {
    Select(usize),
}

/// A component to display tabs.
///
/// It has the following purposes:
///
/// - To provide the tabs in a Tabs component.
///
/// Usage:
/// ```ignore
///
/// html! {
///     <Tabs on_change={Callback::from(|index| log::info!("Tab changed to: {}", index))}>
///         <Tab panel={html!{<div>{"Panel 1"}</div>}}>{"Tab 1"}</Tab>
///         <Tab panel={html!{<div>{"Panel 2"}</div>}}>{"Tab 2"}</Tab>
///     </Tabs>
/// }
/// ```
#[derive(Debug)]
pub struct Tabs {
    indicator_ref: NodeRef,
    selected_tab: usize,
    tab_refs: Vec<NodeRef>,
}

impl Component for Tabs {
    type Message = TabsMessage;
    type Properties = TabsProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let selected_tab = ctx.props().default_tab.unwrap_or(0);
        let tab_refs = ctx
            .props()
            .children
            .iter()
            .map(|_| NodeRef::default())
            .collect::<Vec<_>>();

        Self {
            indicator_ref: NodeRef::default(),
            selected_tab,
            tab_refs,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TabsMessage::Select(index) => {
                if self.selected_tab != index {
                    self.selected_tab = index;
                    ctx.props().on_change.emit(index);
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self::Properties {
            children,
            class,
            style,
            ..
        } = ctx.props();
        let on_select = ctx.link().callback(TabsMessage::Select);

        html! {
            <div class="tabs-container">
                <div class={classes!("tabs", class.clone())} {style}>
                    <div>
                        { children.iter().enumerate().map(|(index, child)| {
                            let is_selected = index == self.selected_tab;
                            html! {
                                <button
                                    key={index}
                                    disabled={child.props.disabled}
                                    class={classes!("tab-button", is_selected.then_some("selected"))}
                                    aria-selected={is_selected.to_string()}
                                    ref={self.tab_refs[index].clone()}
                                    onclick={let on_select = on_select.clone();
                                        Callback::from(move |_| on_select.emit(index))}
                                >
                                    { child.props.children.clone() }
                                </button>
                            }
                        }).collect::<Html>() }
                    </div>
                    <span class={classes!("tabs-indicator")} ref={self.indicator_ref.clone()} />
                </div>
                <div class="tabs-panel">
                    { children.iter().enumerate().find(|(index, _)| *index == self.selected_tab)
                    .map(|(_, child)| child.props.panel.clone())
                        .unwrap_or_default() }
                </div>
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        let indicator = self.indicator_ref.cast::<HtmlElement>().unwrap();

        if let Some(tab) = self.tab_refs[self.selected_tab].cast::<HtmlElement>() {
            let indicator_style = format!(
                "width: {}px; transform: translateX({}px)",
                tab.client_width(),
                tab.offset_left(),
            );
            let _ = indicator.set_attribute("style", &indicator_style);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_tabs() {
        let _ = html! {
            <Tabs>
                <Tab panel={html!{<div>{"Panel 1"}</div>}}>{ "Tab 1" }</Tab>
                <Tab panel={html!{<div>{"Panel 2"}</div>}}>{ "Tab 2" }</Tab>
            </Tabs>
        };
    }
}
