pub mod node;

use yew::prelude::*;

use node::Viewer;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="container-fluid row">
            <div class="col overflow-scroll vh-100">
                <Viewer id=3></Viewer>
                <Viewer id=4></Viewer>
                <Viewer id=5></Viewer>
            </div>
            <div class="col overflow-scroll vh-100">
                <Viewer id=5></Viewer>
            </div>
            <div class="col overflow-scroll vh-100">
                <Viewer id=6></Viewer>
            </div>
        </div>
    }
}

