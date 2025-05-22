use gloo;
use web_sys::{Element, HtmlDialogElement};
use yew::prelude::*;

const MODAL_ROOT_ID: &str = "modal-root";

/// The Modal component has the following props:
///
/// Required props:
///
/// - `children`: The children to be rendered inside the Modal.
/// - `modal_ref`: Node reference to the Modal, which will be used to control
///   the state of the it.
///
/// Optional props:
///
/// - `default_open`: If enabled, the Modal will be open by default.
#[derive(Debug, PartialEq, Properties)]
pub struct ModalProperties {
    pub children: Children,
    pub modal_ref: NodeRef,
    #[prop_or_default]
    pub default_open: bool,
}

#[derive(Debug)]
pub enum ModalMessage {
    Close,
    Open,
}

/// The modal component provides a solid foundation for creating dialogs, popovers, etc.
///
/// Usage:
/// ```ignore
/// use crate::components::modal::{Modal, close_modal, open_modal};
///
/// let modal_ref: NodeRef = NodeRef::default();
///
/// let open_modal = {
///     let modal_ref = modal_ref.clone();
///     Callback::from(move |_| open_modal(&modal_ref))
/// };
///
/// let close_modal = {
///     let modal_ref = modal_ref.clone();
///     Callback::from(move |_| close_modal(&modal_ref))
/// };
///
/// <button onclick={open_modal}>{"Open modal"}</button>
/// <Modal
///     modal_ref={modal_ref}
///     // Optional
///     default_open=false
/// >
///     <h1>{ "This is a modal" }</h1>
///     <button onclick={close_modal}>{"Close modal"}</button>
/// </Modal>
/// ```
///
/// Note:
/// 1. focus outline of `dialog` is manually removed by us.
/// 2. `modal-content` need to stay in `modal-backdrop` to be aligned by it
///    instead of `dialog`. Since when `default_open` is enabled, the position
///    of the first time rendered component inside `dialog` will be off
///    vertically.
/// 3. The default open `dialog` cannot be closed by ESC key.
#[derive(Debug)]
pub struct Modal {
    modal_ref: NodeRef,
    modal_root: Element,
}

pub fn close_modal(modal_ref: &NodeRef) {
    if let Some(dialog) = modal_ref.cast::<HtmlDialogElement>() {
        dialog.close();
    }
}

pub fn open_modal(modal_ref: &NodeRef) {
    if let Some(dialog) = modal_ref.cast::<HtmlDialogElement>() {
        let _ = dialog.show_modal();
    }
}

impl Modal {
    fn close(&self) {
        close_modal(&self.modal_ref);
    }
    fn open(&self) {
        open_modal(&self.modal_ref);
    }
}

impl Component for Modal {
    type Message = ModalMessage;
    type Properties = ModalProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let modal_ref = ctx.props().modal_ref.clone();
        let modal_root = gloo::utils::document()
            .get_element_by_id(MODAL_ROOT_ID)
            .unwrap_or_else(|| panic!("Expected to find a #{} element", MODAL_ROOT_ID));
        Self {
            modal_ref,
            modal_root,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ModalMessage::Close => {
                self.close();
                true
            }
            ModalMessage::Open => {
                self.open();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self::Properties {
            children,
            default_open,
            ..
        } = ctx.props();

        let content = html! {
            <dialog class="modal" ref={self.modal_ref.clone()} open={*default_open}>
                <div
                    class={classes!("modal-backdrop")}
                    onclick={ctx.link().callback(move |_| Self::Message::Close)}
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

        create_portal(content, self.modal_root.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_modal() {
        let modal_ref = NodeRef::default();
        let _ = html! { <Modal {modal_ref}>{ "Content" }</Modal> };
    }
}
