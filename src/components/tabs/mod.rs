use web_sys::HtmlElement;
use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct TabsProperties {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub default_tab: Option<usize>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    pub on_change: Callback<usize>,
}

#[derive(Debug)]
pub enum TabsMessage {
    Select(usize),
}

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
            <div class={classes!("tabs", class.clone())} style={style.clone()}>
                <div>
                    { for children.iter().enumerate().map(|(index, child)| {
                        let is_selected = index == self.selected_tab;
                        html! {
                            <button
                                class={classes!("tab-button", is_selected.then_some("selected"))}
                                aria-selected={is_selected.to_string()}
                                ref={self.tab_refs[index].clone()}
                                onclick={let on_select = on_select.clone();
                                    Callback::from(move |_| on_select.emit(index))}
                            >
                                { child }
                            </button>
                        }
                    }) }
                    // { children.iter().enumerate().map(|(index, child)| {
                    //     let is_selected = index == self.selected_tab;
                    //     html! {
                    //         <button
                    //             class={classes!("tab-button", is_selected.then_some("selected"))}
                    //             aria-selected={is_selected.to_string()}
                    //             ref={self.tab_refs[index].clone()}
                    //             onclick={let on_select = on_select.clone();
                    //                 Callback::from(move |_| on_select.emit(index))}
                    //         >
                    //             { child }
                    //         </button>
                    //     }
                    // }).collect::<Vec<_>>() }
                </div>
                <span class={classes!("tabs-indicator")} ref={self.indicator_ref.clone()} />
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        let indicator = self.indicator_ref.cast::<HtmlElement>().unwrap();

        if let Some(element) = self.tab_refs[self.selected_tab].cast::<HtmlElement>() {
            let indicator_style = format!(
                "width: {}px; transform: translateX({}px)",
                element.client_width(),
                element.offset_left(),
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
        let _ = html! { <Tabs>{ "Tab 1" }</Tabs> };
    }
}
