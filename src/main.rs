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
                    <div style="display: flex">
                        <Sidebar>
                            <SidebarHeader style="padding: 0.5rem;">
                                <SidebarToggle>{ "Menu" }</SidebarToggle>
                            </SidebarHeader>
                            <SidebarContent>
                                <div>{ "Content 1" }</div>
                                <div style="display: flex; gap: 0.5rem">
                                    <div>{ "I" }</div>
                                    <div>{ "am" }</div>
                                    <div>{ "Content" }</div>
                                    <div>{ "2" }</div>
                                </div>
                            </SidebarContent>
                            <SidebarFooter>{ "Footer" }</SidebarFooter>
                        </Sidebar>
                        <Switch<Route> render={switch} />
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
