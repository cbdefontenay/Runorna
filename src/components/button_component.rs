use dioxus::prelude::*;

#[component]
pub fn ButtonComponent(
    theme: Signal<String>,
    name: String,
    text: String,
    on_click: EventHandler<String>,
) -> Element {
    let is_active = theme() == name;

    rsx! {
        button {
            class: format!(
                "px-3 py-1 rounded-md border text-sm cursor-pointer {}",
                if is_active {
                    "bg-[var(--primary)] text-[var(--on-primary)]"
                } else {
                    "hover:bg-[var(--surface-container-high)]"
                }
            ),
            onclick: move |_| on_click.call(name.clone()),
            {text}
        }
    }
}