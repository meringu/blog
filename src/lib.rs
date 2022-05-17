#![allow(non_snake_case)]

mod pages {
    mod blog;

    pub use blog::*;
}

use dioxus::prelude::*;
use dioxus_router::{Route, Router};
use pages::*;

pub fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        Router {
            Route { to: "/", Blog {} },
        }
    })
}
