pub mod node;

use yew::prelude::*;

use node::Viewer;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <Viewer id=3></Viewer>
            <Viewer id=4></Viewer>
            <Viewer id=5></Viewer>
            <Viewer id=6></Viewer>
        </div>
    }
}

