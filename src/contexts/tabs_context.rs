use std::rc::Rc;

use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Tabs {
    pub selected_tab: AttrValue,
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
                log::info!("value {}", value);
                Rc::new(Self {
                    selected_tab: value,
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
}

#[function_component]
pub fn TabsProvider(props: &TabsProviderProperties) -> Html {
    let tabs = use_reducer(|| Tabs {
        selected_tab: "0".into(),
    });

    html! {
        <ContextProvider<TabsContext> context={tabs}>
            { props.children.clone() }
        </ContextProvider<TabsContext>>
    }
}
