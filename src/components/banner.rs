use dioxus::prelude::*;

pub fn banner(cx: Scope) -> Element {
    cx.render(rsx!(
        section {
            p {
                class: "text-lg text-center text-gray-600 pt-0",
                 "Keep your secret safe 🔐",
            }
        }
    ))
}
