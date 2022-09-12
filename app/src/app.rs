use super::Context;

use yew::prelude::*;

use std::rc::Rc;

use crate::node::Viewer;
use crate::menu::Menu;

use reqwest::Client;

use ruinaio_model::Node;

/// The main application logic.
#[function_component(App)]
pub fn app() -> Html {
    let state = use_state(Panel::default);

    let onnew = {
        let state = state.clone();

        Callback::from(move |node| {
            let mut new_state = (*state).clone();

            new_state.nodes.push(Rc::new(node));
            state.set(new_state);
        })
    };

    let context = Context {
        api_client: Client::new(),
    };

    html! {
        <ContextProvider<Context> {context}>
            <div class="container-fluid row text-bg-dark">
                { state.render(onnew) }
            </div>
        </ContextProvider<Context>>
    }
}

#[derive(Clone, Default)]
struct Panel {
    nodes: Vec<Rc<Node>>,
}

impl Panel {
    pub fn render(&self, onnew: Callback<Node>) -> Html {
        html! {
            <div class="col d-flex flex-column vh-100">
                <Menu class="my-3" {onnew}/>
                <div class="overflow-scroll">
                    { for self.nodes.iter().map(|node| html! { <Viewer node={Rc::clone(node)} /> }) }
                </div>
            </div>
        }
    }
}

