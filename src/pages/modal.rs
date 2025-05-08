use crate::components::modal::Modal;
use yew::prelude::*;

#[function_component(ModalPage)]
pub fn modal() -> Html {
    html! {
        <div
            id="modal-root"
            style="display: flex; flex-direction: column; gap: 20px; padding: 20px"
        >
            <h1>{ "Modal Showcase" }</h1>
            <Modal>
                <div class={classes!("modal-content")}>
                    <h2>{ "Hello World" }</h2>
                    <h2>{ "This is a modal" }</h2>
                </div>
            </Modal>
        </div>
    }
}
