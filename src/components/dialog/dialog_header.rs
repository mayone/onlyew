use tailwind_fuse::tw_merge;
use yew::prelude::*;

/// Properties for the [`DialogHeader`].
#[derive(Debug, PartialEq, Properties)]
pub struct DialogHeaderProperties {
    pub children: Children,
    #[prop_or_default]
    pub class: AttrValue,
    #[prop_or_default]
    pub style: Option<AttrValue>,
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
                class={tw_merge!("flex items-center gap-4 py-4 px-6",
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
            <DialogHeader
                class={tw_merge!("text-black", "text-white")}
                style="background-color: gray"
            >
                { "Header" }
            </DialogHeader>
        };
    }
}
