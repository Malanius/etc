use crate::prelude::*;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TimerProps {
    remaining_days: String,
    remaining_hours: String,
    remaining_minutes: String,
    remaining_seconds: String,
    timer_style: String,
}

#[component]
pub fn Timer(props: TimerProps) -> Element {
    let TimerProps {
        remaining_days,
        remaining_hours,
        remaining_minutes,
        remaining_seconds,
        timer_style,
    } = props;

    rsx! {
        div {
            style: "{timer_style}",
            class: "flex justify-center text-lg sm:text-xl md:text-2xl mb-3 sm:mb-4 md:mb-6",
            TimerDigit { value: remaining_days, label: "days", useSeparator: false }
            TimerDigit { value: remaining_hours, label: "hours", useSeparator: true }
            TimerDigit { value: remaining_minutes, label: "minutes", useSeparator: true }
            TimerDigit { value: remaining_seconds, label: "seconds", useSeparator: true }
        }
    }
}
