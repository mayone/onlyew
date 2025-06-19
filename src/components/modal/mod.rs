//! Container based on [`<dialog>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dialog) tag.

use web_sys::Element;
// use web_sys::KeyboardEvent;
use yew::prelude::*;

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
/// 3. The default open dialog cannot be closed by Esc key.
/// 4. `oncancel` is for [`<dialog>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dialog)
///    "cancel" event, required for `on_close` callback when Esc key pressed.
#[derive(Debug)]
pub struct Modal {
    modal_root: Element,
}

impl Component for Modal {
    type Message = ();
    type Properties = ModalProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        let modal_root = gloo::utils::document()
            .get_element_by_id(MODAL_ROOT_ID)
            .unwrap_or_else(|| panic!("Expected to find a #{} element", MODAL_ROOT_ID));

        Self { modal_root }
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
                open={*open}
                // onkeydown={
                //     let on_close = on_close.clone();
                //     Callback::from(move |e: KeyboardEvent| {
                //         if e.key() == "Escape" {
                //             on_close.emit(());
                //         }
                //     })
                // }
            >
                <div
                    class={classes!("modal-backdrop")}
                    onclick={
                        let on_close = on_close.clone();
                        Callback::from(move |_| on_close.emit(()))
                    }
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

        create_portal(content, self.modal_root.clone())
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
