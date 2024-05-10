use crate::prelude::*;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TimerProps {
    days: String,
    hours: String,
    minutes: String,
    seconds: String,
    timer_style: String,
}

#[component]
pub fn Timer(props: TimerProps) -> Element {
    let TimerProps {
        days,
        hours,
        minutes,
        seconds,
        timer_style,
    } = props;

    rsx! {
        div {
            style: "{timer_style}",
            class: "flex justify-center text-lg sm:text-xl md:text-2xl mb-3 sm:mb-4 md:mb-6",
            TimerDigit { value: days, label: "days", useSeparator: false }
            TimerDigit { value: hours, label: "hours", useSeparator: true }
            TimerDigit { value: minutes, label: "minutes", useSeparator: true }
            TimerDigit { value: seconds, label: "seconds", useSeparator: true }
        }
    }
}
