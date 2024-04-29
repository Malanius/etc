use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TimerDigitProps {
    value: String,
    label: String,
    useSeparator: bool,
}

#[component]
pub fn TimerDigit(props: TimerDigitProps) -> Element {
    let TimerDigitProps {
        value,
        label,
        useSeparator,
    } = props;
    let separator = if useSeparator { ":" } else { "" };

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
