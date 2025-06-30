use crate::components::ButtonComponent;
use crate::data::{get_folder_name, get_notes, load_theme_preference, save_note, save_theme_preference, update_note, Note};
use crate::syntax::markdown_to_html;
use ammonia::{Builder, UrlRelative};
use dioxus::prelude::*;
use tokio::time::sleep;

#[component]
pub fn EditorPage(folder_id: i32) -> Element {
    let mut user_input_markdown = use_signal(|| String::new());
    let mut theme = use_signal(|| String::from("base16-eighties.dark"));
    let mut notes = use_signal(|| Vec::<Note>::new());
    let mut current_note_id = use_signal(|| None::<i32>);
    let mut folder_name = use_signal(|| String::new());
    let mut is_saved_note = use_signal(|| false);

    use_effect(move || {
        spawn(async move {
            if let Ok(saved_theme) = load_theme_preference().await {
                theme.set(saved_theme);
            }
        });
    });

    let handle_theme_change = move |new_theme: String| {
        theme.set(new_theme.clone());
        spawn(async move {
            let _ = save_theme_preference(new_theme).await;
        });
    };

    use_effect(move || {
        spawn(async move {
            user_input_markdown.set("".to_string());
            current_note_id.set(None);

            if let Ok(name) = get_folder_name(folder_id).await {
                folder_name.set(name);
            }

            if let Ok(loaded_notes) = get_notes(folder_id).await {
                notes.set(loaded_notes);
                if let Some(latest_note) = notes.read().first() {
                    user_input_markdown.set(latest_note.content.clone());
                    current_note_id.set(Some(latest_note.id));
                }
            }
        });
    });
    
    let save_note = move || {
        let content = user_input_markdown();
        let now = chrono::Local::now().to_rfc3339();

        spawn(async move {
            let result = if let Some(note_id) = current_note_id() {
                update_note(note_id, content.clone(), now).await
            } else {
                save_note(content.clone(), now, folder_id).await
            };

            match result {
                Ok(_) => {
                    is_saved_note.set(true);
                    spawn(async move {
                        sleep(std::time::Duration::from_secs(3)).await;
                        is_saved_note.set(false);
                    });

                    if let Ok(loaded_notes) = get_notes(folder_id).await {
                        notes.set(loaded_notes);
                        if let Some(latest_note) = notes.read().first() {
                            current_note_id.set(Some(latest_note.id));
                        }
                    }
                }
                Err(e) => {
                    log::error!("Failed to save note: {}", e);
                }
            }
        });
    };

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
                    h1 { class: "text-xl font-semibold text-[var(--primary)]",
                        "Editing: {folder_name()}"
                    }

                    div { class: "flex flex-wrap gap-2 justify-center",
                        ButtonComponent {
                            theme: theme.clone(),
                            name: "base16-ocean.dark".to_string(),
                            text: "Ocean".to_string(),
                            on_click: handle_theme_change,
                        }
                        ButtonComponent {
                            theme: theme.clone(),
                            name: "base16-eighties.dark".to_string(),
                            text: "Eighties".to_string(),
                            on_click: handle_theme_change,
                        }
                        ButtonComponent {
                            theme: theme.clone(),
                            name: "base16-mocha.dark".to_string(),
                            text: "Mocha dark".to_string(),
                            on_click: handle_theme_change,
                        }
                        ButtonComponent {
                            theme: theme.clone(),
                            name: "InspiredGitHub".to_string(),
                            text: "GitHub".to_string(),
                            on_click: handle_theme_change,
                        }
                        ButtonComponent {
                            theme: theme.clone(),
                            name: "base16-ocean.light".to_string(),
                            text: "Light".to_string(),
                            on_click: handle_theme_change,
                        }
                        ButtonComponent {
                            theme: theme.clone(),
                            name: "Solarized (dark)".to_string(),
                            text: "Solarized dark".to_string(),
                            on_click: handle_theme_change,
                        }
                        ButtonComponent {
                            theme: theme.clone(),
                            name: "Solarized (light)".to_string(),
                            text: "Solarized light".to_string(),
                            on_click: handle_theme_change,
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
                            div { class: "flex items-center gap-2",
                                div { class: "text-xs text-[var(--on-surface-variant)]",
                                    "{user_input_markdown().chars().count()} characters"
                                }
                                button {
                                    class: "cursor-pointer px-3 py-1 rounded-lg bg-[var(--primary)] text-[var(--on-primary)] hover:bg-[var(--tertiary)] hover:text-[var(--on-tertiary)] text-sm",
                                    onclick: move |_| save_note(),
                                    "Save Note"
                                }
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
        }

        if is_saved_note() {
            div { class: "fixed bottom-4 right-4 z-50",
                div { class: "
                        bg-[var(--inverse-surface)] text-[var(--inverse-on-surface)]
                        px-4 py-2 rounded-lg shadow-lg
                        flex items-center gap-2
                        animate-fade-in
                    ",
                    svg {
                        class: "w-5 h-5",
                        xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 24 24",
                        fill: "currentColor",
                        path { d: "M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41L9 16.17z" }
                    }
                    span { "Note saved successfully!" }
                }
            }
        }
    }
}