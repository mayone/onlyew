use yew::prelude::*;

/// The Dialog header has the following props:
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
pub struct DialogHeaderProperties {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<String>,
}

#[derive(Debug)]
pub struct DialogHeader;

impl Component for DialogHeader {
    type Message = ();
    type Properties = DialogHeaderProperties;

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
                class={classes!("dialog-header",
                    class.clone()
                )}
                style={style.clone()}
            >
                { children.clone() }
            </div>
        }
    }
}
