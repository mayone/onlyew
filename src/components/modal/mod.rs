//! Container based on [`<dialog>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dialog) tag.

use web_sys::{Element, HtmlElement};
use yew::prelude::*;

const APP_ID: &str = "app-root";
const MODAL_ROOT_ID: &str = "modal-root";

/// Properties for the [`Modal`].
#[derive(Debug, PartialEq, Properties)]
pub struct ModalProperties {
    pub children: Children,
    pub open: bool,
    pub on_close: Callback<()>,
}

/// A container based on [`<dialog>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dialog) tag.
///
/// Usage:
/// ```ignore
/// use crate::components::modal::Modal;
///
/// <Modal
///     open=true
///     on_close={Callback::noop()}
/// >
///     <h1>{ "This is a modal" }</h1>
///     <button onclick={Callback::noop()}>{"Close modal"}</button>
/// </Modal>
/// ```
///
/// Note:
/// 1. focus outline of dialog is manually removed by us.
/// 2. `modal-content` need to stay in `modal-backdrop` to be aligned by it
///    instead of `dialog`. Since when `open` is enabled, the position
///    of the first time rendered component inside dialog will be off
///    vertically.
#[derive(Debug)]
pub struct Modal {
    modal_root: Element,
    node_ref: NodeRef,
}

impl Component for Modal {
    type Message = ();
    type Properties = ModalProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        let node_ref = NodeRef::default();
        let modal_root = gloo::utils::document()
            .get_element_by_id(MODAL_ROOT_ID)
            .unwrap_or_else(|| panic!("Expected to find a #{} element", MODAL_ROOT_ID));

        Self {
            modal_root,
            node_ref,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self::Properties {
            children,
            open,
            on_close,
            ..
        } = ctx.props();

        let content = html! {
            <dialog
                class="modal"
                ref={self.node_ref.clone()}
                open={*open}
                onkeydown={let on_close = on_close.clone();
                    Callback::from(move |e: KeyboardEvent| {
                        if e.key() == "Escape" {
                            on_close.emit(());
                        }
                    })}
            >
                <div
                    class={classes!("modal-backdrop")}
                    onclick={let on_close = on_close.clone();
                        Callback::from(move |_| on_close.emit(()))}
                >
                    <div
                        class={classes!("modal-content")}
                        onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}
                    >
                        { children.clone() }
                    </div>
                </div>
            </dialog>
        };

        if !*open {
            return html! {};
        }

        let app_root = gloo::utils::document()
            .get_element_by_id(APP_ID)
            .unwrap_or_else(|| panic!("Expected to find the element {}", APP_ID));
        let _ = app_root.set_attribute("inert", "");

        create_portal(content, self.modal_root.clone())
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        if !self.modal_root.has_child_nodes() {
            let app_root = gloo::utils::document()
                .get_element_by_id(APP_ID)
                .unwrap_or_else(|| panic!("Expected to find the element {}", APP_ID));
            let _ = app_root.remove_attribute("inert");
        } else if let Some(dialog) = self.node_ref.cast::<HtmlElement>() {
            let _ = dialog.focus();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_modal() {
        let _ = html! { <Modal open=true on_close={Callback::noop()}>{ "Content" }</Modal> };
    }
}
