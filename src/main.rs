use yew::prelude::*;

#[function_component]
pub fn Root() -> Html {
    html! {
        <h1>{"Hello World!"}</h1>
    }
}

fn main() {
    yew::Renderer::<Root>::new().render();
}
