//! Container based on [`<dialog>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dialog) tag.
use web_sys::{Element, HtmlDialogElement};
use yew::prelude::*;

const MODAL_ROOT_ID: &str = "modal-root";

/// Properties for the [`Modal`].
#[derive(Debug, PartialEq, Properties)]
pub struct ModalProperties {
    pub children: Children,
    /// The [`NodeRef`](yew::NodeRef) for the `<dialog>` tag.
    pub modal_ref: NodeRef,
    #[prop_or_default]
    pub default_open: bool,
    /// Callback function, called when the Modal is closed.
    #[prop_or_default]
    pub on_close: Callback<()>,
}

#[derive(Debug)]
pub enum ModalMessage {
    Close,
}

/// A container based on [`<dialog>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dialog) tag.
///
/// Usage:
/// ```ignore
/// use crate::components::modal::{Modal, close_modal, open_modal};
///
/// let modal_ref: NodeRef = NodeRef::default();
///
///
/// let close_modal = {
///     let modal_ref = modal_ref.clone();
///     Callback::from(move |_| close_modal(&modal_ref, &Callback::noop()))
/// };
///
/// let hide_modal = {
///     let modal_ref = modal_ref.clone();
///     Callback::from(move |_| hide_modal(&modal_ref))
/// };
///
/// let open_modal = {
///     let modal_ref = modal_ref.clone();
///     Callback::from(move |_| open_modal(&modal_ref))
/// };
///
/// <button onclick={open_modal}>{"Open modal"}</button>
/// <Modal
///     modal_ref={modal_ref}
///     // Optional
///     default_open=false
///     on_close={Callback::noop()}
/// >
///     <h1>{ "This is a modal" }</h1>
///     <button onclick={close_modal}>{"Close modal"}</button>
/// </Modal>
/// ```
///
/// Note:
/// 1. focus outline of dialog is manually removed by us.
/// 2. `modal-content` need to stay in `modal-backdrop` to be aligned by it
///    instead of `dialog`. Since when `default_open` is enabled, the position
///    of the first time rendered component inside dialog will be off
///    vertically.
/// 3. The default open dialog cannot be closed by Esc key.
/// 4. `oncancel` is for [`<dialog>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dialog)
///    "cancel" event, required for `on_close` callback when Esc key pressed.
#[derive(Debug)]
pub struct Modal {
    modal_ref: NodeRef,
    modal_root: Element,
}

pub fn close_modal(modal_ref: &NodeRef, on_close: &Callback<()>) {
    if let Some(dialog) = modal_ref.cast::<HtmlDialogElement>() {
        on_close.emit(());
        dialog.close();
    }
}

pub fn hide_modal(modal_ref: &NodeRef) {
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
    fn close(&self, ctx: &Context<Self>) {
        close_modal(&self.modal_ref, &ctx.props().on_close);
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

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ModalMessage::Close => {
                self.close(ctx);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self::Properties {
            children,
            default_open,
            on_close,
            ..
        } = ctx.props();

        let content = html! {
            <dialog
                class="modal"
                ref={self.modal_ref.clone()}
                open={*default_open}
                oncancel={{
                    let on_close = on_close.clone();
                    Callback::from(move |_: Event| on_close.emit(()))
                }}
            >
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
