use yew::prelude::*;
// use yew_router::prelude::Link;

// use crate::Route;

use crate::components::*;

macro_rules! make_open_dialog {
    ($dialog_ref:expr) => {{
        let dialog_ref = $dialog_ref.clone();
        Callback::from(move |_| open_dialog(&dialog_ref))
    }};
}

macro_rules! make_close_dialog {
    ($dialog_ref:expr, $on_close:expr) => {{
        let dialog_ref = $dialog_ref.clone();
        let on_close = $on_close.clone();
        Callback::from(move |_| close_dialog(&dialog_ref, &on_close))
    }};
}

macro_rules! make_hide_dialog {
    ($dialog_ref:expr) => {{
        let dialog_ref = $dialog_ref.clone();
        Callback::from(move |_| hide_dialog(&dialog_ref))
    }};
}

#[function_component(DialogPage)]
pub fn dialog() -> Html {
    let long_dialog_ref: NodeRef = NodeRef::default();
    let form_dialog_ref: NodeRef = NodeRef::default();

    let open_long_dialog = make_open_dialog!(long_dialog_ref);
    let open_form_dialog = make_open_dialog!(form_dialog_ref);

    html! {
        <div style="display: flex; flex-direction: column; gap: 20px; padding: 20px">
            <h1>{ "Dialog Showcase" }</h1>
            <button style="width: fit-content" onclick={open_long_dialog}>{ "Long Dialog" }</button>
            <button style="width: fit-content" onclick={open_form_dialog}>{ "Form Dialog" }</button>
            // <Link<Route> to={Route::Home}>{ "home" }</Link<Route>>
            <LongDialog dialog_ref={long_dialog_ref} />
            <FormDialog dialog_ref={form_dialog_ref} />
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct LongDialogProps {
    dialog_ref: NodeRef,
}

#[function_component(LongDialog)]
pub fn long_dialog(LongDialogProps { dialog_ref }: &LongDialogProps) -> Html {
    let close_dialog = make_close_dialog!(dialog_ref, Callback::noop());

    html! {
        <Dialog {dialog_ref}>
            <DialogHeader>
                <DialogTitle>{ "This is a long dialog" }</DialogTitle>
            </DialogHeader>
            <DialogContent>
                <div>
                    { "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aliquam imperdiet vehicula diam id sollicitudin. Nulla ultricies nulla eget velit vestibulum, eget semper velit vestibulum. Nulla eleifend vehicula lectus, ac tristique metus fringilla quis. Suspendisse eu porttitor felis. Integer tincidunt elit eget consectetur ultricies. Nullam nec vehicula leo. Aliquam ac neque eu turpis porta eleifend. Donec risus leo, consectetur vitae pharetra et, dapibus et tellus. Suspendisse urna leo, mollis quis lobortis ac, dapibus at arcu. Ut pharetra mattis mauris. Phasellus lobortis tellus euismod convallis volutpat. Sed et velit vitae est eleifend imperdiet. Maecenas maximus justo vel velit pellentesque volutpat.

Sed eget tortor tincidunt, convallis augue sed, pretium nibh. Suspendisse a ipsum ornare, vehicula arcu eu, rutrum metus. Vivamus molestie ipsum id erat venenatis, tempor imperdiet arcu pellentesque. Donec a vehicula libero. Fusce a libero libero. Vestibulum nulla urna, volutpat vitae ornare eu, varius a turpis. Nam eros dui, finibus a faucibus ut, aliquam quis nunc. Aenean sollicitudin neque vel enim venenatis vestibulum. Duis dignissim sem lectus, quis porttitor enim luctus quis. Maecenas ipsum lorem, aliquet et varius vel, efficitur a enim. Sed interdum mattis mauris, nec maximus sem molestie et. Donec rutrum fringilla ex eu efficitur. Donec sit amet risus in purus iaculis molestie.

Cras dictum lacus quam, sed ornare magna porttitor et. Donec commodo eros ligula, sit amet placerat lorem mattis eu. In suscipit pulvinar finibus. Morbi elit ipsum, pulvinar non pharetra faucibus, suscipit nec justo. Morbi quis sagittis neque. Nullam sed suscipit sapien. Pellentesque sed turpis eu libero faucibus convallis. Maecenas sodales vulputate arcu. Sed finibus eros a sapien elementum imperdiet. Morbi commodo convallis velit, vel porttitor mauris ullamcorper sit amet. Nullam in mattis erat. Aenean pellentesque dolor nec maximus eleifend. Mauris interdum est ac iaculis accumsan. Nunc suscipit nibh sed lorem luctus, nec ultrices odio porttitor.

Phasellus suscipit imperdiet ante. Aliquam ultricies metus et finibus convallis. Aenean molestie orci eu fermentum euismod. Suspendisse hendrerit et lorem sed venenatis. Praesent vehicula ex eu massa venenatis, et pretium tortor pharetra. Sed vel nunc ante. Nam a neque dui. Duis vitae facilisis nisi, ut consectetur ante. Integer non metus id augue pulvinar imperdiet ut sit amet enim. Nunc dictum molestie odio, a tempus ipsum pellentesque et. Curabitur ut metus at turpis efficitur aliquet. Ut nulla purus, molestie ac aliquet at, sodales non sem.

Quisque condimentum quis sapien quis consectetur. Quisque ornare sit amet augue at congue. In quis porta tortor. Nulla facilisi. Nulla blandit non justo a varius. Nam sed neque a enim pellentesque scelerisque. Sed et auctor est. Etiam id nibh ut erat vulputate posuere eget et ipsum. Mauris eget diam efficitur, bibendum mi euismod, vestibulum arcu. Suspendisse sit amet sem at orci auctor ultrices. Fusce congue porta eros, non vulputate lacus iaculis scelerisque. Proin neque lectus, finibus et iaculis in, pellentesque id dui. Nam fermentum nisl sit amet ligula auctor, non egestas massa eleifend." }
                </div>
            </DialogContent>
            <DialogFooter>
                <button onclick={close_dialog}>{ "Close Dialog" }</button>
            </DialogFooter>
        </Dialog>
    }
}

#[derive(Properties, PartialEq)]
pub struct FormDialogProps {
    dialog_ref: NodeRef,
}

#[function_component(FormDialog)]
pub fn form_dialog(FormDialogProps { dialog_ref }: &FormDialogProps) -> Html {
    let value = use_state(|| 0);

    let on_change = {
        let value = value.clone();
        Callback::from(move |val| value.set(val))
    };

    let clear_value = {
        let value = value.clone();
        Callback::from(move |_| {
            value.set(0);
        })
    };

    let close_dialog = make_close_dialog!(dialog_ref, clear_value.clone());
    let hide_dialog = make_hide_dialog!(dialog_ref);

    let handle_cancel = {
        let close_dialog = close_dialog.clone();
        Callback::from(move |e: MouseEvent| {
            close_dialog.emit(e);
        })
    };

    let handle_submit = {
        let value = value.clone();
        let close_dialog = close_dialog.clone();
        Callback::from(move |e: MouseEvent| {
            log::info!("Submit: {}", *value);
            close_dialog.emit(e);
        })
    };

    html! {
        <Dialog {dialog_ref} on_close={clear_value}>
            <DialogHeader>
                <DialogTitle>{ "This is a dialog with form" }</DialogTitle>
            </DialogHeader>
            <DialogContent>
                <Counter value={*value} {on_change} />
                <button onclick={hide_dialog.clone()}>{ "Hide" }</button>
            </DialogContent>
            <DialogFooter>
                <button onclick={handle_cancel}>{ "Cancel" }</button>
                <button onclick={handle_submit}>{ "Submit" }</button>
            </DialogFooter>
        </Dialog>
    }
}

#[derive(Debug, Properties, PartialEq)]
pub struct CounterProps {
    value: i32,
    on_change: Callback<i32>,
}

#[function_component(Counter)]
pub fn counter(CounterProps { value, on_change }: &CounterProps) -> Html {
    let decrement: Callback<MouseEvent> = {
        let on_change = on_change.clone();
        let value = *value;
        Callback::from(move |_| on_change.emit(value - 1))
    };

    let increment = {
        let on_change = on_change.clone();
        let value = *value;
        Callback::from(move |_| on_change.emit(value + 1))
    };

    html! {
        <div style="display: flex; align-items: center; gap: 0.5rem">
            <button onclick={decrement}>{ "-" }</button>
            { *value }
            <button onclick={increment}>{ "+" }</button>
        </div>
    }
}
