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

    #[test]
    fn test_render_tabs_provider() {
        let _ = html! { <TabsProvider /> };
    }

    #[test]
    fn test_tabs_provider_with_props() {
        let default_value = AttrValue::from("tab1");

        let _ = html! {
            <TabsProvider default_value={Some(default_value)} on_change={Callback::noop()}>
                <div>{ "Tabs" }</div>
            </TabsProvider>
        };
    }

    #[test]
    fn test_tabs_state_reducer() {
        let initial_state = TabsState {
            selected_tab: AttrValue::from("initial"),
        };

        let new_tab = AttrValue::from("new_tab");
        let reduced =
            TabsState::reduce(Rc::new(initial_state), TabsAction::Select(new_tab.clone()));

        assert_eq!(reduced.selected_tab, new_tab);
    }
}
