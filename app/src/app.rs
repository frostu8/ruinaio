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
    let context = Context { api_client: Client::new() };

    let fallback = html! {
        <div class="text-center my-3">
            <div class="spinner-border" style="width: 3rem; height: 3rem;" role="status">
                <span class="visually-hidden">{ "Loading..." }</span>
            </div>
        </div>
    };

    html! {
        <ContextProvider<Context> {context}>
            <Suspense {fallback}>
                <Content />
            </Suspense>
        </ContextProvider<Context>>
    }
}

#[function_component(Content)]
fn content() -> HtmlResult {
    let nodes = match crate::node::use_nodes()? {
        Ok(nodes) => nodes,
        Err(e) => return Ok(html! {
            <h1>{ e }</h1>
        }),
    };

    let state = use_state(|| nodes.into_iter().map(|n| NodeState::viewing(n)).collect::<Vec<NodeState>>());

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

    Ok(html! {
        <div class="container d-flex flex-column vh-100">
            <Menu class="my-3" {onnew}/>
            <div class="overflow-scroll">
                { for nodes }
            </div>
        </div>
    })
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

