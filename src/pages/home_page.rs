use crate::syntax::markdown_to_html;
use ammonia::Builder;
use dioxus::prelude::*;
use crate::components::ButtonComponent;

#[component]
pub fn HomePage() -> Element {
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
        .url_relative(ammonia::UrlRelative::PassThrough)
        .clean(&custom_html)
        .to_string();

    rsx! {
        div { class: "min-h-screen bg-[var(--surface-container-lowest)] text-[var(--on-surface)] flex flex-col",
            header { class: "w-full bg-[var(--surface-container-high)] border-b border-[var(--outline-variant)] px-8 py-4",
                div { class: "max-w-7xl mx-auto flex justify-between items-center",
                    h1 { class: "text-2xl font-semibold text-primary", "Code Themes:" }
                    
                    div { class: "flex gap-2",
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

           main { class: "flex-1 p-8 w-full",
                div { class: "flex gap-6 h-full min-h-[calc(100vh-8rem)] max-w-7xl mx-auto",
                    div { class: "w-[50%] flex flex-col",
                        div { class: "flex items-center justify-between mb-2",
                            h2 { class: "text-lg font-medium text-[var(--on-surface-variant)]", "Editor" }
                            div { class: "text-xs text-[var(--on-surface-variant)]",
                                "{user_input_markdown().chars().count()} characters"
                            }
                        }
                        textarea {
                            class: "
                                flex-1 w-full p-6 rounded-xl border border-[var(--outline-variant)]
                                bg-[var(--surface-container-high)] text-[var(--on-surface)]
                                text-base font-mono resize-none outline-none
                                focus:ring-2 focus:ring-primary/50 focus:border-transparent
                                transition-all shadow-sm
                            ",
                            value: "{user_input_markdown()}",
                            oninput: move |e| user_input_markdown.set(e.value().clone()),
                            placeholder: "Write your markdown here...",
                        }
                    }

                   div { class: "w-[50%] flex flex-col",
                        div { class: "flex items-center justify-between mb-2",
                            h2 { class: "text-lg font-medium text-[var(--on-surface-variant)]", "Preview" }
                            div { class: "text-xs text-[var(--on-surface-variant)]",
                                "Live rendering"
                            }
                        }
                        div {
                            class: "
                                flex-1 w-full p-6 rounded-xl border border-[var(--outline-variant)]
                                bg-[var(--surface-container-high)] overflow-auto
                                prose prose-sm max-w-none
                                transition-all shadow-sm
                            ",
                            dangerous_inner_html: "{sanitized_html}",
                        }
                    }
                }
            }

            footer { class: "w-full bg-[var(--surface-container-high)] border-t border-[var(--outline-variant)] px-8 py-2",
                div { class: "max-w-7xl mx-auto flex justify-between items-center text-xs text-[var(--on-surface-variant)]",
                    div { "Codeor v1.0" }
                    div { class: "flex gap-4",
                        span { "Markdown supported" }
                        span { "Syntax highlighting" }
                        span { "Auto-save" }
                    }
                }
            }
        }
    }
}