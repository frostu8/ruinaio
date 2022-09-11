//! Node viewer.

use yew::prelude::*;

use super::use_node;

use pulldown_cmark::{html, Parser, Options, LinkType, BrokenLink, CowStr};

/// Properties for a [`Viewer`].
#[derive(Properties, PartialEq)]
pub struct Props {
    /// The id of the node.
    ///
    /// This is required to actually view nodes and be useful.
    pub id: i32,
}

/// A single card viewer for a node.
#[function_component(Viewer)]
pub fn viewer(props: &Props) -> Html {
    let fallback = html! {
        <div class="card text-bg-dark my-3">
            <div class="card-body">
                <h5 class="card-title mb-2 text-muted">{ format!("loading #{}...", props.id) }</h5>
            </div>
        </div>
    };

    html! {
        <Suspense {fallback}>
            <Content id={props.id}/>
        </Suspense>
    }
}

#[function_component(Content)]
fn content(props: &Props) -> HtmlResult {
    match use_node(props.id)? {
        Ok(node) => {
            // render body
            let mut broken_link_callback = broken_link_callback;

            let parser = Parser::new_with_broken_link_callback(
                &node.body,
                Options::all() & !Options::ENABLE_HEADING_ATTRIBUTES,
                Some(&mut broken_link_callback),
            );

            let mut body = String::new();
            html::push_html(&mut body, parser);

            // create element
            let body_div = gloo::utils::document().create_element("div").unwrap();
            body_div.set_inner_html(&body);

            Ok(html! {
                <div class="card text-bg-dark my-3">
                    <div class="card-body">
                        <h6 class="card-subtitle mb-2 text-muted">{ &node.slug }</h6>
                        { Html::VRef(body_div.into()) }
                    </div>
                </div>
            })
        }
        Err(err) => {
            // format error
            Ok(html! {
                <div class="card text-bg-danger my-3">
                    <div class="card-body">
                        <h5 class="card-title">{ "failed to retrieve node" }</h5>
                        <h6 class="card-subtitle">{ err.to_string() }</h6>
                    </div>
                </div>
            })
        }
    }
}

fn broken_link_callback<'a>(link: BrokenLink<'a>) -> Option<(CowStr<'a>, CowStr<'a>)> {
    // copy shortcut to target
    match link.link_type {
        LinkType::Shortcut => Some((link.reference.clone(), link.reference)),
        _ => None,
    }
}


