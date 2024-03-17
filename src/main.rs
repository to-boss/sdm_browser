#![allow(non_snake_case)]

use dioxus::{desktop::Config, prelude::*};

use crate::{
    components::{list::FilteredList, model::ModelComponent},
    smartdata::models::{Model, ModelList},
};

mod components;
mod smartdata;

const TAILWIND_LINK: &str = r#"<link rel="stylesheet" href="public/tailwind.css">"#;

fn main() {
    let config = Config::new().with_custom_head(TAILWIND_LINK.to_string());

    LaunchBuilder::desktop().with_cfg(config).launch(App);
}

fn App() -> Element {
    let data_model_url = use_signal(|| String::from(""));

    let model_list = use_resource(move || async move { ModelList::fetch().await });
    let model = use_resource(move || async move {
        let url = data_model_url.read().clone();
        Model::fetch(url.as_str()).await
    });

    rsx! {
        div {
            class: "bg-white size-full flex flex-row overflow-hidden",
            div {
                class: "h-screen w-96",
                match &*model_list.read() {
                    Some(Ok(model_list)) => rsx! {
                        FilteredList{
                            model_list: model_list.clone(),
                            data_model_url: data_model_url,
                        }
                    },
                    Some(Err(err)) => rsx! { p {
                        "{err}"
                    }},
                    None => rsx! { p {
                        "Loading model list..."
                    }},
                }
            },
            div {
                class: "size-full",
                {if let Some(Ok(model)) =  model.read().as_ref() {
                    rsx! { ModelComponent { model: model.clone() }}
                } else {
                    rsx! { p {"No model selected."}}
                }}
            }
        }
    }
}
