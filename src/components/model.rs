use dioxus::prelude::*;

use crate::{
    components::container::Container,
    smartdata::models::{GeoProperty, Model, Property},
};

#[component]
pub fn ModelComponent(model: Model, name: String) -> Element {
    let properties = model.clone().into_sorted_properties();

    rsx! {
        Container {
            // Title
            div {
                class: "w-60 flex flex-col gap-2",
                div {
                    class: "flex flex-row my-auto",
                    h1 {
                        class: "font-bold text-slate-950 text-lg",
                        "{name}"
                    },
                    a {
                        class: "ml-auto text-xs text-blue-400 hover:underline",
                        href: model.url,
                        "(link)"
                    }
                }
                p {
                    class: "text-xs text-slate-500",
                    "{model.description}"
                },
            }
            hr {},
            // Properties
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
                *element = matches!(event.data.value().as_str(), "true")
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
                class: "flex flex-row gap-2",
                label {
                    class: "text-sm text-slate-500",
                    "{prop.name}"
                },
                { prop.maybe_combobox() },
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

impl Property {
    pub fn maybe_combobox(&self) -> Option<Element> {
        // TODO: missing other one_of combobox options
        if self.one_of.is_some() && self.name == "location" {
            return Some(rsx! {
                select {
                    class: "border text-xs text-slate-500",
                    for geo_prop in GeoProperty::array() {
                        option {
                            value: "{geo_prop.str()}",
                             "{geo_prop.str()}"
                        }
                    }
                }
            });
        }
        None
    }
}
