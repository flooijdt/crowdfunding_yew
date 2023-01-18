use yew::prelude::*;

struct Counter {
    counter: u32,
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Counter { counter: 0 });

    let onclick = {
        let state = state.clone();

        Callback::from(move |_| {
            state.set(Counter {
                counter: state.counter + 1,
            })
        })
    };

    html!(
        <>
        <label>{ state.counter }</label><br/>
        <button {onclick}>{"+1"}</button>
        </>
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}
