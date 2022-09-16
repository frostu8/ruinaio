use super::Context;

use yew::prelude::*;
use yew::platform::spawn_local;

use std::rc::Rc;

use crate::node::{Editor, Viewer};
use crate::menu::Menu;

use reqwest::Client;

use ruinaio_model::Node;

/// The main application logic.
#[function_component(App)]
pub fn app() -> Html {
    let context = Context { api_client: Client::new() };
    let state = use_state(Vec::<NodeState>::default);

    // TODO: move this to its own dedicated system, out of this poor function
    // component
    if state.is_empty() {
        let api_client = context.api_client.clone();
        let state_ref = state.clone();
        spawn_local(async move {
            let res = api_client.get(
                format!("{}/api/v1/nodes", super::origin())
            )
                .send()
                .await
                .unwrap();

            if res.status().is_success() {
                let nodes = res.json::<Vec<Node>>().await.unwrap();

                state_ref.set(nodes.into_iter().map(|n| NodeState::viewing(Rc::new(n))).collect());
            } else {
                let error = res.json::<ruinaio_model::Error>().await.unwrap();

                // TODO: handle error
            }
        });
    }

    let onnew = {
        let state = state.clone();

        Callback::from(move |node| {
            let mut new_state = (*state).clone();

            new_state.push(NodeState::editing(Rc::new(node)));
            state.set(new_state);
        })
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
                    let onedit = {
                        let state_ref = state.clone();
                        Callback::from(move |node| {
                            let mut nodes = (*state_ref).clone();
                            
                            nodes[i] = NodeState {
                                editing: true,
                                node,
                            };

                            state_ref.set(nodes);
                        })
                    };

                    let ondelete = Callback::from(move |()| {
                        let mut nodes = (*state_ref).clone();

                        nodes.remove(i);

                        state_ref.set(nodes);
                    });

                    html! { <Viewer node={node.node.clone()} {onedit} {ondelete} /> }
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

