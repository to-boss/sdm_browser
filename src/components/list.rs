use dioxus::prelude::*;

use crate::{
    components::cards::RepoCard,
    smartdata::models::{DataModelRepo, ModelList},
    DataModelData,
};

#[component]
pub fn FilteredList(model_list: ModelList, data_model_data: Signal<DataModelData>) -> Element {
    let mut filter = use_signal(|| String::from(""));

    rsx! {
        div {
            class: "size-full flex flex-col border rounded-lg gap-2 m-2 p-2",
            // Title
            h1 {
                class: "text-slate-950 text-lg",
                "Model Selection"
            },
            // FilterInput
            div {
                class: "relative",
                svg {
                    class: "absolute left-2 top-2.5 h-4 w-4 text-muted-foreground",
                    xmlns: "http://www.w3.org/2000/svg",
                    width: 24,
                    height: 24,
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: 2,
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    circle { cx: 11, cy: 11, r: 8 },
                    path { d: "m21 21-4.3-4.3" },
                },
                input {
                    class: "flex h-9 pl-8 w-full rounded-md border
                    border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors
                    placeholder:text-muted-foreground focus-visible:outline-none
                    focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed
                    disabled:opacity-50",
                    value: "{filter}",
                    placeholder: "Search",
                    spellcheck: false,
                    oninput: move |event| filter.set(event.value())
                },
            },
            // List
            div {
                class: "w-full divide-y border rounded-lg shadow-sm px-3 py-1 mb-1
                        overflow-x-hidden overflow-y-scroll",
                for data_model_repo in model_list.entries
                    .iter()
                    .filter(|&dmr| contains_filter(dmr, &filter())) {
                    RepoCard {
                        data_model_repo: data_model_repo.clone(),
                        filter,
                        data_model_data,
                        collapsed: false,
                    }
                }
            }
        }
    }
}

fn contains_filter(data_model_repo: &DataModelRepo, filter: &String) -> bool {
    let in_names = data_model_repo.name.contains(filter);
    // short curcuit
    if in_names {
        return true;
    }

    let in_children = data_model_repo
        .data_models
        .iter()
        .any(|n| n.contains(filter));

    in_children
}
