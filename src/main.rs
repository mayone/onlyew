use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod contexts;
mod pages;
mod route;

use components::{CollapsedMode, Sidebar, SidebarContent, SidebarFooter};
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
                    <div class="flex">
                        <Sidebar>
                            <SidebarHeader>
                                <SidebarToggle>{ "Toggle" }</SidebarToggle>
                            </SidebarHeader>
                            <SidebarContent>
                                <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
                                <Link<Route> to={Route::Pagination}>{ "Pagination" }</Link<Route>>
                                <Link<Route> to={Route::Dialog}>{ "Dialog" }</Link<Route>>
                                <Link<Route> to={Route::Tabs}>{ "Tabs" }</Link<Route>>
                                <Link<Route> to={Route::SegmentedControl}>
                                    { "Segmented Control" }
                                </Link<Route>>
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
