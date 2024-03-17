use dioxus::prelude::*;

use crate::smartdata::models::Model;

#[component]
pub fn ModelComponent(model: Model, name: String) -> Element {
    let properties = model.clone().into_sorted_properties();

    rsx! {
        div {
            class: "size-full flex flex-col border rounded-lg gap-2 m-2 p-2",
            h1 { "{name}" },
            p { "{model.description}" },
            for prop in properties.iter() {
                p { "{prop.name}" }
            }
        }
    }
}
