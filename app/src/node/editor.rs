//! Node editor.

use yew::prelude::*;

use ruinaio_model::{Node, slug};

use web_sys::HtmlInputElement;

use std::rc::Rc;

/// Properties for an [`Editor`].
#[derive(Properties, PartialEq)]
pub struct Props {
    /// The node to edit.
    pub node: Rc<Node>,
}

struct State {
    namespace: Option<String>,
    title: String,
    body: String,
}

enum Update {
    Title(String),
    Body(String),
}

impl Reducible for State {
    type Action = Update;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Update::Title(title) => Rc::new(State {
                namespace: self.namespace.clone(),
                body: self.body.clone(),
                title,
            }),
            Update::Body(body) => Rc::new(State {
                namespace: self.namespace.clone(),
                title: self.title.clone(),
                body,
            }),
        }
    }
}

/// A single card editor for a node.
#[function_component(Editor)]
pub fn editor(props: &Props) -> Html {
    let (namespace, _) = slug::split(&props.node.slug);

    let state = use_reducer(|| State {
        namespace: namespace.map(|s| s.to_owned()),
        title: props.node.title.clone(),
        body: props.node.body.clone(),
    });

    // generate slug
    let slug = slug::slugify(&state.title).unwrap_or("".into());
    let slug = match state.namespace.clone() {
        Some(namespace) => namespace + &slug,
        None => slug.into_owned(),
    };

    let title_ref = use_node_ref();
    let body_ref = use_node_ref();

    let oninput = {
        let title_ref = title_ref.clone();
        let state = state.clone();

        Callback::from(move |_| {
            let value = title_ref
                .cast::<HtmlInputElement>()
                .unwrap()
                .value();

            // update title
            state.dispatch(Update::Title(value));
        })
    };

    html! {
        <div class="card text-bg-dark my-3">
            <div class="card-body">
                <h6 class="card-subtitle mb-2 text-muted">{ slug }</h6>
                <input class="mb-2 form-control text-bg-dark" type="text" maxlength=128 value={ state.title.clone() } {oninput} ref={title_ref}/>
                <textarea class="mb-2 form-control text-bg-dark font-monospace">{ &state.body }</textarea>
            </div>
        </div>
    }
}


