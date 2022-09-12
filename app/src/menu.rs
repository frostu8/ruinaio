use yew::prelude::*;
use yew::platform::spawn_local;

use web_sys::HtmlInputElement;

use crate::{origin, Context};

use ruinaio_model::{Node, Error, params::CreateNode, slug::slugify};

/// Props for [`Menu`].
#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,
    pub onnew: Callback<Node>,
}

enum State {
    Index,
    Create(String, String),
}

/// Panel menu.
#[function_component(Menu)]
pub fn menu(props: &Props) -> Html {
    let Context { api_client } = use_context::<Context>().unwrap();

    let state = use_state(|| State::Index);
    let loading = use_state(|| false);

    match &*state {
        State::Index => {
            let action_new = Callback::from(move |_| state.set(State::Create(String::new(), String::new())));

            html! {
                <div class={classes!("d-flex", props.class.clone())}>
                    <button class="btn btn-primary mr-3" type="button" onclick={action_new}>{ "New" }</button>
                    <button class="btn btn-outline-secondary mx-3" type="button" disabled=true>{ "Search" }</button>
                </div>
            }
        }
        State::Create(namespace, title) if *loading => {
            html! {
                <div class={classes!("d-flex", props.class.clone())}>
                    <button class="btn btn-outline-secondary" type="button" disabled=true>{ "Back" }</button>
                    <p class="px-2 py-1 m-0 text-nowrap">{ namespace }</p>
                    <div class="input-group">
                        <input type="text" class="form-control text-bg-dark" maxlength=128 value={title.clone()} disabled=true/>
                        <div class="input-group-append">
                            <button class="btn btn-primary" type="button" disabled=true>
                                //<span class="invisible">{ "Create" }</span>
                                <div class="spinner-border spinner-border-sm text-light" role="status">
                                    <span class="visually-hidden">{ "Loading..." }</span>
                                </div>
                            </button>
                        </div>
                    </div>
                </div>
            }
        }
        State::Create(namespace, title) => {
            let can_submit = title.len() > 0 && title.len() <= 128;

            let action_back = {
                let state = state.clone();
                Callback::from(move |_| state.set(State::Index))
            };

            let action_create = {
                let title = title.clone();
                let namespace = namespace.clone();
                let state = state.clone();
                let loading = loading.clone();
                let onnew = props.onnew.clone();

                Callback::from(move |_: ()| {
                    loading.set(true);

                    let title = title.clone();
                    let namespace = namespace.clone();
                    let api_client = api_client.clone();
                    let state = state.clone();
                    let loading = loading.clone();
                    let onnew = onnew.clone();

                    spawn_local(async move {
                        let res = api_client.post(
                            format!("{}/api/v1/nodes/new", origin())
                        )
                            .json(&CreateNode {
                                namespace: Some(namespace),
                                title,
                                body: String::new(),
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
            let oninput = {
                let input_ref = input_ref.clone();
                let state = state.clone();
                let namespace = namespace.clone();

                Callback::from(move |_| {
                    let namespace = namespace.clone();

                    let value = input_ref
                        .cast::<HtmlInputElement>()
                        .unwrap()
                        .value();

                    if let Some(idx) = value.rfind('/') {
                        // move to namespace
                        let appendage = slugify(&value[..idx]).unwrap();
                        let value = value[idx+1..].to_owned();

                        state.set(State::Create(format!("{}{}/", namespace, &appendage), value))
                    } else {
                        state.set(State::Create(namespace, value))
                    }
                })
            };

            let onkeydown = if title.is_empty() && !namespace.is_empty() {
                let state = state.clone();
                let namespace = namespace.clone();

                Callback::from(move |ev: KeyboardEvent| {
                    if ev.key() == "Backspace" {
                        ev.prevent_default();

                        // remove trailing '/'
                        let namespace = &namespace[..namespace.len()-1];

                        match namespace.rfind('/') {
                            Some(idx) => {
                                let title = namespace[idx+1..].to_owned();
                                let namespace = namespace[..idx+1].to_owned();

                                state.set(State::Create(namespace, title))
                            }
                            None => {
                                state.set(State::Create(String::new(), namespace.to_owned()))
                            }
                        }
                    }
                })
            } else if can_submit {
                let action_create = action_create.clone();

                Callback::from(move |ev: KeyboardEvent| {
                    if ev.key() == "Enter" {
                        action_create.emit(());
                    }
                })
            } else {
                Callback::default()
            };

            html! {
                <div class={classes!("d-flex", props.class.clone())}>
                    <button class="btn btn-outline-secondary" type="button" onclick={action_back}>{ "Back" }</button>
                    <p class="px-2 py-1 m-0 text-nowrap">{ namespace }</p>
                    <div class="input-group">
                        <input type="text" class="form-control text-bg-dark" maxlength=128 value={title.clone()} ref={input_ref} {oninput} {onkeydown}/>
                        <div class="input-group-append">
                            <button class="btn btn-primary" type="button" onclick={action_create.reform(|_| ())} disabled={!can_submit}>{ "Create" }</button>
                        </div>
                    </div>
                </div>
            }
        }
    }
}

