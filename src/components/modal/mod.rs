use gloo;
use web_sys::{Element, HtmlDialogElement};
use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct ModalProperties {
    #[prop_or_default]
    pub children: Html,
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
        log::info!("close");
        if let Some(dialog) = self.modal_ref.cast::<HtmlDialogElement>() {
            dialog.close()
        }
    }
    pub fn open_modal(&mut self) {
        log::info!("open");
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
            .get_element_by_id("modal-root")
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
            <dialog
                id="heather-ui-modal"
                tabindex="-1"
                ref={self.modal_ref.clone()}
                open={*default_open}
            >
                <div
                    class={classes!("modal-overlay")}
                    onclick={ctx.link().callback(move |_| Self::Message::Close)}
                >
                    <div onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                        { children.clone() }
                    </div>
                </div>
            </dialog>
        };

        create_portal(content, self.modal_root.clone())
    }
}
