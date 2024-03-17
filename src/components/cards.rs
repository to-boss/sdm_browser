use dioxus::prelude::*;

use crate::{
    smartdata::models::{DataModelRepo, GITHUB_MODEL_YAML},
    DataModelData,
};

#[component]
pub fn RepoCard(
    data_model_repo: DataModelRepo,
    filter: String,
    collapsed: bool,
    data_model_data: Signal<DataModelData>,
) -> Element {
    let mut collapsed = use_signal(|| if !filter.is_empty() { false } else { collapsed });

    let item_len = data_model_repo.data_models.len();
    let collapse_icon = if collapsed() { "▲" } else { "▼" };

    rsx! {
        div {
            class: "w-full flex flex-row px-1 rounded-md",
            onclick: move |_| collapsed.set(!collapsed()),
            div {
                class: "my-2 w-full flex flex-row hover:cursor-pointer",
                h1 {
                    class: "w-64 text-ellipsis overflow-hidden text-base
                            font-medium tracking-tight text-slate-900",
                    "{data_model_repo.name}",
                },
                div {
                    class: "ml-auto my-auto flex flex-row gap-2",
                    span {
                        class: "text-xs text-slate-300 ",
                        "({item_len})",
                    },
                    a {
                        class: "text-xs text-blue-400 hover:underline",
                        href: data_model_repo.link,
                        "(link)"
                    },
                    div {
                        class: "text-xs text-slate-500",
                        "{collapse_icon}",
                    },
                },
            },
        },
        if !collapsed() {
            ul {
                class: "w-full",
                for name in data_model_repo.data_models.iter() {
                    div {
                        onclick: {
                            let repo_name = data_model_repo.name.clone();
                            let name = name.clone();
                            move |_| {
                                let url = GITHUB_MODEL_YAML.to_data_model_repo(&repo_name, &name);
                                data_model_data.set(DataModelData {
                                    name: name.clone(),
                                    url
                                });
                            }
                        },
                        div {
                            class: "",
                            { color_name_based_on_filter(name, &filter) }
                        },
                    }
                }
            }
        }
    }
}

struct FilterSplit<'a> {
    s: &'a str,
    equals_filter: bool,
}

impl<'a> FilterSplit<'a> {
    fn new(s: &'a str, equals_filter: bool) -> Self {
        FilterSplit { s, equals_filter }
    }
}

const NAME_STYLE: &str = "p-1 m-1 text-sm text-slate-500 text-ellipsis rounded-md
                          hover:bg-gray-200 hover:cursor-pointer";
fn color_name_based_on_filter(name: &String, filter: &String) -> Element {
    if filter.is_empty() {
        return rsx! {
            li {
                class: NAME_STYLE,
                "{name}"
            }
        };
    }

    if !name.contains(filter) {
        return rsx! { li {
            class: "hidden",
            "{name}"
        }};
    }

    // There is probably an easier way to do this...
    let splits = name.split_inclusive(filter);
    let final_splits: Vec<_> = splits.into_iter().fold(Vec::new(), |mut vec, s| {
        if let Some((a, b)) = s.split_once(filter) {
            if !a.is_empty() {
                vec.push(FilterSplit::new(a, false));
            }
            vec.push(FilterSplit::new(filter, true));
            if !b.is_empty() {
                vec.push(FilterSplit::new(b, false));
            }
        } else {
            vec.push(FilterSplit::new(s, false));
        }
        vec
    });

    rsx! {
        li {
            class: NAME_STYLE,
            for split in final_splits.iter() {
                if split.equals_filter {
                    mark {
                        class: "font-semibold bg-rose-200",
                        "{split.s}"
                    }
                } else {
                    "{split.s}"
                }
            }
        }
    }
}
