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
