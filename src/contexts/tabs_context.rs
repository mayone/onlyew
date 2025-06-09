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
    #[prop_or_default]
    pub default_tab: Option<AttrValue>,
}

#[function_component]
pub fn TabsProvider(props: &TabsProviderProperties) -> Html {
    let default_tab = props.default_tab.clone().unwrap_or("0".into());
    let tabs = use_reducer(|| Tabs {
        selected_tab: default_tab,
    });

    html! {
        <ContextProvider<TabsContext> context={tabs}>
            { props.children.clone() }
        </ContextProvider<TabsContext>>
    }
}
