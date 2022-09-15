use yew::prelude::*;

use web_sys::HtmlInputElement;

use ruinaio_model::slug::slugify;

/// The value of [`TitleInput`].
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Title {
    pub namespace: Option<String>,
    pub title: String,
}

#[doc(hidden)]
#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub value: Title,
    #[prop_or_default]
    pub oninput: Callback<Title>,
}

/// A title input box.
#[function_component(TitleInput)]
pub fn title_input(props: &Props) -> Html {
    let state = use_state(|| props.value.clone());

    let input_ref = use_node_ref();
    let oninput = {
        let input_ref = input_ref.clone();
        let state = state.clone();
        let oninput = props.oninput.clone();

        Callback::from(move |_| {
            let value = input_ref
                .cast::<HtmlInputElement>()
                .unwrap()
                .value();

            if let Some(idx) = value.rfind('/') {
                // move to namespace
                let appendage = slugify(&value[..idx]).unwrap();
                let value = value[idx+1..].to_owned();

                let namespace = match state.namespace.as_ref() {
                    Some(namespace) => Some(format!("{}{}/", namespace, appendage)),
                    None => Some(format!("{}/", appendage)),
                };

                let result = Title { namespace, title: value };
                oninput.emit(result.clone());
                state.set(result);
            } else {
                let result = Title { namespace: state.namespace.clone(), title: value };
                oninput.emit(result.clone());
                state.set(result);
            }
        })
    };

    let onkeydown = if state.namespace.is_some() {
        let state = state.clone();
        let oninput = props.oninput.clone();
        let input_ref = input_ref.clone();

        Callback::from(move |ev: KeyboardEvent| {
            let input_ref = input_ref
                .cast::<HtmlInputElement>()
                .unwrap();
            let selection_start = input_ref
                .selection_start()
                .unwrap()
                .unwrap();

            if ev.key() == "Backspace" && selection_start == 0 {
                ev.prevent_default();

                // remove trailing '/'
                let namespace = state.namespace.as_ref().unwrap();
                let namespace = &namespace[..namespace.len()-1];

                match namespace.rfind('/') {
                    Some(idx) => {
                        let title = namespace[idx+1..].to_owned() + &state.title;
                        let namespace = namespace[..idx+1].to_owned();

                        let result = Title { namespace: Some(namespace), title };
                        oninput.emit(result.clone());
                        state.set(result);
                    }
                    None => {
                        let result = Title { namespace: None, title: namespace.to_owned() + &state.title };
                        oninput.emit(result.clone());
                        state.set(result);
                    }
                }
            }
        })
    } else {
        Callback::default()
    };

    html! {
        <>
            if let Some(namespace) = state.namespace.as_ref() {
                <span class="input-group-text text-bg-dark">{ namespace }</span>
            }
            <input type="text" class="form-control text-bg-dark" maxlength=128 value={ state.title.clone() } ref={input_ref} {oninput} {onkeydown}/>
        </>
    }
}

