use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html!(
        <h1>{"Hi"}</h1>
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}
