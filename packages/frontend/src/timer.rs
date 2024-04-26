use core::str;
use std::time::Duration;

use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use futures_timer::Delay;

const DEADLINE: &str = "2024-05-11T00:00:00Z";

#[derive(Props, Clone, PartialEq)]
struct TimerDigitProps {
    value: String,
    label: String,
    useSeparator: bool,
}

#[component]
fn TimerDigit(props: TimerDigitProps) -> Element {
    let TimerDigitProps {
        value,
        label,
        useSeparator,
    } = props;
    let separator = if useSeparator { ": " } else { "" };

    rsx! {
        div { class: "w-1/4 min-w-max",
            span {
                class: "text-4xl sm:text-6xl md:text-8xl font-semibold",
                id: "{label}",
                "{separator}{value}"
            }
        }
    }
}

#[component]
pub fn Timer() -> Element {
    let time = use_signal(|| chrono::Utc::now().to_string());
    let deadline: DateTime<Utc> = DEADLINE.parse().expect("failed to parse deadline");

    use_effect(move || {
        let mut time = time.clone();
        spawn(async move {
            loop {
                Delay::new(Duration::from_secs(1)).await;
                time.set(chrono::Utc::now().to_string());
            }
        });
    });

    let now: DateTime<Utc> = time.read().parse().expect("failed to parse time");
    let delta = deadline - now;
    let remaining_days = format!("{:02}", delta.num_days());
    let remaining_hours = format!("{:02}", delta.num_hours() % 24);
    let remaining_minutes = format!("{:02}", delta.num_minutes() % 60);
    let remaining_seconds = format!("{:02}", delta.num_seconds() % 60);

    rsx! {
        div { class: "flex justify-center text-lg sm:text-xl md:text-2xl mb-3 sm:mb-4 md:mb-6",
            TimerDigit { value: remaining_days, label: "days", useSeparator: false }
            TimerDigit { value: remaining_hours, label: "hours", useSeparator: true }
            TimerDigit { value: remaining_minutes, label: "minutes", useSeparator: true }
            TimerDigit { value: remaining_seconds, label: "seconds", useSeparator: true }
        }
    }
}
