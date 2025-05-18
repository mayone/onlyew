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
    selected_tab: usize,
}

impl Component for Tabs {
    type Message = TabsMessage;
    type Properties = TabsProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let selected_tab = ctx.props().default_tab.unwrap_or(0);
        Self { selected_tab }
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
                { for children.iter().enumerate().map(|(index, child)| {
                    let is_selected = index == self.selected_tab;
                    html! {
                        <button
                            class={classes!("tab-button", is_selected.then_some("selected"))}
                            aria-selected={is_selected.to_string()}
                            onclick={let on_select = on_select.clone();
                                Callback::from(move |_| on_select.emit(index))}
                        >
                            { child }
                        </button>
                    }
                })}
                // { children.iter().enumerate().map(|(index, child)| {
                //     let is_selected = index == self.selected_tab;
                //     html! {
                //         <button
                //             class={classes!("tab-button", is_selected.then_some("selected"))}
                //             aria-selected={is_selected.to_string()}
                //             onclick={let on_select = on_select.clone();
                //                 Callback::from(move |_| on_select.emit(index))}
                //         >
                //             { child }
                //         </button>
                //     }
                // }).collect::<Vec<_>>()}
            </div>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_tabs() {
        let _ = html! { <Tabs>{"Tab 1"}</Tabs> };
    }
}
