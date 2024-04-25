#![allow(non_snake_case)]

use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use log::LevelFilter;

// Urls are relative to your Cargo.toml file
const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}

const DEADLINE: &str = "2024-05-11T00:00:00Z";

#[component]
fn App() -> Element {
    let deadline: DateTime<Utc> = DEADLINE.parse().expect("failed to parse deadline");
    // let now = use_signal(|| chrono::Utc::now());
    let now = chrono::Utc::now();

    let delta = deadline - now;
    let remaining_days = format!("{:02}", delta.num_days());
    let remaining_hours = format!("{:02}", delta.num_hours() % 24);
    let remaining_minutes = format!("{:02}", delta.num_minutes() % 60);
    let remaining_seconds = format!("{:02}", delta.num_seconds() % 60);

    rsx! {
        div { class: "bg-black h-screen flex flex-col items-center justify-center text-white",
            h1 { class: "text-4xl font-bold mb-6", "Terror starts in:" }

            div { class: "flex text-2xl mb-6",
                div { class: "mr-2",
                    span { class: "font-semibold", id: "days", "{remaining_days}" }
                    " :"
                }
                div { class: "mr-2",
                    span { class: "font-semibold", id: "hours", "{remaining_hours}" }
                    " :"
                }
                div { class: "mr-2",
                    span { class: "font-semibold", id: "minutes", "{remaining_minutes}" }
                    " :"
                }
                div {
                    span { class: "font-semibold", id: "seconds", "{remaining_seconds}" }
                    ""
                }
            }
        }
    }
}
