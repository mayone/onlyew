use crate::components::modal::{Modal, close_modal, open_modal};
use yew::prelude::*;

#[function_component(ModalPage)]
pub fn modal() -> Html {
    let modal_ref: NodeRef = NodeRef::default();

    let open_modal = {
        let modal_ref = modal_ref.clone();
        Callback::from(move |_| open_modal(&modal_ref))
    };

    let close_modal = {
        let modal_ref = modal_ref.clone();
        Callback::from(move |_| close_modal(&modal_ref))
    };

    html! {
        <div style="display: flex; flex-direction: column; gap: 20px; padding: 20px">
            <h1>{ "Modal Showcase" }</h1>
            <button style="width: fit-content" onclick={open_modal}>{ "Open Modal" }</button>
            <Modal modal_ref={modal_ref}>
                <div class={classes!("dialog")}>
                    <h2>{ "This is a modal" }</h2>
                    <h3>{ "Hello World" }</h3>
                    <button onclick={close_modal}>{ "Close Modal" }</button>
                </div>
            </Modal>
        </div>
    }
}
