//! Node manipulation models.

#[doc(hidden)]
pub mod viewer;

pub use viewer::Viewer;

use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionResult};
use yew::platform::spawn_local;

use ruinaio_model::{Error, Node};

use crate::{origin, Context};

use std::rc::Rc;
use std::ops::Deref;

#[hook]
fn use_node(id: i32) -> SuspensionResult<Result<Rc<Node>, Error>> {
    let Context { api_client } = use_context::<Context>().unwrap();

    let state = use_state(|| None::<Result<Rc<Node>, Error>>);

    match state.deref() {
        Some(Ok(node)) if node.id == id => Ok(Ok(Rc::clone(node))),
        Some(Err(err)) => Ok(Err(err.clone())),
        _ => {
            // create suspension
            let (s, handle) = Suspension::new();

            // fetch node
            spawn_local(async move {
                let res = api_client.get(
                    format!("{}/api/v1/node/{}", origin(), id)
                )
                    .send()
                    .await
                    .unwrap();

                if res.status().is_success() {
                    let node = res.json::<Node>().await.unwrap();

                    state.set(Some(Ok(Rc::new(node))));
                } else {
                    let error = res.json::<Error>().await.unwrap();

                    state.set(Some(Err(error)));
                }

                handle.resume();
            });

            Err(s)
        }
    }
}

