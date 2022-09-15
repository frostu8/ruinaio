use yew::prelude::*;
use yew::platform::spawn_local;

use crate::{origin, Context, input::title::{TitleInput, Title}};

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
    Create(Title),
}

/// Panel menu.
#[function_component(Menu)]
pub fn menu(props: &Props) -> Html {
    let Context { api_client } = use_context::<Context>().unwrap();

    let state = use_state(|| State::Index);
    let loading = use_state(|| false);

    match &*state {
        State::Index => {
            let action_new = Callback::from(move |_| state.set(State::Create(Title::default())));

            html! {
                <div class={classes!("d-flex", props.class.clone())}>
                    <button class="btn btn-primary mr-3" type="button" onclick={action_new}>{ "New" }</button>
                    <button class="btn btn-outline-secondary mx-3" type="button" disabled=true>{ "Search" }</button>
                </div>
            }
        }
        State::Create(title) if *loading => {
            html! {
                <div class={classes!("d-flex", props.class.clone())}>
                    <div class="input-group">
                        <button class="btn btn-outline-secondary" type="button" disabled=true>{ "Back" }</button>
                        if let Some(namespace) = title.namespace.as_ref() {
                            <span class="input-group-text text-bg-dark">{ namespace }</span>
                        }
                        <input type="text" class="form-control text-bg-dark" maxlength=128 value={ title.title.clone() } disabled=true/>
                        <button class="btn btn-primary" type="button" disabled=true>{ "Create" }</button>
                    </div>
                </div>
            }
        }
        State::Create(title) => {
            let can_submit = slugify(&title.title).is_ok()
                && title.title.len() > 0 
                && title.title.len() <= 128;

            let action_back = {
                let state = state.clone();
                Callback::from(move |_| state.set(State::Index))
            };

            let action_create = {
                let state = state.clone();
                let loading = loading.clone();
                let onnew = props.onnew.clone();
                let title = title.clone();

                Callback::from(move |_: ()| {
                    loading.set(true);

                    let title = title.clone();
                    let api_client = api_client.clone();
                    let state = state.clone();
                    let loading = loading.clone();
                    let onnew = onnew.clone();

                    spawn_local(async move {
                        let res = api_client.post(
                            format!("{}/api/v1/nodes/new", origin())
                        )
                            .json(&CreateNode {
                                namespace: title.namespace.clone(),
                                title: title.title.clone(),
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

            let oninput = {
                let state = state.clone();
                Callback::from(move |title| state.set(State::Create(title)))
            };

            let onkeydown = if can_submit {
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
                    <div class="input-group" {onkeydown}>
                        <button class="btn btn-outline-secondary" type="button" onclick={action_back}>{ "Back" }</button>
                        <TitleInput value={title.clone()} {oninput}/>
                        <button class="btn btn-primary" type="button" onclick={action_create.reform(|_| ())} disabled={!can_submit}>{ "Create" }</button>
                    </div>
                </div>
            }
        }
    }
}

