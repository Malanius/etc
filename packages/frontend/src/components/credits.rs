use dioxus::prelude::*;

#[component]
pub fn Credits() -> Element {
    let emoji = "🦀";

    rsx! {
        div { class: "block absolute bottom-5 right-5 text-gray-600 font-base ",
            div {
                "made with {emoji} by "
                span { class: "underline decoration-gray-500",
                    a { href: "https://github.com/Malanius/etc/tree/main/packages/frontend",
                        "malanius"
                    }
                }
            }
        }
    }
}
