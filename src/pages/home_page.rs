use dioxus::prelude::*;

#[component]
pub fn HomePage() -> Element {
    let mut show_dialog = use_signal(|| false);
    let mut error_message = use_signal(|| String::new());
    let mut folders = use_signal(|| Vec::<String>::new());
    let mut new_folder_name = use_signal(|| String::new());

    let mut submit_folder = move |_| {
        let name = new_folder_name();
        let trimmed_name = name.trim();

        if !trimmed_name.is_empty() {
            error_message.set(String::new());
            folders.with_mut(|f| f.push(trimmed_name.to_string()));
            new_folder_name.set(String::new());
            show_dialog.set(false);
        } else {
            error_message.set(String::from("You need to enter a valid value"));
        }
    };

    rsx! {
        div { class: "flex h-screen w-full bg-[var(--background)]",
            // Side panel
            div { class: "w-64 border-r border-[var(--outline-variant)] bg-[var(--surface-container-low)] flex flex-col",
                // Panel header
                div { class: "p-4 border-b border-[var(--outline-variant)] flex justify-between items-center",
                    h2 { class: "text-lg font-medium text-[var(--on-surface)]", "Folders" }
                    button {
                        class: "text-[var(--secondary)] hover:text-[var(--secondary)]",
                        onclick: move |_| show_dialog.set(true),
                        "+ New"
                    }
                }

                // Folder list
                div { class: "flex-1 overflow-y-auto p-2",
                    for folder in folders.read().iter() {
                        div { class: "group flex items-center py-2 px-3 rounded-lg bg-[var(--tertiary)] hover:bg-[var(--secondary)]",
                            button { class: "text-[var(--on-tertiary)] hover:text-[var(--on-secondary)] mr-2",
                                ">"
                            }
                            span { class: "text-[var(--on-tertiary)]", {folder.clone()} }
                        }
                    }
                }
            }

            // Main content area
            div { class: "flex-1 p-8",
                h1 { class: "text-2xl font-bold text-[var(--on-surface)] mb-4", "Code Notes" }
                p { class: "text-[var(--on-surface-variant)]",
                    "Select a folder from the sidebar to view or add notes"
                }
            }

            // Dialog overlay
            if show_dialog() {
                div {
                    class: "fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50",
                    onclick: move |_| show_dialog.set(false),

                    // Dialog container
                    div {
                        class: "bg-[var(--surface)] rounded-xl p-6 shadow-lg w-96",
                        onclick: move |e| e.stop_propagation(),

                        h2 { class: "text-xl font-semibold mb-4 text-[var(--on-surface)]",
                            "Create New Folder"
                        }

                        input {
                            r#type: "text",
                            class: "border border-[var(--outline-variant)] rounded-lg p-3 w-full mb-4 bg-[var(--surface-container-low)] text-[var(--on-surface)]",
                            placeholder: "Folder name...",
                            value: "{new_folder_name}",
                            oninput: move |e| new_folder_name.set(e.value().clone()),
                            onkeydown: move |e| {
                                if e.key() == Key::Enter {
                                    submit_folder(())
                                }
                            },
                        }
                        if !error_message().is_empty() {
                            div { class: "mb-4 p-2 rounded-lg bg-[var(--error)] text-[var(--on-error)]",
                                {error_message()}
                            }
                        }

                        div { class: "flex justify-end gap-2",
                            button {
                                class: "px-4 py-2 rounded-lg text-[var(--on-surface-variant)] hover:bg-[var(--surface-container-high)]",
                                onclick: move |_| show_dialog.set(false),
                                "Cancel"
                            }
                            button {
                                class: "px-4 py-2 rounded-lg bg-[var(--primary)] text-[var(--on-primary)] hover:bg-[var(--primary-container)]",
                                onclick: move |_| submit_folder(()),
                                "Create"
                            }
                        }
                    }
                }
            }
        }
    }
}
