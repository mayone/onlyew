use std::rc::Rc;

use yew::prelude::*;

const SIDEBAR_KEYBOARD_SHORTCUT: &str = "b";

#[derive(Clone, Debug, PartialEq)]
pub struct Sidebar {
    pub is_open: bool,
}

#[derive(Debug)]
pub enum SidebarAction {
    Toggle,
}

impl Reducible for Sidebar {
    type Action = SidebarAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Self::Action::Toggle => Rc::new(Self {
                is_open: !self.is_open,
            }),
        }
    }
}

pub type SidebarContext = UseReducerHandle<Sidebar>;

#[derive(Debug, PartialEq, Properties)]
pub struct SidebarProviderProperties {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub default_open: bool,
}

#[function_component]
pub fn SidebarProvider(props: &SidebarProviderProperties) -> Html {
    let SidebarProviderProperties {
        children,
        default_open,
        ..
    } = props;

    let context = use_reducer(|| Sidebar {
        is_open: *default_open,
    });

    html! {
        <ContextProvider<SidebarContext> context={context.clone()}>
            <div
                style="outline: none"
                tabindex="-1"
                onkeydown={Callback::from(move |e: KeyboardEvent| {
                    if e.key() == SIDEBAR_KEYBOARD_SHORTCUT && (e.meta_key() || e.ctrl_key()) {
                        context.dispatch(SidebarAction::Toggle);
                    }
                })}
            >
                { children.clone() }
            </div>
        </ContextProvider<SidebarContext>>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn html_with_all_props() {
        let _ = html! {
            <SidebarProvider default_open=true>
                <div>{ "Sidebar" }</div>
            </SidebarProvider>
        };
    }

    #[test]
    fn tabs_state_reducer() {
        let initial_state = Sidebar { is_open: true };

        let reduced = Sidebar::reduce(Rc::new(initial_state), SidebarAction::Toggle);

        assert!(!reduced.is_open);
    }
}
