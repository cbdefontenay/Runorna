use dioxus::prelude::*;

#[component]
pub fn ButtonComponent(theme: Signal<String>, name: String, text: String) -> Element {
    rsx! {
        button {
            class: "px-3 py-1 rounded-md border text-sm cursor-pointer",
            onclick: move |_| theme.set(name.clone()),
            {text}
        }
    }
}
