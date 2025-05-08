use crate::components::modal::Modal;
use web_sys::HtmlDialogElement;
use yew::prelude::*;

#[function_component(ModalPage)]
pub fn modal() -> Html {
    let modal_ref: NodeRef = NodeRef::default();

    let open_modal = {
        let modal_ref = modal_ref.clone();
        Callback::from(move |_| {
            if let Some(dialog) = modal_ref.cast::<HtmlDialogElement>() {
                let _ = dialog.show_modal();
            }
        })
    };

    html! {
        <div
            id="modal-root"
            style="display: flex; flex-direction: column; gap: 20px; padding: 20px"
        >
            <h1>{ "Modal Showcase" }</h1>
            <button style="width: fit-content" onclick={open_modal}>{ "Open Modal" }</button>
            <Modal modal_ref={modal_ref}>
                <div class={classes!("modal-content")}>
                    <h2>{ "This is a modal" }</h2>
                    <h3>{ "Hello World" }</h3>
                </div>
            </Modal>
        </div>
    }
}
