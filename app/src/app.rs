use super::Context;

use yew::prelude::*;

use std::rc::Rc;

use crate::node::{Editor, Viewer};
use crate::menu::Menu;

use reqwest::Client;

use ruinaio_model::Node;

/// The main application logic.
#[function_component(App)]
pub fn app() -> Html {
    let state = use_state(Vec::<NodeState>::default);

    let onnew = {
        let state = state.clone();

        Callback::from(move |node| {
            let mut new_state = (*state).clone();

            new_state.push(NodeState::editing(Rc::new(node)));
            state.set(new_state);
        })
    };

    let context = Context {
        api_client: Client::new(),
    };

    let nodes = {
        state
            .iter()
            .enumerate()
            .map(|(i, node)| {
                let state_ref = state.clone();

                if node.editing {
                    let onupdate = Callback::from(move |node| {
                        let mut nodes = (*state_ref).clone();
                        
                        nodes[i] = NodeState {
                            editing: false,
                            node: Rc::new(node),
                        };

                        state_ref.set(nodes);
                    });

                    html! { <Editor node={node.node.clone()} {onupdate} /> }
                } else {
                    html! { <Viewer node={node.node.clone()} /> }
                }
            })
    };

    html! {
        <ContextProvider<Context> {context}>
            <div class="container d-flex flex-column vh-100">
                <Menu class="my-3" {onnew}/>
                <div class="overflow-scroll">
                    { for nodes }
                </div>
            </div>
        </ContextProvider<Context>>
    }
}

#[derive(Clone)]
struct NodeState {
    node: Rc<Node>,
    editing: bool,
}

impl NodeState {
    fn editing(node: Rc<Node>) -> NodeState {
        NodeState { node, editing: true }
    }

    fn viewing(node: Rc<Node>) -> NodeState {
        NodeState { node, editing: false }
    }
}

