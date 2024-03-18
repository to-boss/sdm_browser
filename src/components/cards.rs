use dioxus::prelude::*;

use crate::{
    smartdata::models::{data_model_yaml, DataModelRepo},
    DataModelData,
};

const NAME_STYLE: &str = "p-1 m-1 text-sm text-slate-500 text-ellipsis rounded-md
    hover:cursor-pointer";

#[component]
pub fn RepoCard(
    data_model_repo: DataModelRepo,
    filter: String,
    collapsed: bool,
    data_model_data: Signal<DataModelData>,
) -> Element {
    let mut collapsed = use_signal(|| if !filter.is_empty() { false } else { collapsed });
    let mut selected_model = use_signal(|| "".to_string());

    let item_len = data_model_repo.data_models.len();
    let collapse_icon = if collapsed() { "▲" } else { "▼" };

    let mut update_data_model_data = {
        move |repo_name: &str, name: &str| {
            *selected_model.write() = name.to_owned();
            let url = data_model_yaml(repo_name, name);
            data_model_data.set(DataModelData {
                repo_name: repo_name.to_owned(),
                name: name.to_owned(),
                url,
            });
        }
    };

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
                            move |_| update_data_model_data(&repo_name, &name)
                        },
                        {color_name_based_on_filter(&name,  &filter, selected_model)}
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

fn color_name_based_on_filter(name: &str, filter: &str, selected_model: Signal<String>) -> Element {
    // TODO(bug): This only changes the color if the model is under the same repo
    // TODO(bug): after resetting the filter the highlight fades
    let highlight_bg = if selected_model() == name {
        "bg-green-100 hover:bg-green-200"
    } else {
        "hover:bg-gray-100"
    };

    if filter.is_empty() {
        return rsx! {
            li {
                class: "{NAME_STYLE} {highlight_bg}",
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
            class: "{NAME_STYLE} {highlight_bg}",
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
