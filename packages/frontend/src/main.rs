#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;

mod timer;
mod terrors;

mod prelude {
    pub use crate::timer::*;
    pub use crate::terrors::*;
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

#[component]
fn App() -> Element {
    rsx! {
        div { class: "bg-black h-screen flex flex-col items-center justify-center text-white px-2",
            h2 { class: "text-lg sm:text-xl md:text-2xl font-bold mb-3 sm:mb-4 md:mb-6",
                "The terrors of"
            }
            Terror {}
            h2 { class: "text-lg sm:text-xl md:text-2xl font-bold mb-3 sm:mb-4 md:mb-6",
                "begin in:"
            }
            Timer {}
        }
    }
}
