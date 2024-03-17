use dioxus::prelude::*;

use crate::smartdata::models::Model;

#[component]
pub fn ModelComponent(model: Model) -> Element {
    let properties = model.clone().into_sorted_properties();

    rsx! {
        p { "{model.description}" },
        for prop in properties.iter() {
            p { "{prop.name}" }
        }
    }
}
