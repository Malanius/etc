#![allow(non_snake_case)]

use std::time::Duration;

use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use futures_timer::Delay;
use log::LevelFilter;

mod components;

mod prelude {
    pub use crate::components::prelude::*;
}

use prelude::*;

// Urls are relative to your Cargo.toml file
const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}

const DEADLINE: &str = "2024-05-11T00:00:00Z";
const COUNTER_SATURATION: i32 = 100; // Full saturation
const COUNTER_LIGHTNESS: i32 = 50; // Normal lightness

fn calculate_color(hours_left: usize) -> f32 {
    if hours_left > 336 {
        return 120.0;
    }

    let hue_start = 120.0; // Green
    let hue_end = 0.0; // Red

    hue_start - (hue_start - hue_end) * ((336 - hours_left) as f32 / 336.0)
}

#[component]
fn App() -> Element {
    let time = use_signal(|| chrono::Utc::now().to_string());
    let deadline: DateTime<Utc> = DEADLINE.parse().expect("failed to parse deadline");

    use_effect(move || {
        let mut time = time.clone();
        spawn(async move {
            loop {
                Delay::new(Duration::from_secs(1)).await;
                time.set(chrono::Utc::now().to_string());
                // Move time faster for testing purposes
                // let virtual_clock: DateTime<Utc> =
                //     time.read().parse().expect("failed to parse time");
                // time.set((virtual_clock + chrono::Duration::hours(1)).to_string());
            }
        });
    });

    let now: DateTime<Utc> = time.read().parse().expect("failed to parse time");
    let delta_down = deadline - now;
    let delta_up = now - deadline;

    if delta_down.num_seconds() <= 0 {
        let elapsed_days = format!("{:02}", delta_up.num_days());
        let elapsed_hours = format!("{:02}", delta_up.num_hours() % 24);
        let elapsed_minutes = format!("{:02}", delta_up.num_minutes() % 60);
        let elapsed_seconds = format!("{:02}", delta_up.num_seconds() % 60);
        let counter_hue = calculate_color(delta_up.num_hours() as usize);
        let timer_style =
            format!("color: hsl({counter_hue}, {COUNTER_SATURATION}%, {COUNTER_LIGHTNESS}%);");

        return rsx! {
            div { class: "bg-black h-screen flex flex-col items-center justify-center text-white px-2",
                h1 { class: "text-xl sm:text-2xl md:text-4xl font-bold mb-3 sm:mb-4 md:mb-6",
                    "さよなら, suckers。"
                }
                h2 { class: "text-lg sm:text-xl md:text-2xl font-bold mb-3 sm:mb-4 md:mb-6",
                    "Do you miss me already? It just has been:"
                }
                Timer {
                    days: elapsed_days,
                    hours: elapsed_hours,
                    minutes: elapsed_minutes,
                    seconds: elapsed_seconds,
                    timer_style
                }
                Credits {}
            }
        };
    }

    let counter_hue = calculate_color(delta_down.num_hours() as usize);
    let timer_style =
        format!("color: hsl({counter_hue}, {COUNTER_SATURATION}%, {COUNTER_LIGHTNESS}%);");

    let remaining_days = format!("{:02}", delta_down.num_days());
    let remaining_hours = format!("{:02}", delta_down.num_hours() % 24);
    let remaining_minutes = format!("{:02}", delta_down.num_minutes() % 60);
    let remaining_seconds = format!("{:02}", delta_down.num_seconds() % 60);

    rsx! {
        div { class: "bg-black h-screen flex flex-col items-center justify-center text-white px-2",
            h2 { class: "text-lg sm:text-xl md:text-2xl font-bold mb-3 sm:mb-4 md:mb-6",
                "The terrors of"
            }
            Terror {}
            h2 { class: "text-lg sm:text-xl md:text-2xl font-bold mb-3 sm:mb-4 md:mb-6",
                "begin in:"
            }
            Timer {
                days: remaining_days,
                hours: remaining_hours,
                minutes: remaining_minutes,
                seconds: remaining_seconds,
                timer_style
            }
            Credits {}
        }
    }
}
