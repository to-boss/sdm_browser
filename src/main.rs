#![allow(non_snake_case)]
use dioxus::{desktop::Config, prelude::*};

use crate::{components::list::FilteredList, smartdata::models::ModelList};

mod components;
mod smartdata;

const TAILWIND_LINK: &str = r#"<link rel="stylesheet" href="public/tailwind.css">"#;

fn main() {
    let config = Config::new().with_custom_head(TAILWIND_LINK.to_string());

    LaunchBuilder::desktop().with_cfg(config).launch(App);
}

fn App() -> Element {
    let model_list_future = use_resource(move || async move { ModelList::fetch().await });

    rsx! {
        div {
            class: "size-full flex flex-row overflow-hidden bg-gray-900",
            div {
                class: "mx-4 my-8 h-screen w-80",
                match &*model_list_future.read() {
                    Some(Ok(model_list)) => rsx! {
                        FilteredList{ model_list: model_list.clone() }
                    },
                    Some(Err(err)) => rsx! { div {
                        "{err}"
                    }},
                    None => rsx! { div {
                        "Loading models..."
                    }},
                }
            },
            div {
                class: "bg-green-200 size-full",
                "SEXO",
            }
        }
    }
}
