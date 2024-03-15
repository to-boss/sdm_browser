use dioxus::prelude::*;

use crate::{
    components::cards::RepoCard,
    smartdata::models::{DataModelRepo, ModelList},
};

#[derive(Debug, PartialEq, Props, Clone)]
pub struct FilteredListProps {
    model_list: ModelList,
}

pub fn FilteredList(props: FilteredListProps) -> Element {
    let mut filter = use_signal(|| String::from(""));

    rsx! {
        // Filter
        div {
            class: "size-full",
            input {
                class: "p-2",
                value: "{filter}",
                placeholder: "Search",
                spellcheck: false,
                oninput: move |event| filter.set(event.value())
            },
            // List
            div {
                class: "size-full overflow-auto divide-y-2",
                for data_model_repo in props.model_list.entries
                    .iter()
                    .filter(|&dmr| contains_filter(dmr, &filter())) {
                    RepoCard {
                        data_model_repo: data_model_repo.clone(),
                        filter: filter()
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
