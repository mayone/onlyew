use tailwind_fuse::tw_merge;
use yew::prelude::*;

/// Properties for the [`SidebarItem`].
#[derive(Debug, PartialEq, Properties)]
pub struct SidebarItemProperties {
    pub children: Children,
    #[prop_or_default]
    pub class: AttrValue,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[derive(Debug)]
pub struct SidebarItem;

impl Component for SidebarItem {
    type Message = ();
    type Properties = SidebarItemProperties;

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
                class={tw_merge!("flex items-center transition-[opacity,visibility] delay-[0,300ms] duration-300 group-data-[collapsed-mode=icon]:opacity-0 group-data-[collapsed-mode=icon]:invisible", class.as_ref())}
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
            <SidebarItem
                class={tw_merge!("text-black", "text-white")}
                style="background-color: gray"
            >
                { "Item" }
            </SidebarItem>
        };
    }
}
