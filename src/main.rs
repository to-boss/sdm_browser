#![allow(non_snake_case)]

use dioxus::{desktop::Config, prelude::*};

use crate::{
    cache::ModelCache,
    components::{list::FilteredList, model::Model},
    smartdata::models::ModelList,
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
pub struct ModelData {
    repo: String,
    name: String,
    url: String,
}

impl ModelData {
    pub fn name(&self) -> Option<&str> {
        if self.name.is_empty() {
            return None;
        }
        Some(&self.name)
    }
}

#[component]
fn ShowError(error: String) -> Element {
    rsx!(p {
        class:"bg-red-400 p-4 m-4 text-xs",
        "{error}"
    })
}

fn App() -> Element {
    // SIGNALS
    let mut cache = use_context_provider(|| Signal::new(ModelCache::new()));
    let model_data = use_signal(|| None);

    // RESOURCES and RENDERED RESOURCE
    let model_list = use_resource(|| async move { ModelList::fetch().await });
    let rendered_model_list = match &*model_list.read() {
        Some(Ok(list)) => rsx!(FilteredList {
            list: list.to_owned(),
            model_data,
        }),
        Some(Err(err)) => rsx!(ShowError {
            error: err.to_string(),
        }),
        None => None,
    };

    let selected_model = use_resource(move || async move {
        if let Some(model_data) = model_data.read().as_ref() {
            return Some(cache.write().get_or_fetch_and_insert(model_data).await);
        }
        None
    });
    let rendered_selected_model = match &*selected_model.read() {
        // The nesting is kinda ugly, but the logic in the resource is better this way
        Some(Some(Ok(model))) => rsx!(Model {
            model: model.clone()
        }),
        Some(Some(Err(err))) => rsx!(ShowError {
            error: err.to_string(),
        }),
        Some(None) => None,
        None => None,
    };

    rsx!(div {
        class: "flex flex-row gap-1",
        {rendered_model_list},
        {rendered_selected_model},
    })
}
