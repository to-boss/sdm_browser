use dioxus::prelude::*;

use crate::smartdata::models::{DataModelRepo, GITHUB_MODEL_YAML};

/*
#[derive(Props)]
pub struct RepoCardProps {
    data_model_repo: &'a DataModelRepo,
    filter: String,
    #[props(default = false)]
    collapsed: bool,
}
*/

#[component]
pub fn RepoCard(
    data_model_repo: DataModelRepo,
    filter: String,
    collapsed: bool,
    data_model_url: Signal<String>,
) -> Element {
    let mut collapsed = use_signal(|| if !filter.is_empty() { false } else { collapsed });
    // let mut data_model_url = consume_context::<Signal<String>>();

    let dmr_len = data_model_repo.data_models.len();
    let collapse_icon = if collapsed() { "▲" } else { "▼" };

    rsx! {
        ul {
            div {
                class: "flex flex-row hover:cursor-pointer",
                onclick: move |_| collapsed.set(!collapsed()),
                div {
                    class: "flex flex-row",
                    span {
                        class: "text-slate-300 my-auto font-light text-sm",
                        "({dmr_len})",
                    },
                    h1 {
                        class: "ml-2 text-ellipsis text-base font-medium tracking-tight text-slate-900",
                        "{data_model_repo.name}",
                    },
                }
                div {
                    class: "text-slate-500 ml-auto border-1 size-4",
                    "{collapse_icon}",
                }
            }
            if !collapsed() {
                ul {
                    class: "ml-8",
                    for name in data_model_repo.data_models.iter() {
                        div {
                            onclick: {
                                let repo_name = data_model_repo.name.clone();
                                let name = name.clone();
                                move |_| {
                                    let url = GITHUB_MODEL_YAML.to_data_model_repo(repo_name.as_str(), name.as_str());
                                    data_model_url.set(url);
                                }
                            },
                            { color_name_based_on_filter(name, &filter) }
                        }
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

const NAME_STYLE: &str = "text-slate-500 hover:bg-gray-400 hover:cursor-pointer";
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
        if s.contains(filter) {
            let (a, b) = s.split_once(filter).unwrap();
            if a.is_empty() && b.is_empty() {
                vec.push(FilterSplit::new(filter.as_str(), true));
                return vec;
            }

            if a.is_empty() {
                vec.push(FilterSplit::new(filter.as_str(), true));
                vec.push(FilterSplit::new(b, false));
                vec
            } else {
                vec.push(FilterSplit::new(a, false));
                vec.push(FilterSplit::new(filter.as_str(), true));
                vec
            }
        } else {
            vec.push(FilterSplit::new(s, false));
            vec
        }
    });

    rsx! {
        li {
            class: NAME_STYLE,
            for split in final_splits.iter() {
                if split.equals_filter {
                    mark {
                        class: "font-semibold bg-yellow-300",
                        "{split.s}"
                    }
                } else {
                    "{split.s}"
                }
            }
        }
    }
}
