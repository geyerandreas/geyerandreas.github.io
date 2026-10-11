use yew::prelude::*;

use crate::constants::EMAIL;

#[component(Hero)]
pub fn hero() -> Html {
    html! {
        <section id="top" class="hero shell">
            <div class="hero-copy">
                <p class="eyebrow">{"Hi, my name is"}</p>
                <h1>{"Andreas Geyer."}<br /><span>{"I build things for the web."}</span></h1>
                <p class="hero-intro">
                    <strong>{"Full-Stack Software Engineer"}</strong>
                    <span>{"Somewhere between a blank canvas and a production server."}</span>
                </p>
                <a class="button button-primary" href="#work">{"See my work"}</a>
            </div>
            <div class="hero-orbit" aria-hidden="true">
                <div class="orbit orbit-one"></div>
                <div class="orbit orbit-two"></div>
                <div class="orbit-core">{"</>"}</div>
            </div>
        </section>
    }
}

#[component(Banner)]
pub fn banner() -> Html {
    html! {
        <section class="banner shell" aria-label="Personal slogan">
            <div class="banner-card">
                <p>{"Curious by nature. Engineer by choice"}</p>
            </div>
        </section>
    }
}

#[component(About)]
pub fn about() -> Html {
    html! {
        <section id="about" class="section shell split-section">
            <SectionHeading number="01" title="About me" />
            <div class="about-grid">
                <div class="body-copy">
                    <p>{"With a background in theoretical physics, I'm drawn to understanding how things work — from abstract principles to complex software systems."}</p>
                    <p>{"Today, I channel that curiosity into software engineering: taking ownership, collaborating closely, and building clean, maintainable software. I believe the best solutions start with understanding the problem, measuring what matters, and keeping complexity under control."}</p>
                </div>
            </div>
        </section>
    }
}

#[component(Principles)]
pub fn principles() -> Html {
    html! {
        <section id="principles" class="section shell split-section">
            <SectionHeading number="02" title="My principles" />
            <div class="about-grid">
                <div class="principles">
                    <article class="card">
                        <h3>{"Understand first"}</h3>
                        <p>{"Explore the problem, question assumptions, and understand the system before jumping into implementation."}</p>
                    </article>
                    <article class="card">
                        <h3>{"Measure what matters"}</h3>
                        <p>{"Use metrics to make performance, quality, and improvements observable and comparable."}</p>
                    </article>
                    <article class="card">
                        <h3>{"Keep it maintainable"}</h3>
                        <p>{"Write clean code that's easy to read, reason about, change, and build upon."}</p>
                    </article>
                    <article class="card">
                        <h3>{"Own it together"}</h3>
                        <p>{"Take responsibility, share context, and treat collaboration as an essential part of good engineering."}</p>
                    </article>
                </div>
            </div>
        </section>
    }
}

#[component(Experience)]
pub fn experience() -> Html {
    html! {
        <section id="experience" class="section shell split-section">
            <SectionHeading number="02" title="Where I have worked" />
            <div class="experience-list">
                <article class="experience-item">
                    <p class="experience-date">{"2022 — Present"}</p>
                    <div>
                        <h3>{"Your current role"}<span>{" · "}</span><a href="#contact">{"Company name"}</a></h3>
                        <p>{"Describe the role, the team, and the impact you made. Keep this concise so the strongest details stand out."}</p>
                        <div class="tag-list"><span>{"Rust"}</span><span>{"Yew"}</span><span>{"TypeScript"}</span></div>
                    </div>
                </article>
                <article class="experience-item">
                    <p class="experience-date">{"2020 — 2022"}</p>
                    <div>
                        <h3>{"Previous role"}<span>{" · "}</span><a href="#contact">{"Previous company"}</a></h3>
                        <p>{"Use this space for a short summary of your responsibilities and the outcomes you helped deliver."}</p>
                        <div class="tag-list"><span>{"Web development"}</span><span>{"APIs"}</span><span>{"UX"}</span></div>
                    </div>
                </article>
            </div>
        </section>
    }
}

#[component(Work)]
pub fn work() -> Html {
    html! {
        <section id="work" class="section shell">
            <SectionHeading number="03" title="Some things I have built" />
            <div class="project-grid">
                <article class="project-card featured-project">
                    <div class="project-preview preview-green"><span>{"project / one"}</span></div>
                    <div class="project-content">
                        <p class="eyebrow">{"Featured project"}</p>
                        <h3>{"Project title"}</h3>
                        <p>{"A short description of the project, the problem it solves, and why it matters."}</p>
                        <div class="tag-list"><span>{"Yew"}</span><span>{"Rust"}</span><span>{"WebAssembly"}</span></div>
                        <a class="project-link" href="#contact">{"View project"}<span aria-hidden="true">{" ↗"}</span></a>
                    </div>
                </article>
                <article class="project-card">
                    <div class="project-preview preview-slate"><span>{"project / two"}</span></div>
                    <div class="project-content">
                        <p class="eyebrow">{"Selected work"}</p>
                        <h3>{"Another project"}</h3>
                        <p>{"Replace this card with another project and a sentence about its outcome."}</p>
                        <div class="tag-list"><span>{"Design"}</span><span>{"Frontend"}</span></div>
                        <a class="project-link" href="#contact">{"View project"}<span aria-hidden="true">{" ↗"}</span></a>
                    </div>
                </article>
            </div>
        </section>
    }
}

#[derive(Properties, PartialEq)]
pub struct SectionHeadingProps {
    pub number: &'static str,
    pub title: &'static str,
}

#[component(SectionHeading)]
fn section_heading(props: &SectionHeadingProps) -> Html {
    html! {
        <div class="section-heading">
            <span class="section-number">{format!("{}.", props.number)}</span>
            <h2>{props.title}</h2>
            <span class="heading-rule"></span>
        </div>
    }
}

#[component(Contact)]
pub fn contact() -> Html {
    html! {
        <section id="contact" class="contact section shell">
            <p class="eyebrow">{"04. What is next?"}</p>
            <h2>{"Let's build something together."}</h2>
            <p>{"Have an idea, an opportunity, or just want to say hello? This is a good place for your closing invitation."}</p>
            <a class="button button-primary" href={format!("mailto:{EMAIL}")}>{"Say hello"}</a>
        </section>
    }
}
