use yew::prelude::*;

use crate::constants::EMAIL;

#[component(Header)]
pub fn header() -> Html {
    html! {
        <header class="site-header">
            <nav class="nav shell" aria-label="Main navigation">
                <a class="brand" href="#top" aria-label="Back to top">
                    <span class="brand-mark">{"AG"}</span>
                    <span>{"Andreas Geyer"}</span>
                </a>
                <div class="nav-links">
                    <a href="#about"><span>{"01."}</span>{" About"}</a>
                    <a href="#principles"><span>{"02."}</span>{" Principles"}</a>
                    <a class="nav-cta" href={format!("mailto:{EMAIL}")}>{"Get in touch"}</a>
                </div>
            </nav>
        </header>
    }
}
