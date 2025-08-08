use tailwind_fuse::tw_merge;
use yew::prelude::*;

/// Properties for the [`DialogContent`].
#[derive(Debug, PartialEq, Properties)]
pub struct DialogContentProperties {
    pub children: Children,
    #[prop_or_default]
    pub class: AttrValue,
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
                class={tw_merge!("flex flex-col overflow-y-auto py-4 px-6",
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
            <DialogContent
                class={tw_merge!("text-black", "text-white")}
                style="background-color: gray"
            >
                { "Content" }
            </DialogContent>
        };
    }
}
