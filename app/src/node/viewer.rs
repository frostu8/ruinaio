//! Node viewer.

use yew::prelude::*;

use pulldown_cmark::{html, Parser, Options, LinkType, BrokenLink, CowStr};

use ruinaio_model::Node;

use std::rc::Rc;

/// Properties for a [`Viewer`].
#[derive(Properties, PartialEq)]
pub struct Props {
    /// The node to view.
    pub node: Rc<Node>,
}

/// A single card viewer for a node.
#[function_component(Viewer)]
pub fn viewer(props: &Props) -> Html {
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

    html! {
        <div class="card text-bg-dark my-3">
            <div class="card-body">
                <h6 class="card-subtitle mb-2 text-muted">{ &props.node.slug }</h6>
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


