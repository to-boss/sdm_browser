#![allow(non_snake_case)]

use std::collections::HashMap;

use dioxus::{desktop::Config, prelude::*};

use crate::{
    components::{container::Container, list::FilteredList, model::ModelComponent},
    smartdata::models::{Model, ModelList},
};

mod codegen;
mod components;
mod smartdata;

const TAILWIND_LINK: &str = r#"<link rel="stylesheet" href="public/tailwind.css">"#;

fn main() {
    let config = Config::new().with_custom_head(TAILWIND_LINK.to_string());

    LaunchBuilder::desktop().with_cfg(config).launch(App);
}

#[derive(Default, Debug, Clone)]
pub struct DataModelData {
    repo_name: String,
    name: String,
    url: String,
}

fn App() -> Element {
    let current_model_data = use_signal(DataModelData::default);
    let mut cache = use_signal(HashMap::<String, Model>::new);

    let model_list = use_resource(move || async move { ModelList::fetch().await });

    // Could you make this cache use cleaner?
    let current_model = use_resource(move || async move {
        let dmd_ref = current_model_data.read();
        let cache_ref = cache.read();

        if let Some(model) = cache_ref.get(&dmd_ref.name) {
            return Ok(model.clone());
        }
        drop(cache_ref);

        let model = Model::fetch(&dmd_ref).await;
        if let Ok(model) = &model {
            cache.with_mut(|c| c.insert(dmd_ref.name.clone(), model.clone()));
        }
        model
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
                            current_model_data,
                            cache,
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
            // Middle
            {
                if let Some(Ok(model)) =  &*current_model.read() {
                    rsx! { ModelComponent {
                            model: model.clone(),
                            name: current_model_data.read().name.clone(),
                        }
                    }
                } else {
                    rsx! { Container { "No model selected." } }
                }
            }
            // Right side
            Container {
                "CODEGEN",
            }
        }
    }
}
