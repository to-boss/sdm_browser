use dioxus::prelude::*;

use crate::smartdata::models::{Model, Property};

#[component]
pub fn ModelComponent(model: Model, name: String) -> Element {
    let properties = model.clone().into_sorted_properties();

    rsx! {
        div {
            class: "size-full flex flex-col border rounded-lg gap-2 m-2 p-2",
            h1 {
                class: "font-bold text-slate-950 text-lg",
                "{name}"
            },
            p {
                class: "text-xs text-slate-500",
                "{model.description}"
            },
            hr {},
            Properties { properties },
        }
    }
}

#[component]
fn Properties(properties: Vec<Property>) -> Element {
    let mut marks: Signal<Vec<bool>> = use_signal(|| properties.iter().map(|p| p.marked).collect());
    let mut update_marks = move |event: Event<FormData>, i: usize| {
        marks.with_mut(|vec| {
            if let Some(element) = vec.get_mut(i) {
                *element = match event.data.value().as_str() {
                    "true" => true,
                    _ => false,
                };
            }
        })
    };

    rsx! {
        h1 {
            class: "",
            "Properties"
        },
        for (i, prop) in properties.iter().enumerate() {
            div {
                class: "flex flex-row",
                label {
                    class: "text-sm text-slate-500",
                    "{prop.name}"
                },
                div {
                    class: "ml-auto flex flex-row gap-2",
                    if prop.required {
                        span {
                            class: "mr-4 text-xs text-red-400",
                            "(required)"
                        }
                    },
                    input {
                        class: "",
                        r#type: "checkbox",
                        onchange: move |event| update_marks(event, i),
                    },
                }
            }
        }
    }
}
