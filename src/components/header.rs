use yew::prelude::*;

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
                    <a href="#experience"><span>{"02."}</span>{" Experience"}</a>
                    <a href="#work"><span>{"03."}</span>{" Work"}</a>
                    <a class="nav-cta" href="#contact">{"Get in touch"}</a>
                </div>
            </nav>
        </header>
    }
}
