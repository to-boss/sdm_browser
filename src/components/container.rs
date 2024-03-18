use dioxus::prelude::*;

#[component]
pub fn Container(children: Element) -> Element {
    rsx! {
        div {
            class: "size-full flex flex-col relative border rounded-lg gap-2 m-2 p-2",
            {children}
        }
    }
}
