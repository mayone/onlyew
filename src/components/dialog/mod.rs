pub mod dialog_content;
pub mod dialog_footer;
pub mod dialog_header;
pub mod dialog_title;

pub use dialog_content::DialogContent;
pub use dialog_footer::DialogFooter;
pub use dialog_header::DialogHeader;
pub use dialog_title::DialogTitle;

use tailwind_fuse::tw_merge;
use yew::prelude::*;

use crate::components::Modal;

/// Properties for the [`Dialog`].
#[derive(Debug, PartialEq, Properties)]
pub struct DialogProperties {
    pub children: Children,
    pub open: bool,
    #[prop_or_default]
    pub class: AttrValue,
    #[prop_or_default]
    pub style: Option<AttrValue>,
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
/// <Dialog open=true on_close={Callback::noop()}>
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
            open,
            class,
            style,
            on_close,
        } = ctx.props();

        html! {
            <Modal {open} {on_close}>
                <div
                    class={tw_merge!("flex flex-col rounded-2xl max-h-[calc(100dvh-8rem)] max-w-136 overflow-y-auto text-white bg-neutral-800", class.as_ref())}
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
    fn html_with_all_props() {
        let _ = html! {
            <Dialog
                class={tw_merge!("text-black", "text-white")}
                style="background-color: gray"
                open=true
                on_close={Callback::noop()}
            >
                <DialogHeader>
                    <DialogTitle>{ "Title" }</DialogTitle>
                </DialogHeader>
                <DialogContent>{ "Content" }</DialogContent>
                <DialogFooter>{ "Footer" }</DialogFooter>
            </Dialog>
        };
    }
}
