//! Node viewer.

use yew::prelude::*;
use yew::platform::spawn_local;

use pulldown_cmark::{html, Parser, Options, LinkType, BrokenLink, CowStr};

use crate::{origin, Context};

use ruinaio_model::{Error, Node};

use std::rc::Rc;

/// Properties for a [`Viewer`].
#[derive(Properties, PartialEq)]
pub struct Props {
    /// The node to view.
    pub node: Rc<Node>,
    /// An event that is called to edit the node.
    #[prop_or_default]
    pub onedit: Callback<Rc<Node>>,
    /// An event that is called to delete the node.
    #[prop_or_default]
    pub ondelete: Callback<()>,
}

/// A single card viewer for a node.
#[function_component(Viewer)]
pub fn viewer(props: &Props) -> Html {
    let Context { api_client } = use_context::<Context>().unwrap();

    // render body
    let mut broken_link_callback = broken_link_callback;

    let parser = Parser::new_with_broken_link_callback(
        &props.node.body,
        Options::all() & !Options::ENABLE_HEADING_ATTRIBUTES,
        Some(&mut broken_link_callback),
    );

    let mut body = String::new();
    html::push_html(&mut body, parser);

    // create element
    let body_div = gloo::utils::document().create_element("div").unwrap();
    body_div.set_inner_html(&body);

    let onedit = {
        let onedit = props.onedit.clone();
        let node = props.node.clone();

        Callback::from(move |_| onedit.emit(node.clone()))
    };

    let ondelete = {
        let ondelete = props.ondelete.clone();
        let node = props.node.clone();

        Callback::from(move |_| {
            let ondelete = ondelete.clone();
            let api_client = api_client.clone();
            let node = node.clone();

            spawn_local(async move {
                let res = api_client.delete(
                    format!("{}/api/v1/node/{}", origin(), node.id)
                )
                    .send()
                    .await
                    .unwrap();

                if res.status().is_success() {
                    ondelete.emit(());
                } else {
                    let error = res.json::<Error>().await.unwrap();

                    // TODO: handle error
                }
            });
        })
    };

    html! {
        <div class="card text-bg-dark my-3">
            <div class="card-body">
                <div class="d-flex">
                    <h6 class="card-subtitle me-auto text-muted">{ &props.node.slug }</h6>
                    <button class="btn btn-link py-0" onclick={onedit}>
                        <i class="bi bi-pencil-fill"></i>
                    </button>
                    <button class="btn btn-link py-0" onclick={ondelete}>
                        <i class="bi bi-trash-fill"></i>
                    </button>
                </div>
                <h1 class="card-title">{ &props.node.title }</h1>
                { Html::VRef(body_div.into()) }
            </div>
        </div>
    }
}

fn broken_link_callback<'a>(link: BrokenLink<'a>) -> Option<(CowStr<'a>, CowStr<'a>)> {
    // copy shortcut to target
    match link.link_type {
        LinkType::Shortcut => Some((link.reference.clone(), link.reference)),
        _ => None,
    }
}


