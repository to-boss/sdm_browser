use dioxus::prelude::*;

use crate::{
    cache::ModelCache,
    components::container::Container,
    smartdata::models::{GeoProperty, ParsedModel, Property},
};

#[component]
pub fn Model(model: ParsedModel) -> Element {
    let name = model.name.clone();
    let url = model.url.clone();
    let description = model.description.clone();

    rsx! {
        Container {
            // Title
            div {
                class: "w-60 flex flex-col gap-2",
                div {
                    class: "flex flex-row",
                    h1 {
                        class: "font-bold text-slate-950 text-lg",
                        "{name}"
                    },
                    a {
                        class: "my-auto ml-auto text-xs text-blue-400 hover:underline",
                        href: url,
                        "(link)"
                    }
                }
                p {
                    class: "text-xs text-slate-500",
                    "{description}"
                },
            }
            hr {},
            // Properties
            if !name.is_empty() {
                Properties { selected_model: name}
            } else {
                p { "Loading..." }
            }
        }
    }
}

#[component]
fn Properties(selected_model: String) -> Element {
    let mut cache = consume_context::<Signal<ModelCache>>();

    let rendered_model = match cache.read().get(&selected_model) {
        Some(model) => rsx!(
            h1 {
                class: "",
                "Properties"
            },
            for (i, prop) in model.properties.iter().enumerate() {
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
                            checked: prop.checked,
                            onchange: {
                                let selected_model = selected_model.clone();
                                move |_| cache.write().flip_checked(&selected_model, i)
                            },
                        },
                    }
                }
            }
        ),
        None => rsx!("Loading..."),
    };

    rsx!({ rendered_model })
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
