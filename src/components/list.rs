use dioxus::prelude::*;

use crate::{
    components::cards::RepoCard,
    smartdata::models::{DataModelRepo, ModelList},
};

#[component]
pub fn FilteredList(model_list: ModelList, data_model_url: Signal<String>) -> Element {
    let mut filter = use_signal(|| String::from(""));

    rsx! {
        // Filter
        div {
            class: "size-full",
            input {
                class: "w-full text-slate-500",
                value: "{filter}",
                placeholder: "Search",
                spellcheck: false,
                oninput: move |event| filter.set(event.value())
            },
            // List
            div {
                class: "size-full overflow-auto scrollbar divide-y-2",
                for data_model_repo in model_list.entries
                    .iter()
                    .filter(|&dmr| contains_filter(dmr, &filter())) {
                    RepoCard {
                        data_model_repo: data_model_repo.clone(),
                        filter: filter,
                        data_model_url: data_model_url,
                        collapsed: false,
                    }
                }
            }
        }
    }
}

fn contains_filter(data_model_repo: &DataModelRepo, pattern: &String) -> bool {
    let in_names = data_model_repo.name.contains(pattern);
    let in_children = data_model_repo
        .data_models
        .iter()
        .any(|n| n.contains(pattern));

    in_names || in_children
}
