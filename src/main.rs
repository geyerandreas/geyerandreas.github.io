use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <main style="display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh; font-family: sans-serif;">
            <h1>{"Hallo aus Rust!"}</h1>
            <p>{"Diese Webseite wurde mit Rust & WebAssembly gebaut und läuft auf GitHub Pages."}</p>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
