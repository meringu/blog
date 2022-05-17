use dioxus::prelude::*;

pub fn Home(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            class: "text-center pt-16 md:pt-32",
            h1 {
                class: "font-bold text-3xl md:text-5xl",
                "Welcome to my blog"
            }
        }
    ))
}
