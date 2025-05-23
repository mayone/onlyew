use yew::prelude::*;

/// The Dialog content has the following props:
///
/// Required props:
///
/// - `children`: The children to be rendered inside.
///
/// Optional props:
///
/// - `class`: `yew::Classes`
/// - `style`: The style attribute.
#[derive(Debug, PartialEq, Properties)]
pub struct DialogContentProperties {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[derive(Debug)]
pub struct DialogContent;

impl Component for DialogContent {
    type Message = ();
    type Properties = DialogContentProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self::Properties {
            children,
            class,
            style,
        } = ctx.props();

        html! {
            <div
                class={classes!("dialog-content",
                    class.clone()
                )}
                {style}
            >
                { children.clone() }
            </div>
        }
    }
}
