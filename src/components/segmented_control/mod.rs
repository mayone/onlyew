mod control_item;

pub use control_item::ControlItem;

use std::rc::Rc;

use yew::prelude::*;

/// Properties for the [`SegmentedControl`].
#[derive(Debug, PartialEq, Properties)]
pub struct SegmentedControlProperties {
    #[prop_or_default]
    pub children: ChildrenWithProps<ControlItem>,
    #[prop_or_default]
    pub default_value: AttrValue,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<AttrValue>,
    #[prop_or_default]
    pub on_change: Callback<AttrValue>,
}

#[derive(Debug)]
pub enum SegmentedControlMessage {
    Changed(AttrValue),
}

/// A component to contain a list of [`ControlItem`].
///
/// Usage:
/// ```ignore
/// <SegmentedControl
///     default_value="ControlItem 3"
///     on_change={Callback::from(|value| log::info!("Segmented Control changed to: {value}"))}
/// >
///     <ControlItem value="ControlItem 1">
///         { "ControlItem 1" }
///     </ControlItem>
///     <ControlItem value="ControlItem 2" disabled=true>
///         { "ControlItem 2" }
///     </ControlItem>
///     <ControlItem value="ControlItem 3">
///         { "ControlItem 3" }
///     </ControlItem>
/// </SegmentedControl>
/// ```
/// A component to contain a list of [`ControlItem`].
#[derive(Debug)]
pub struct SegmentedControl {
    selected: AttrValue,
}

impl SegmentedControl {
    pub fn handle_change(&mut self, value: AttrValue, ctx: &Context<Self>) -> bool {
        let SegmentedControlProperties { on_change, .. } = ctx.props();

        if value == self.selected {
            false
        } else {
            self.selected = value;
            on_change.emit(self.selected.clone());

            true
        }
    }
}

impl Component for SegmentedControl {
    type Message = SegmentedControlMessage;
    type Properties = SegmentedControlProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let selected = if ctx.props().default_value.is_empty() {
            ctx.props()
                .children
                .iter()
                .next()
                .map(|child| child.props.value.clone())
                .unwrap_or_default()
        } else {
            ctx.props().default_value.clone()
        };

        Self { selected }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SegmentedControlMessage::Changed(value) => self.handle_change(value, ctx),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let SegmentedControlProperties {
            children,
            class,
            style,
            ..
        } = ctx.props();

        let children = children
            .iter()
            .map(|mut child| {
                let props = Rc::make_mut(&mut child.props);
                let value = props.value.clone();

                props.is_selected = value == self.selected;
                props.on_change = ctx.link().callback(Self::Message::Changed);

                child
            })
            .collect::<Html>();

        html! {
            <div class={classes!("segmented-control-container", class.clone())} {style}>
                { children }
            </div>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn html_with_all_props() {
        let _ = html! {
            <SegmentedControl
                default_value="ControlItem 3"
                class={classes!("test-class")}
                style="background-color: red"
                on_change={Callback::from(|value| log::info!("Segmented Control changed to: {value}"))}
            >
                <ControlItem value="ControlItem 1">{ "ControlItem 1" }</ControlItem>
                <ControlItem value="ControlItem 2" disabled=true>{ "ControlItem 2" }</ControlItem>
                <ControlItem value="ControlItem 3">{ "ControlItem 3" }</ControlItem>
            </SegmentedControl>
        };
    }
}
