//! Node editor.

use yew::prelude::*;
use yew::platform::spawn_local;

use ruinaio_model::{params::UpdateNode, Node, Error, Patch, slug::slugify};

use crate::{origin, Context, input::title::{Title, TitleInput}};

use web_sys::HtmlTextAreaElement;

use std::rc::Rc;

/// Properties for an [`Editor`].
#[derive(Properties, PartialEq)]
pub struct Props {
    /// The node to edit.
    pub node: Rc<Node>,
    /// A callback for when the node is updated.
    #[prop_or_default]
    pub onupdate: Callback<Node>,
}

#[derive(Default)]
struct State {
    title: Title,
}

/// A single card editor for a node.
#[function_component(Editor)]
pub fn editor(props: &Props) -> Html {
    let Context { api_client } = use_context::<Context>().unwrap();

    let state = use_state(|| State {
        title: Title {
            namespace: props.node.namespace().map(|s| s.to_owned()),
            title: props.node.title.clone(),
        },
    });
    let loading = use_state(|| false);

    let can_submit = slugify(&state.title.title).is_ok()
        && state.title.title.len() > 0 
        && state.title.title.len() <= 128;
    let title_changed = props.node.title != state.title.title;
    let namespace_changed = props.node.namespace() != state.title.namespace.as_ref().map(|s| s.as_str());

    let body_ref = use_node_ref();

    if *loading {
        return html! {
            <div class="card text-bg-dark my-3">
                <div class="card-body">
                    <div class="input-group mb-2">
                        <TitleInput value={ state.title.clone() }/>
                    </div>
                    <textarea class="mb-2 form-control text-bg-dark font-monospace" value={ props.node.body.clone() } rows=16/>
                    <button class="btn btn-primary" disabled=true>
                        <div class="spinner-border spinner-border-sm text-light" role="status">
                            <span class="visually-hidden">{ "Loading..." }</span>
                        </div>
                    </button>
                </div>
            </div>
        }
    }

    let oninput = {
        let state = state.clone();

        Callback::from(move |title| state.set(State { title }))
    };

    let onclick = {
        let node = props.node.clone();
        let state = state.clone();
        let onupdate = props.onupdate.clone();
        let body_ref = body_ref.clone();
        let loading = loading.clone();

        Callback::from(move |_| {
            let api_client = api_client.clone();
            let node = node.clone();
            let state = state.clone();
            let onupdate = onupdate.clone();
            let loading = loading.clone();

            loading.set(true);

            // get value
            let body = body_ref
                .cast::<HtmlTextAreaElement>()
                .unwrap()
                .value();

            // update
            spawn_local(async move {
                let res = api_client.patch(
                    format!("{}/api/v1/node/{}", origin(), node.id)
                )
                    .json(&UpdateNode {
                        namespace: if namespace_changed {
                            match state.title.namespace.clone() {
                                Some(namespace) => Patch::Some(namespace),
                                None => Patch::Null,
                            }
                        } else {
                            Patch::None
                        },
                        title: if title_changed {
                            Some(state.title.title.clone())
                        } else {
                            None
                        },
                        body: if node.body != body {
                            Some(body)
                        } else {
                            None
                        },
                    })
                    .send()
                    .await
                    .unwrap();

                if res.status().is_success() {
                    let node = res.json::<Node>().await.unwrap();

                    onupdate.emit(node);
                    loading.set(false);
                } else {
                    let error = res.json::<Error>().await.unwrap();

                    // TODO: handle error
                    loading.set(false);
                }
            });
        })
    };

    html! {
        <div class="card text-bg-dark my-3">
            <div class="card-body">
                <div class="input-group mb-2">
                    if title_changed || namespace_changed {
                        <span class="input-group-text text-bg-warning">
                            <i class="bi bi-exclamation-triangle-fill"></i>
                        </span>
                    }
                    <TitleInput value={ state.title.clone() } {oninput}/>
                </div>
                <textarea class="mb-2 form-control text-bg-dark font-monospace" ref={body_ref} value={ props.node.body.clone() } rows=16/>
                <button class="btn btn-primary" {onclick} disabled={!can_submit}>{ "Save" }</button>
            </div>
        </div>
    }
}


