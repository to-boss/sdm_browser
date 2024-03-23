use dioxus::prelude::*;

use crate::{
    smartdata::models::{data_model_yaml, DataModelRepo},
    ModelData,
};

#[component]
pub fn RepoCard(
    data_model_repo: DataModelRepo,
    filter: String,
    model_data: Signal<Option<ModelData>>,
    collapsed: bool,
) -> Element {
    let mut collapsed = use_signal(|| if !filter.is_empty() { false } else { collapsed });

    let item_len = data_model_repo.data_models.len();
    let collapse_icon = if collapsed() { "▲" } else { "▼" };

    let mut update_model_data = {
        move |repo: String, name: String| {
            let url = data_model_yaml(&repo, &name);
            model_data.set(Some(ModelData { repo, name, url }));
        }
    };

    let is_selected_name = |name: &str| {
        if let Some(model_data) = model_data.read().as_ref() {
            model_data.name.as_str() == name
        } else {
            false
        }
    };

    let rendered_list = rsx!(ul {
        for name in data_model_repo.data_models.iter() {
            div {
                onclick: {
                    let repo = data_model_repo.name.clone();
                    let name = name.clone();
                    move |_| update_model_data(repo.clone(), name.clone())
                },
                FilteredName {
                    name,
                    filter: filter.clone(),
                    is_selected: is_selected_name(name),
               }
            }
        }
    });

    rsx!(div {
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
        {rendered_list}
    })
}

#[component]
fn FilteredName(name: String, filter: String, is_selected: bool) -> Element {
    const NAME_STYLE: &str = "p-1 m-1 text-sm text-slate-500 text-ellipsis rounded-md
        hover:cursor-pointer";

    let highlight_bg = if is_selected {
        "bg-green-100 hover:bg-green-200"
    } else {
        "hover:bg-gray-100"
    };

    if filter.is_empty() {
        return rsx!(li {
            class: "{NAME_STYLE} {highlight_bg}",
            "{name}"
        });
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

    let rendered_splits = splits.iter().map(|(text, equals)| {
        rsx!(if *equals {
                mark {
                    class: "font-semibold bg-rose-200",
                    "{text}"
                }
            } else {
                "{text}"
            }
        )
    });

    rsx!(li {
        class: "{NAME_STYLE} {highlight_bg}",
        {rendered_splits}
    })
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {}
}
