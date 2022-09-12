use yew::prelude::*;
use yew::platform::spawn_local;

use web_sys::HtmlInputElement;

use crate::{origin, Context};

use ruinaio_model::{Node, Error, params::CreateNode};

/// Props for [`Menu`].
#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,
    pub onnew: Callback<Node>,
}

enum State {
    Index,
    Create(String),
}

/// Panel menu.
#[function_component(Menu)]
pub fn menu(props: &Props) -> Html {
    let Context { api_client } = use_context::<Context>().unwrap();

    let state = use_state(|| State::Index);
    let loading = use_state(|| false);

    match &*state {
        State::Index => {
            let action_new = Callback::from(move |_| state.set(State::Create(String::new())));

            html! {
                <div class={classes!("d-flex", props.class.clone())}>
                    <button class="btn btn-primary mr-3" type="button" onclick={action_new}>{ "New" }</button>
                    <button class="btn btn-outline-secondary mx-3" type="button" disabled=true>{ "Search" }</button>
                </div>
            }
        }
        State::Create(title) if *loading => {
            html! {
                <div class={classes!("input-group", props.class.clone())}>
                    <div class="input-group-prepend">
                        <button class="btn btn-outline-secondary" type="button" disabled=true>{ "Back" }</button>
                    </div>
                    <input type="text" class="form-control text-bg-dark" value={title.clone()} disabled=true/>
                    <div class="input-group-append">
                        <button class="btn btn-primary" type="button" disabled=true>
                            //<span class="invisible">{ "Create" }</span>
                            <div class="spinner-border spinner-border-sm text-light" role="status">
                                <span class="visually-hidden">{ "Loading..." }</span>
                            </div>
                        </button>
                    </div>
                </div>
            }
        }
        State::Create(title) => {
            let can_submit = title.len() > 0;

            let action_back = {
                let state = state.clone();
                Callback::from(move |_| state.set(State::Index))
            };

            let action_create = {
                let title = title.clone();
                let state = state.clone();
                let loading = loading.clone();
                let onnew = props.onnew.clone();

                Callback::from(move |_| {
                    loading.set(true);

                    let title = title.clone();
                    let api_client = api_client.clone();
                    let state = state.clone();
                    let loading = loading.clone();
                    let onnew = onnew.clone();

                    spawn_local(async move {
                        let slug = ruinaio_model::slug::slugify(&title).unwrap();

                        let res = api_client.post(
                            format!("{}/api/v1/nodes/new", origin())
                        )
                            .json(&CreateNode {
                                slug: slug.into_owned(),
                                body: format!("# {}", title),
                            })
                            .send()
                            .await
                            .unwrap();

                        if res.status().is_success() {
                            let node = res.json::<Node>().await.unwrap();

                            onnew.emit(node);
                            loading.set(false);
                            state.set(State::Index);
                        } else {
                            let error = res.json::<Error>().await.unwrap();

                            loading.set(false);
                            // TODO: handle error
                        }
                    });
                })
            };

            let input_ref = NodeRef::default();
            let input = {
                let input_ref = input_ref.clone();
                let state = state.clone();

                Callback::from(move |_| {
                    state.set(State::Create(input_ref
                            .cast::<HtmlInputElement>()
                            .unwrap()
                            .value()))
                })
            };

            html! {
                <div class={classes!("input-group", props.class.clone())}>
                    <div class="input-group-prepend">
                        <button class="btn btn-outline-secondary" type="button" onclick={action_back}>{ "Back" }</button>
                    </div>
                    <input type="text" class="form-control text-bg-dark" value={title.clone()} ref={input_ref} oninput={input}/>
                    <div class="input-group-append">
                        <button class="btn btn-primary" type="button" onclick={action_create} disabled={!can_submit}>{ "Create" }</button>
                    </div>
                </div>
            }
        }
    }
}

