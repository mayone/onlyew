use tailwind_fuse::tw_merge;
use yew::prelude::*;

/// Properties for the [`DialogTitle`].
#[derive(Debug, PartialEq, Properties)]
pub struct DialogTitleProperties {
    pub children: Children,
    #[prop_or_default]
    pub class: AttrValue,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[derive(Debug)]
pub struct DialogTitle;

impl Component for DialogTitle {
    type Message = ();
    type Properties = DialogTitleProperties;

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
                class={tw_merge!("text-xl",
                    class.as_ref()
                )}
                {style}
            >
                { children.clone() }
            </div>
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn html_with_all_props() {
        let _ = html! {
            <DialogTitle
                class={tw_merge!("text-black", "text-white")}
                style="background-color: gray"
            >
                { "Title" }
            </DialogTitle>
        };
    }
}
