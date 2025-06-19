use yew::prelude::*;
use yew_router::prelude::Link;

use crate::Route;
use crate::components::*;

macro_rules! make_toggle_callbacks {
    ($state:expr) => {{
        let state = $state.clone();

        let toggle_for_click = {
            let state = state.clone();
            Callback::from(move |_| {
                state.set(!*state);
            })
        };

        let toggle_for_close = {
            let state = state.clone();
            Callback::from(move |_: ()| {
                state.set(!*state);
            })
        };

        (toggle_for_click, toggle_for_close)
    }};
}

#[function_component(DialogPage)]
pub fn dialog() -> Html {
    let is_long_dialog_open = use_state(|| false);
    let is_form_dialog_open = use_state(|| false);
    let is_tabs_dialog_open = use_state(|| false);

    let (toggle_long_dialog, toggle_long_dialog_for_close) =
        make_toggle_callbacks!(is_long_dialog_open);
    let (toggle_form_dialog, toggle_form_dialog_for_close) =
        make_toggle_callbacks!(is_form_dialog_open);
    let (toggle_tabs_dialog, toggle_tabs_dialog_for_close) =
        make_toggle_callbacks!(is_tabs_dialog_open);

    html! {
        <div style="display: flex; flex-direction: column; gap: 20px; padding: 20px">
            <h1>{ "Dialog Showcase" }</h1>
            <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
            <button style="width: fit-content" onclick={toggle_long_dialog}>{ "Long Dialog" }</button>
            <button style="width: fit-content" onclick={toggle_form_dialog}>{ "Form Dialog" }</button>
            <button style="width: fit-content" onclick={toggle_tabs_dialog}>{ "Tabs Dialog" }</button>
            <LongDialog is_open={*is_long_dialog_open} handle_close={toggle_long_dialog_for_close} />
            <FormDialog is_open={*is_form_dialog_open} handle_close={toggle_form_dialog_for_close} />
            <TabsDialog is_open={*is_tabs_dialog_open} handle_close={toggle_tabs_dialog_for_close} />
        </div>
    }
}

#[derive(Debug, Properties, PartialEq)]
pub struct LongDialogProps {
    is_open: bool,
    handle_close: Callback<()>,
}

#[function_component(LongDialog)]
pub fn long_dialog(
    LongDialogProps {
        is_open,
        handle_close,
    }: &LongDialogProps,
) -> Html {
    let handle_close_for_click = {
        let handle_close = handle_close.clone();
        Callback::from(move |_| {
            handle_close.emit(());
        })
    };

    html! {
        <Dialog open={*is_open} on_close={handle_close}>
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
                <button onclick={handle_close_for_click}>{ "Close Dialog" }</button>
            </DialogFooter>
        </Dialog>
    }
}

#[derive(Debug, Properties, PartialEq)]
pub struct FormDialogProps {
    is_open: bool,
    handle_close: Callback<()>,
}

#[function_component(FormDialog)]
pub fn form_dialog(
    FormDialogProps {
        is_open,
        handle_close,
    }: &FormDialogProps,
) -> Html {
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

    let handle_close_for_click = {
        let handle_close = handle_close.clone();
        Callback::from(move |_| {
            handle_close.emit(());
        })
    };

    let handle_submit = {
        let value = value.clone();
        let clear_value = clear_value.clone();
        let handle_close = handle_close.clone();
        Callback::from(move |_| {
            log::info!("Submit: {}", *value);
            handle_close.emit(());
            clear_value.emit(());
        })
    };

    html! {
        <Dialog open={*is_open} on_close={handle_close}>
            <DialogHeader>
                <DialogTitle>{ "This is a dialog with form" }</DialogTitle>
            </DialogHeader>
            <DialogContent>
                <Counter value={*value} {on_change} />
                <button onclick={handle_close_for_click.clone()}>{ "Hide" }</button>
            </DialogContent>
            <DialogFooter>
                <button onclick={Callback::from(move |e: MouseEvent| {
                    clear_value.emit(());
                    handle_close_for_click.emit(e);
                })}>{ "Clear" }</button>
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

#[derive(Debug, Properties, PartialEq)]
pub struct TabsDialogProps {
    is_open: bool,
    handle_close: Callback<()>,
}

#[function_component(TabsDialog)]
pub fn tabs_dialog(
    TabsDialogProps {
        is_open,
        handle_close,
    }: &TabsDialogProps,
) -> Html {
    let handle_close_for_click = {
        let handle_close = handle_close.clone();
        Callback::from(move |_| {
            handle_close.emit(());
        })
    };

    html! {
        <Dialog open={*is_open} on_close={handle_close}>
            <DialogHeader>
                <DialogTitle>{ "This is a dialog with tabs" }</DialogTitle>
            </DialogHeader>
            <DialogContent>
                // TODO: Fix the indicator in dialog failed to get the width in first render, this issue might related to create_portal.
                <Tabs default_value="1">
                    <TabList>
                        <Tab value="1">{ "Dandelion" }</Tab>
                        <Tab value="3" disabled=true>{ "Wayne" }</Tab>
                        <Tab value="2">{ "Heather" }</Tab>
                    </TabList>
                    <TabPanel value="1">
                        <div>{ "Dandelion" }</div>
                    </TabPanel>
                    <TabPanel value="3">
                        <div>{ "Wayne" }</div>
                    </TabPanel>
                    <TabPanel value="2">
                        <div>{ "Heather" }</div>
                    </TabPanel>
                </Tabs>
            </DialogContent>
            <DialogFooter>
                <button onclick={handle_close_for_click}>{ "Close Dialog" }</button>
            </DialogFooter>
        </Dialog>
    }
}
