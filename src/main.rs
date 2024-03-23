#![allow(non_snake_case)]

use dioxus::{desktop::Config, prelude::*};

use crate::{
    cache::ModelCache,
    components::{container::Container, list::FilteredList, model::ModelComponent},
    smartdata::models::{ModelList, ParsedModel},
};

mod cache;
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

impl DataModelData {
    pub fn maybe_current(&self) -> Option<&str> {
        if self.name.is_empty() {
            return None;
        }
        Some(&self.name)
    }
}

fn App() -> Element {
    let mut cache = use_context_provider(|| Signal::new(ModelCache::new()));
    let data_model_data = use_signal(DataModelData::default);

    let model_list = use_resource(|| async move { ModelList::fetch().await });
    let current_model =
        use_resource(
            move || async move { cache.write().get_or_fetch(&data_model_data.read()).await },
        );

    let rendered_model_component = match &*current_model.read() {
        Some(Ok(model)) => rsx!(ModelComponent {
            model: model.clone(),
            name: data_model_data.read().name.clone(),
        }),
        Some(Err(e)) => rsx!(p { class:"", "Error: {e}"}),
        _ => rsx!(p { class:"", "Loading..."}),
    };

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
            // Middle
            {rendered_model_component}
            // Right side
            Container {
                "CODEGEN",
            }
        }
    }
}
