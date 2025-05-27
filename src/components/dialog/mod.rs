pub mod dialog_content;
pub mod dialog_footer;
pub mod dialog_header;
pub mod dialog_title;

pub use dialog_content::DialogContent;
pub use dialog_footer::DialogFooter;
pub use dialog_header::DialogHeader;
pub use dialog_title::DialogTitle;

use yew::prelude::*;

use crate::components::Modal;
pub use crate::components::{close_modal as close_dialog, open_modal as open_dialog};

/// The Dialog component has the following props:
///
/// Required props:
///
/// - `children`: The children to be rendered inside the Dialog.
/// - `dialog_ref`: Node reference to the Dialog, which will be used to control
///   the state of the it.
///
/// Optional props:
///
/// - `default_open`: If enabled, the Dialog will be open by default.
/// - `class`: `yew::Classes`
/// - `style`: The style attribute.
///
/// Event handlers:
///
/// - `on_close`: Callback function, called when the Dialog is closed.
#[derive(Debug, PartialEq, Properties)]
pub struct DialogProperties {
    pub children: Children,
    pub dialog_ref: NodeRef,
    #[prop_or_default]
    pub default_open: bool,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<AttrValue>,
    #[prop_or_default]
    pub on_close: Callback<()>,
}

/// A container component to display content in a Dialog.
///
/// It has the following purposes:
///
/// - To provide the content in a Dialog.
///
/// Usage:
/// ```ignore
///
/// let dialog_ref: NodeRef = NodeRef::default();
///
/// let open_dialog = {
///     let dialog_ref = dialog_ref.clone();
///     Callback::from(move |_| open_dialog(&dialog_ref))
/// };
///
/// let close_dialog = {
///     let dialog_ref = dialog_ref.clone();
///     Callback::from(move |_| close_dialog(&dialog_ref))
/// };
///
/// <button onclick={open_dialog}>{"Open dialog"}</button>
/// <Dialog {dialog_ref}>
///     <DialogHeader>
///         <DialogTitle>{ "..." }</DialogTitle>
///     </DialogHeader>
///     <DialogContent>
///         { "..." }
///     </DialogContent>
///     // Optional
///     <DialogFooter>
///         { "..." }
///     </DialogFooter>
/// </Dialog>
/// ```
///
/// Note:
/// - `dialog_ref` is passed to Modal to control it
/// - `close_dialog` is actually the re-export of `close_modal`
/// - `open_dialog` is actually the re-export of `open_modal`
#[derive(Debug)]
pub struct Dialog;

impl Component for Dialog {
    type Message = ();
    type Properties = DialogProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self::Properties {
            children,
            dialog_ref,
            default_open,
            class,
            style,
            on_close,
        } = ctx.props();

        html! {
            <Modal modal_ref={dialog_ref} {default_open} on_close={on_close}>
                <div
                    class={classes!("dialog",
                        class.clone()
                    )}
                    {style}
                >
                    { children.clone() }
                </div>
            </Modal>
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_render_dialog() {
        let dialog_ref = NodeRef::default();
        let _ = yew::html! {
            <Dialog {dialog_ref}>
                <DialogHeader>
                    <DialogTitle>{ "Title" }</DialogTitle>
                </DialogHeader>
                <DialogContent>{ "Content" }</DialogContent>
                <DialogFooter>{ "Footer" }</DialogFooter>
            </Dialog>
        };
    }
}
