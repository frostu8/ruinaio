//! Node editor.

use yew::prelude::*;

use ruinaio_model::{Node, slug};

use crate::input::title::{Title, TitleInput};

use std::rc::Rc;

/// Properties for an [`Editor`].
#[derive(Properties, PartialEq)]
pub struct Props {
    /// The node to edit.
    pub node: Rc<Node>,
}

#[derive(Default)]
struct State {
    title: Title,
}

/// A single card editor for a node.
#[function_component(Editor)]
pub fn editor(props: &Props) -> Html {
    let (namespace, _) = slug::split(&props.node.slug);

    let state = use_state(|| State {
        title: Title {
            namespace: namespace.map(|s| s.to_owned()),
            title: props.node.title.clone(),
        },
    });

    let title_changed = props.node.title != state.title.title
        || namespace != state.title.namespace.as_ref().map(|s| s.as_str());

    let oninput = {
        let state = state.clone();

        Callback::from(move |title| state.set(State { title }))
    };

    html! {
        <div class="card text-bg-dark my-3">
            <div class="card-body">
                <div class="input-group">
                    if title_changed {
                        <span class="input-group-text text-bg-warning">
                            <i class="bi bi-exclamation-triangle-fill"></i>
                        </span>
                    }
                    <TitleInput value={ state.title.clone() } {oninput}/>
                </div>
                <textarea class="mb-2 form-control text-bg-dark font-monospace" >{ &state.body }</textarea>
            </div>
        </div>
    }
}


