use yew::prelude::*;

#[component(Header)]
pub fn header() -> Html {
    html! {
        <header>
            <div>
                <h1>{"Andreas Geyer"}</h1>
            </div>
        </header>
    }
}
