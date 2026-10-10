use yew::prelude::*;

use crate::components::icons::{Email, GitHub, LinkedIn, ORCID};

#[component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="footer shell">
            <div class="social-links">
                <a href="https://github.com/geyerandreas" target="_blank" rel="noreferrer" aria-label="GitHub"><GitHub /></a>
                <a href="https://www.linkedin.com/in/geyerandreas" target="_blank" rel="noreferrer" aria-label="LinkedIn"><LinkedIn /></a>
                <a href="https://orcid.org/0000-0001-8213-8350" target="_blank" rel="noreferrer" aria-label="ORCID"><ORCID /></a>
                <a href="mailto:andreas@qbitone.de" aria-label="Email"><Email /></a>
            </div>
            <p>{"Designed and built with Rust and Yew."}</p>
        </footer>
    }
}
