use yew::prelude::*;

/// Props for [`Menu`].
#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,
}

/// Panel menu.
#[function_component(Menu)]
pub fn menu(props: &Props) -> Html {
    html! {
        <div class={classes!("input-group", props.class.clone())}>
            <input type="text" class="form-control text-bg-dark"/>
            <div class="input-group-append">
                <button class="btn btn-primary" type="button">{ "Create" }</button>
            </div>
        </div>
    }
}

