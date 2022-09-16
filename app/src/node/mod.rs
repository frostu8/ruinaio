//! Node manipulation models.

#[doc(hidden)]
pub mod editor;
#[doc(hidden)]
pub mod viewer;

pub use editor::Editor;
pub use viewer::Viewer;

use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionResult};
use yew::platform::spawn_local;

use ruinaio_model::{Error, Node};

use crate::{origin, Context};

use std::rc::Rc;
use std::ops::Deref;

#[hook]
pub fn use_nodes() -> SuspensionResult<Result<Vec<Rc<Node>>, Error>> {
    let Context { api_client } = use_context::<Context>().unwrap();

    let state = use_state(|| None::<Result<Vec<Rc<Node>>, Error>>);

    match state.deref() {
        Some(Ok(nodes)) => Ok(Ok(nodes.clone())),
        Some(Err(err)) => Ok(Err(err.clone())),
        _ => {
            // create suspension
            let (s, handle) = Suspension::new();

            // fetch node
            spawn_local(async move {
                let res = api_client.get(
                    format!("{}/api/v1/nodes", origin())
                )
                    .send()
                    .await
                    .unwrap();

                if res.status().is_success() {
                    let nodes = res.json::<Vec<Node>>().await.unwrap();

                    state.set(Some(Ok(nodes.into_iter().map(|node| Rc::new(node)).collect())));
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

