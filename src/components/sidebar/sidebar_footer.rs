use tailwind_fuse::tw_merge;
use yew::prelude::*;

/// Properties for the [`SidebarFooter`].
#[derive(Debug, PartialEq, Properties)]
pub struct SidebarFooterProperties {
    pub children: Children,
    #[prop_or_default]
    pub class: AttrValue,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[derive(Debug)]
pub struct SidebarFooter;

impl Component for SidebarFooter {
    type Message = ();
    type Properties = SidebarFooterProperties;

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
                class={tw_merge!("flex flex-col justify-center py-4 px-6 transition-[opacity,visibility] delay-[0,300ms] duration-300 group-data-[collapsed-mode=icon]:opacity-0 group-data-[collapsed-mode=icon]:invisible", class.as_ref())}
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
            <SidebarFooter
                class={tw_merge!("text-black", "text-white")}
                style="background-color: gray"
            >
                { "Footer" }
            </SidebarFooter>
        };
    }
}
