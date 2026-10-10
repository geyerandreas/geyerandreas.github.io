mod components;
pub(crate) mod constants;

use yew::prelude::*;

use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::sections::{About, Banner, Hero, Principles};

#[component(App)]
fn app() -> Html {
    html! {
        <>
        <Header />
        <main>
            <Hero />
            <Banner />
            <About />
            <Principles />
        </main>
        <Footer />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
