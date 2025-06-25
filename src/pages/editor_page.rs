use crate::syntax::markdown_to_html;
use crate::components::ButtonComponent;
use ammonia::{Builder, UrlRelative};
use dioxus::prelude::*;

#[component]
pub fn EditorPage() -> Element {
    let mut user_input_markdown = use_signal(|| "".to_string());
    let theme = use_signal(|| "base16-eighties.dark".to_string());

    let custom_html = markdown_to_html(&user_input_markdown(), &theme());

    let sanitized_html = Builder::default()
        .add_tags([
            "pre", "code", "span", "input", "label", "div", "section", "article", "table", "thead",
            "tbody", "tfoot", "tr", "th", "td", "del", "ins", "mark", "sup", "sub", "details",
            "summary", "math", "mrow", "mi", "mo", "mn", "msup", "msub", "msubsup", "mfrac",
        ])
        .add_tag_attributes("input", ["type", "checked", "disabled"].into_iter())
        .add_generic_attributes(["class", "style", "id", "aria-hidden", "data-*"].into_iter())
        .url_relative(UrlRelative::PassThrough)
        .clean(&custom_html)
        .to_string();

    rsx! {
        div { class: "min-h-screen bg-[var(--surface-container-lowest)] text-[var(--on-surface)] flex flex-col",
            header { class: "w-[90%] rounded-lg mt-5 mb-auto ml-auto mr-auto shadow-md bg-[var(--surface-container-high)] border-b border-[var(--outline-variant)] px-4 sm:px-8 py-4 sticky top-0 z-10",
                div { class: "max-w-7xl mx-auto flex flex-col sm:flex-row justify-between items-center gap-4",
                    h1 { class: "text-xl sm:text-2xl font-semibold text-[var(--primary)]",
                        "Code Themes:"
                    }

                    div { class: "flex flex-wrap gap-2 justify-center",
                        ButtonComponent {
                            theme: theme.clone(),
                            name: "base16-ocean.dark".to_string(),
                            text: "Ocean".to_string(),
                        }
                        ButtonComponent {
                            theme: theme.clone(),
                            name: "base16-eighties.dark".to_string(),
                            text: "Eighties".to_string(),
                        }
                        ButtonComponent {
                            theme: theme.clone(),
                            name: "base16-mocha.dark".to_string(),
                            text: "Mocha dark".to_string(),
                        }
                        ButtonComponent {
                            theme: theme.clone(),
                            name: "InspiredGitHub".to_string(),
                            text: "GitHub".to_string(),
                        }
                        ButtonComponent {
                            theme: theme.clone(),
                            name: "base16-ocean.light".to_string(),
                            text: "Light".to_string(),
                        }
                        ButtonComponent {
                            theme: theme.clone(),
                            name: "Solarized (dark)".to_string(),
                            text: "Solarized dark".to_string(),
                        }
                        ButtonComponent {
                            theme: theme.clone(),
                            name: "Solarized (light)".to_string(),
                            text: "Solarized light".to_string(),
                        }
                    }
                }
            }

            main { class: "flex-1 p-4 sm:p-8 w-full",
                div { class: "grid grid-cols-1 lg:grid-cols-2 gap-6 h-full max-w-7xl mx-auto min-h-[calc(100vh-8rem)]",
                    // Editor Panel
                    div { class: "flex flex-col h-full",
                        div { class: "flex items-center justify-between mb-2",
                            h2 { class: "text-lg font-medium text-[var(--on-surface-variant)]",
                                "Editor"
                            }
                            div { class: "text-xs text-[var(--on-surface-variant)]",
                                "{user_input_markdown().chars().count()} characters"
                            }
                        }
                        div { class: "flex-1 flex flex-col border border-[var(--outline-variant)] rounded-xl overflow-hidden",
                            textarea {
                                class: "
                                    flex-1 w-full p-4 sm:p-6
                                    bg-[var(--surface-container-high)] text-[var(--on-surface)]
                                    text-base font-mono resize-none outline-none
                                    [scrollbar-width:none] [-webkit-scrollbar:none]
                                ",
                                spellcheck: "false",
                                value: "{user_input_markdown()}",
                                oninput: move |e| user_input_markdown.set(e.value().clone()),
                                placeholder: "Write your markdown here...",
                            }
                        }
                    }

                    // Preview Panel
                    div { class: "flex flex-col h-full",
                        div { class: "flex items-center justify-between mb-2",
                            h2 { class: "text-lg font-medium text-[var(--on-surface-variant)]",
                                "Preview"
                            }
                            div { class: "text-xs text-[var(--on-surface-variant)]",
                                "Live rendering"
                            }
                        }
                        div { class: "flex-1 flex flex-col border border-[var(--outline-variant)] rounded-xl overflow-hidden",
                            div {
                                class: "
                                    flex-1 w-full p-4 sm:p-6 overflow-auto
                                    bg-[var(--surface-container-high)]
                                    prose prose-sm max-w-none
                                    [&_*]:text-[var(--on-surface)]
                                    [&_pre]:bg-[var(--surface-container-low)]
                                    [scrollbar-width:none] [-webkit-scrollbar:none]
                                ",
                                dangerous_inner_html: "{sanitized_html}",
                            }
                        }
                    }
                }
            }

            footer { class: "w-full bg-[var(--surface-container-high)] border-t border-[var(--outline-variant)] px-4 sm:px-8 py-2",
                div { class: "max-w-7xl mx-auto flex flex-col sm:flex-row justify-between items-center text-xs text-[var(--on-surface-variant)] gap-2",
                    div { "Codeor v1.0" }
                    div { class: "flex flex-wrap gap-2 sm:gap-4 justify-center",
                        span { "Markdown supported" }
                        span { "Syntax highlighting" }
                        span { "Auto-save" }
                    }
                }
            }
        }
    }
}
