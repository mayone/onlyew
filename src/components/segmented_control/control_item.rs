use yew::prelude::*;

/// Properties for the [`ControlItem`].
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ControlItemProperties {
    pub value: AttrValue,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub is_selected: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<AttrValue>,
    #[prop_or_default]
    pub on_change: Callback<AttrValue>,
}

/// A component to represent a single item in a [`SegmentedControl`] component.
#[derive(Debug)]
pub struct ControlItem;

impl Component for ControlItem {
    type Message = ();
    type Properties = ControlItemProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self::Properties {
            value,
            children,
            disabled,
            is_selected,
            node_ref,
            class,
            style,
            on_change,
            ..
        } = ctx.props();

        let on_click = {
            let value = value.clone();
            let on_change = on_change.clone();
            Callback::from(move |_| on_change.emit(value.clone()))
        };

        html! {
            <button
                ref={node_ref}
                disabled={*disabled}
                class={classes!("segmented-control-item", is_selected.then_some("selected"), disabled.then_some("disabled"), class.clone())}
                {style}
                onclick={on_click}
            >
                { children.clone() }
            </button>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn html_with_all_props() {
        let _ = html! {
            <ControlItem
                value="1"
                disabled=false
                is_selected=true
                class={classes!("test-class")}
                style="color: red"
                on_change={Callback::noop()}
            >
                { "ControlItem 1" }
            </ControlItem>
        };
    }
}
