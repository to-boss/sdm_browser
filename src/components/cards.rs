use dioxus::prelude::*;

use crate::smartdata::models::DataModelRepo;

#[derive(PartialEq, Props, Clone)]
pub struct RepoCardProps {
    data_model_repo: DataModelRepo,
    filter: String,
    #[props(default = false)]
    collapsed: bool,
}

pub fn RepoCard(props: RepoCardProps) -> Element {
    let mut collapsed = use_signal(|| false);
    let dmr_len = props.data_model_repo.data_models.len();

    rsx! {
        ul {
            div {
                class: "flex flex-row hover:cursor-pointer",
                onclick: move |_| collapsed.set(!collapsed()),
                div {
                    class: "flex flex-row",
                    span {
                        class: "text-gray-300 my-auto font-light text-sm",
                        "({dmr_len})",
                    },
                    h1 {
                        class: "ml-2 text-lg text-white",
                        "{props.data_model_repo.name}",
                    },
                }
                div {
                    class: "text-white ml-auto border-1 size-4",
                    if collapsed() {
                        "▼"
                    } else {
                        "▲"
                    }
                }
            }
            if !collapsed() {
                ul {
                    class: "ml-8 text-gray-300",
                    for data_model in props.data_model_repo.data_models.iter() {
                        { color_name_based_on_filter(data_model, &props.filter) }
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

fn color_name_based_on_filter(name: &String, filter: &String) -> Element {
    if filter.is_empty() {
        return rsx! {
            li {
                class: "hover:bg-slate-200 hover:cursor-pointer",
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

    // Maybe this is too complicated? Maybe find a way to make it simpler
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
            class: "hover:bg-slate-200 hover:cursor-pointer",
            for split in final_splits.iter() {
                if split.equals_filter {
                    mark {
                        class: "font-semibold",
                        "{split.s}"
                    }
                } else {
                    "{split.s}"
                }
            }
        }
    }
}
