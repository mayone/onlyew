use gloo;
// use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlDialogElement};
use yew::prelude::*;

const MODAL_ROOT_ID: &str = "modal-root";

#[derive(Debug, PartialEq, Properties)]
pub struct ModalProperties {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub default_open: bool,
    #[prop_or_default]
    pub modal_ref: NodeRef,
}

#[derive(Debug)]
pub enum ModalMessage {
    Close,
    Open,
}

#[derive(Debug)]
pub struct Modal {
    modal_root: Element,
    modal_ref: NodeRef,
}

impl Modal {
    pub fn close_modal(&mut self) {
        if let Some(dialog) = self.modal_ref.cast::<HtmlDialogElement>() {
            dialog.close()
        }
    }
    pub fn open_modal(&mut self) {
        if let Some(dialog) = self.modal_ref.cast::<HtmlDialogElement>() {
            let _ = dialog.show_modal();
        }
    }
}

impl Component for Modal {
    type Message = ModalMessage;
    type Properties = ModalProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let modal_ref = ctx.props().modal_ref.clone();
        let modal_root = gloo::utils::document()
            .get_element_by_id(MODAL_ROOT_ID)
            .expect("Expected to find a #modal-root element");
        Self {
            modal_ref,
            modal_root,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ModalMessage::Close => {
                self.close_modal();
                true
            }
            ModalMessage::Open => {
                self.open_modal();
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
            <dialog id="heather-ui-modal" ref={self.modal_ref.clone()} open={*default_open}>
                <div
                    class={classes!("modal-overlay")}
                    onclick={ctx.link().callback(move |_| Self::Message::Close)}
                >
                    <div
                        class={classes!("modal-content")}
                        onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}
                    >
                        { children.clone() }
                    </div>
                    // When default_open is enabled. The position of the first time rendered component inside dialog will be off vertically (not center).
                    // To address this, modal-content need to stay in modal-overlay to be aligned by it instead of dialog.
                    // The default open dialog also cannot be closed by ESC key and we have not resolved it yet.
                </div>
            </dialog>
        };

        create_portal(content, self.modal_root.clone())
    }
}
