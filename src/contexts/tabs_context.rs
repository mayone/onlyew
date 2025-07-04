use std::rc::Rc;

use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct TabsState {
    pub selected_tab: AttrValue,
}

#[derive(Debug)]
pub enum TabsAction {
    Select(AttrValue),
}

impl Reducible for TabsState {
    type Action = TabsAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Self::Action::Select(value) => Rc::new(Self {
                selected_tab: value,
            }),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TabsContext {
    pub state: UseReducerHandle<TabsState>,
    pub on_change: Callback<AttrValue>,
}

#[derive(Debug, PartialEq, Properties)]
pub struct TabsProviderProperties {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub default_value: Option<AttrValue>,
    #[prop_or_default]
    pub on_change: Callback<AttrValue>,
}

#[function_component]
pub fn TabsProvider(props: &TabsProviderProperties) -> Html {
    let state = use_reducer(|| TabsState {
        selected_tab: props.default_value.clone().unwrap_or_default(),
    });

    let context = TabsContext {
        state,
        on_change: props.on_change.clone(),
    };

    html! {
        <ContextProvider<TabsContext> {context}>
            { props.children.clone() }
        </ContextProvider<TabsContext>>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use wasm_bindgen_test::*;

    #[test]
    fn render_tabs_provider() {
        let _ = html! { <TabsProvider /> };
    }

    #[test]
    fn tabs_provider_with_props() {
        let default_value = AttrValue::from("tab1");

        let _ = html! {
            <TabsProvider default_value={Some(default_value)} on_change={Callback::noop()}>
                <div>{ "Tabs" }</div>
            </TabsProvider>
        };
    }

    #[test]
    fn tabs_state_reducer() {
        let initial_state = TabsState {
            selected_tab: AttrValue::from("initial"),
        };

        let new_tab = AttrValue::from("new_tab");
        let reduced =
            TabsState::reduce(Rc::new(initial_state), TabsAction::Select(new_tab.clone()));

        assert_eq!(reduced.selected_tab, new_tab);
    }

    // TODO: Add test for context.
    // #[function_component(TestConsumer)]
    // fn test_consumer() -> Html {
    //     let context = use_context::<TabsContext>().expect("context not found");

    //     html! { <div id="test-output">{ context.state.selected_tab.clone() }</div> }
    // }

    // #[function_component(TestRoot)]
    // fn test_root() -> Html {
    //     let default_value = AttrValue::from("tab-1");

    //     html! {
    //         <TabsProvider default_value={Some(default_value.clone())}>
    //             <TestConsumer />
    //         </TabsProvider>
    //     }
    // }

    // wasm_bindgen_test_configure!(run_in_browser);

    // #[wasm_bindgen_test]
    // fn test_tabs_provider_initializes_with_default_value() {
    //     let document = gloo::utils::document();
    //     let body = document.body().expect("document should have a body");

    //     let root = document.create_element("div").unwrap();
    //     root.set_id("test-root");
    //     body.append_child(&root).unwrap();

    //     yew::Renderer::<TestRoot>::with_root(root).render();

    //     let output = document
    //         .get_element_by_id("test-output")
    //         .unwrap()
    //         .text_content()
    //         .unwrap();

    //     assert_eq!(output, "tab-1");
    // }
}
