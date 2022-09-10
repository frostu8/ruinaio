//! Node manipulation models.

pub mod viewer;

pub use viewer::Viewer;

use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionResult};
use yew::platform::spawn_local;

use ruinaio_model::Node;

use std::rc::Rc;
use std::ops::Deref;

#[hook]
fn use_node(id: i32) -> SuspensionResult<Rc<Node>> {
    let state = use_state(|| None::<Rc<Node>>);

    match state.deref() {
        Some(node) if node.id == id => Ok(Rc::clone(node)),
        _ => {
            // create suspension
            let (s, handle) = Suspension::new();

            // fetch node
            spawn_local(async move {
                let node = reqwest::get(
                    format!("http://127.0.0.1:8080/api/v1/node/{}", id)
                )
                    .await
                    .unwrap()
                    .json::<Node>()
                    .await
                    .unwrap();

                state.set(Some(Rc::new(node)));
                handle.resume();
            });

            Err(s)
        }
    }
}

