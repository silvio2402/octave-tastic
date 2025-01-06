use yew::prelude::*;
use yew::Properties;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_loading: bool,
}

#[function_component]
pub fn App() -> Html {
    let cool_bool = use_state(|| false);
    let onclick = {
        let cool_bool = cool_bool.clone();
        move |_| {
            cool_bool.set(!*cool_bool);
        }
    };

    html! {
        <div>
            <button {onclick}>{"change it"}</button>
            <HelloWorld is_loading={*cool_bool} />
        </div>
    }
}

#[function_component]
fn HelloWorld(props: &Props) -> Html {
    html! {
        <p>{props.is_loading.clone()}</p>
    }
}
