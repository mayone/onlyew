use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod contexts;
mod pages;
mod route;

use components::{Sidebar, SidebarContent, SidebarFooter};
use components::{SidebarHeader, SidebarToggle};
use contexts::SidebarProvider;

use route::Route;
use route::switch;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <div id="app-root">
                <SidebarProvider>
                    <div class="sidebar-wrapper">
                        <Sidebar>
                            <SidebarHeader style="padding: 0.5rem;">
                                <SidebarToggle>{ "Menu" }</SidebarToggle>
                            </SidebarHeader>
                            <SidebarContent>
                                <div>{ "Content 1" }</div>
                                <div>{ "Content 2" }</div>
                            </SidebarContent>
                            <SidebarFooter>{ "Footer" }</SidebarFooter>
                        </Sidebar>
                        <div class="sidebar-inset">
                            <Switch<Route> render={switch} />
                        </div>
                    </div>
                </SidebarProvider>
            </div>
            <div id="modal-root" />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Starting Yew app");
    yew::Renderer::<App>::new().render();
}
