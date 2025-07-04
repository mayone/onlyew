use std::rc::Rc;

use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct SidebarState {
    pub open: bool,
}

#[derive(Debug)]
pub enum SidebarAction {
    Toggle(),
}

impl Reducible for SidebarState {
    type Action = SidebarAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Self::Action::Toggle() => Rc::new(Self { open: !self.open }),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SidebarContext {
    pub state: UseReducerHandle<SidebarState>,
    pub on_change: Callback<AttrValue>,
}

#[derive(Debug, PartialEq, Properties)]
pub struct SidebarProviderProperties {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub default_open: bool,
    #[prop_or_default]
    pub on_change: Callback<AttrValue>,
}

#[function_component]
pub fn SidebarProvider(props: &SidebarProviderProperties) -> Html {
    let SidebarProviderProperties {
        children,
        default_open,
        on_change,
        ..
    } = props;

    let state = use_reducer(|| SidebarState {
        open: *default_open,
    });

    let context = SidebarContext {
        state,
        on_change: on_change.clone(),
    };

    html! {
        <ContextProvider<SidebarContext> {context}>
            { children.clone() }
        </ContextProvider<SidebarContext>>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn html_with_all_props() {
        let _ = html! {
            <SidebarProvider default_open=true on_change={Callback::noop()}>
                <div>{ "Sidebar" }</div>
            </SidebarProvider>
        };
    }

    #[test]
    fn tabs_state_reducer() {
        let initial_state = SidebarState { open: true };

        let reduced = SidebarState::reduce(Rc::new(initial_state), SidebarAction::Toggle());

        assert!(!reduced.open);
    }
}
