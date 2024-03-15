#![allow(non_snake_case)]
use dioxus::{desktop::Config, prelude::*};

const TAILWIND_LINK: &'static str = r#"<link rel="stylesheet" href="public/tailwind.css">"#;

fn main() {
    let config = Config::new().with_custom_head(TAILWIND_LINK.to_string());

    LaunchBuilder::desktop().with_cfg(config).launch(App);
}

fn App() -> Element {
    rsx! {
        div {
            class: "text-red-400 body-font",
            "Hello, world!"
         }
    }
}
