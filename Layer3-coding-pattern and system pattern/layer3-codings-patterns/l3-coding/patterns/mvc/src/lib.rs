//! Very small Yew MVC: Model = counter, View = html!, Controller = callbacks.

use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let count = use_state(|| 0);
    let onclick = {
        let count = count.clone();
        Callback::from(move |_| count.set(*count + 1))
    };
    html! {
        <div>
            <h1>{ "Count: " }{ *count }</h1>
            <button {onclick}>{ "Increment" }</button>
        </div>
    }
}

fn main() { yew::Renderer::<App>::new().render(); }
