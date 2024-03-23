use dioxus::prelude::*;

use crate::{
    smartdata::models::{data_model_yaml, DataModelRepo},
    DataModelData,
};

#[component]
pub fn RepoCard(
    data_model_repo: DataModelRepo,
    filter: String,
    data_model_data: Signal<DataModelData>,
    collapsed: bool,
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
                        FilteredName {
                            name,
                            selected_model,
                            filter: filter.clone(),
                        }
                    }
                }
            }
        }
    }
}

const NAME_STYLE: &str = "p-1 m-1 text-sm text-slate-500 text-ellipsis rounded-md
    hover:cursor-pointer";

#[component]
fn FilteredName(name: String, filter: String, selected_model: String) -> Element {
    // TODO(bug): This only changes the color if the model is under the same repo
    // TODO(bug): after resetting the filter the highlight fades
    let highlight_bg = if selected_model == name {
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

    if !name.contains(&filter) {
        return None;
    }

    let splits: Vec<_> = name
        .split_inclusive(&filter)
        .flat_map(|split| match split.split_once(&filter) {
            Some((a, b)) => {
                let mut vec = vec![];
                if !a.is_empty() {
                    vec.push((a, false));
                }
                vec.push((&filter, true));
                if !b.is_empty() {
                    vec.push((b, false));
                }
                vec
            }
            _ => vec![(split, false)],
        })
        .collect();

    rsx! {
        li {
            class: "{NAME_STYLE} {highlight_bg}",
            for (text, equals) in splits.iter() {
                if *equals {
                    mark {
                        class: "font-semibold bg-rose-200",
                        "{text}"
                    }
                } else {
                    "{text}"
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {}
}
