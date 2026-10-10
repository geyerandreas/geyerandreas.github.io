mod components;

use yew::prelude::*;

use crate::components::header::Header;

#[component(App)]
fn app() -> Html {
    html! {
        <>
        <Header />
        <main>

        </main>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
