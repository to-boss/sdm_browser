use dioxus::prelude::*;

use crate::{
    components::{cards::RepoCard, container::Container},
    smartdata::models::ModelList,
    DataModelData,
};

#[component]
pub fn FilteredList(model_list: ModelList, data_model_data: Signal<DataModelData>) -> Element {
    let mut filter = use_signal(|| String::from(""));

    let filtered_entries = model_list.get_filtered_entries(&filter());
    let filtered_entries_rendered = filtered_entries.iter().map(|entry| {
        rsx!(RepoCard {
            data_model_repo: entry.clone(),
            filter,
            data_model_data,
            collapsed: false,
        })
    });

    rsx!(
        Container {
            h1 {
                class: "font-bold text-slate-950 text-lg",
                "Model Selection"
            },
            // FilterInput and List
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
            }
            // List
            div {
                class: "w-full divide-y border rounded-lg shadow-sm px-3 py-1 mb-1
                        overflow-x-hidden overflow-y-scroll",
                if filtered_entries.is_empty() {
                    p {
                        class: "p-1 m-1 text-sm text-slate-500",
                        "No entries matching."
                    }
                } else {
                    {filtered_entries_rendered}
                }
            },
        }
    )
}
