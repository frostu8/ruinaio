pub mod node;
pub mod menu;

use yew::prelude::*;

use node::Viewer;
use menu::Menu;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="container-fluid row">
            <div class="col d-flex flex-column vh-100">
                <Menu class="my-3"/>
                <div class="overflow-scroll">
                    <Viewer id=3 />
                    <Viewer id=4 />
                    <Viewer id=5 />
                </div>
            </div>
            <div class="col d-flex flex-column vh-100">
                <Menu class="my-3"/>
                <div class="overflow-scroll">
                    <Viewer id=5 />
                </div>
            </div>
            <div class="col d-flex flex-column vh-100">
                <Menu class="my-3"/>
                <div class="overflow-scroll">
                    <Viewer id=6 />
                </div>
            </div>
        </div>
    }
}

