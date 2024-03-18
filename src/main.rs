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

#[derive(Default, Clone)]
pub struct DataModelData {
    repo_name: String,
    name: String,
    url: String,
}

fn App() -> Element {
    let data_model_data = use_signal(DataModelData::default);

    let model_list = use_resource(move || async move { ModelList::fetch().await });
    let model = use_resource(move || async move {
        let data_model_data = data_model_data.read().clone();
        Model::fetch(data_model_data).await
    });

    rsx! {
        // Main Container
        div {
            class: "bg-white size-full flex flex-row gap-2 overflow-hidden",
            // Left Side
            div {
                class: "h-screen w-96 shrink-0",
                match &*model_list.read() {
                    Some(Ok(model_list)) => rsx! {
                        FilteredList{
                            model_list: model_list.clone(),
                            data_model_data,
                        }
                    },
                    Some(Err(err)) => rsx! { p {
                        "Error: {err}"
                    }},
                    None => rsx! { p {
                        "Loading model list..."
                    }},
                }
            },
            // Right side
            div {
                class: "flex-grow size-full",
                {if let Some(Ok(model)) =  model.read().as_ref() {
                    rsx! { ModelComponent {
                        model: model.clone(),
                        name: data_model_data.read().name.clone(),
                    }}
                } else {
                    rsx! { p {"No model selected."}}
                }}
            }
        }
    }
}
