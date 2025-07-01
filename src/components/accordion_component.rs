use dioxus::prelude::*;
use crate::helpers::AccordionProps;

#[component]
pub fn AccordionComponent(props: AccordionProps) -> Element {
    let mut is_open = use_signal(|| false);

    rsx! {
        div { class: "w-full border border-[var(--outline-variant)] rounded-lg overflow-hidden transition-all my-2",
            // Header
            button {
                class: "cursor-pointer w-full flex items-center justify-between p-4 bg-[var(--surface-container-lowest)] hover:bg-[var(--surface-container-low)] text-left transition-colors",
                onclick: move |_| is_open.set(!is_open()),
                h3 { class: "text-lg font-medium text-[var(--on-surface)]",
                    "{props.accordion_title.clone().unwrap_or_default()}"
                }
                svg {
                    class: "w-5 h-5 text-[var(--on-surface-variant)] transform transition-transform duration-200",
                    class: if is_open() { "rotate-180" } else { "" },
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    view_box: "0 0 24 24",
                    xmlns: "http://www.w3.org/2000/svg",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        d: "M19 9l-7 7-7-7",
                    }
                }
            }

            // Content
            div {
                class: "px-4 overflow-hidden transition-all duration-200 ease-in-out",
                class: if is_open() { "max-h-96 py-4 opacity-100" } else { "max-h-0 py-0 opacity-0" },
                div { class: "pb-2 text-[var(--on-surface-variant)] text-sm",
                    {props.accordion_description.clone()}
                }
            }
        }
    }
}