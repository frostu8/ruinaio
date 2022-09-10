//! Node viewer.

use yew::prelude::*;

use super::use_node;

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
    let fallback = html! { <h2>{ props.id }</h2> };

    html! {
        <Suspense {fallback}>
            <Content id={props.id}/>
        </Suspense>
    }
}

#[function_component(Content)]
fn content(props: &Props) -> HtmlResult {
    let node = use_node(props.id)?;

    Ok(html! {
        <div>
            <h1>{ &node.title }</h1>
            <p>{ &node.body }</p>
        </div>
    })
}

