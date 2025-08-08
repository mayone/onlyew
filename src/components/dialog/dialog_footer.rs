use tailwind_fuse::tw_merge;
use yew::prelude::*;

/// Properties for the [`DialogFooter`].
#[derive(Debug, PartialEq, Properties)]
pub struct DialogFooterProperties {
    pub children: Children,
    #[prop_or_default]
    pub class: AttrValue,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[derive(Debug)]
pub struct DialogFooter;

impl Component for DialogFooter {
    type Message = ();
    type Properties = DialogFooterProperties;

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
            <DialogFooter
                class={tw_merge!("text-black", "text-white")}
                style="background-color: gray"
            >
                { "Footer" }
            </DialogFooter>
        };
    }
}
