use std::rc::Rc;

use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Tabs {
    pub selected_tab: AttrValue,
    pub on_change: Callback<AttrValue>,
}

#[derive(Debug)]
pub enum TabsAction {
    Select(AttrValue),
}

impl Reducible for Tabs {
    type Action = TabsAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Self::Action::Select(value) => {
                // TODO: Try to remove on_change from state and call it here.

                Rc::new(Self {
                    selected_tab: value,
                    ..(*self).clone()
                })
            }
        }
    }
}

pub type TabsContext = UseReducerHandle<Tabs>;

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
    let default_value = props.default_value.clone().unwrap_or_default();
    let tabs = use_reducer(|| Tabs {
        selected_tab: default_value,
        on_change: props.on_change.clone(),
    });

    html! {
        <ContextProvider<TabsContext> context={tabs}>
            { props.children.clone() }
        </ContextProvider<TabsContext>>
    }
}
