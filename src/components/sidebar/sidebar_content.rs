use tailwind_fuse::tw_merge;
use yew::prelude::*;

/// Properties for the [`SidebarContent`].
#[derive(Debug, PartialEq, Properties)]
pub struct SidebarContentProperties {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: AttrValue,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[derive(Debug)]
pub struct SidebarContent;

impl Component for SidebarContent {
    type Message = ();
    type Properties = SidebarContentProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self::Properties {
            children,
            class,
            style,
            ..
        } = ctx.props();

        html! {
            <div
                class={tw_merge!(
            "flex flex-col p-3 grow overflow-auto group-data-[collapsed-mode=icon]:overflow-hidden",
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
            <SidebarContent
                class={tw_merge!("text-black", "text-white")}
                style="background-color: gray"
            >
                { "Content" }
            </SidebarContent>
        };
    }
}
