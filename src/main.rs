use yew::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <h1>{ "Hello Shanto!" }</h1>
        </div>
    }
}
