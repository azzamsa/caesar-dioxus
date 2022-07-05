use dioxus::prelude::*;

use super::icons;

pub fn footer(cx: Scope) -> Element {
    cx.render(rsx!(
        section { class: "max-w-lg mx-auto flex justify-center text-white font-medium",
              a { class: "duration-500 transform hover:-translate-y-1 hover:scale-125 hover:underline",
                  href: "#",
                  "Support Me"
              }
                  i { class: "inline-block mx-1 align-middle w-4 h-4",
                      icons::heart {}
                  }
              span { class: "mx-3",
                     "\u{2022}",
              }
              a { class: "hover:underline",
                  href: "#",
                  "Meta"
              }
        }
    ))
}
