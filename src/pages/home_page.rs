use std::collections::HashSet;
use crate::components::FolderItem;
use crate::data::{
    delete_folder_recursive, get_folder_name, get_folders, get_notes, save_folder,
    update_folder_name, Folder,
};
use crate::helpers::DialogMode;
use crate::pages::EditorPage;
use chrono::Local;
use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn HomePage() -> Element {
    let mut show_dialog: Signal<bool> = use_signal(|| false);
    let mut error_message: Signal<String> = use_signal(|| String::new());
    let mut folders: Signal<Vec<Folder>> = use_signal(|| Vec::<Folder>::new());
    let mut new_folder_name: Signal<String> = use_signal(|| String::new());
    let mut loading_error: Signal<Option<String>> = use_signal(|| None);
    let mut is_loading: Signal<bool> = use_signal(|| true);
    let mut selected_subfolder: Signal<Option<i32>> = use_signal(|| None::<i32>);
    let expanded_folders = use_signal(|| HashSet::<i32>::new());

    let show_dropdown_for_folder = use_signal(|| Option::<i32>::None);
    let mut dialog_mode = use_signal(|| DialogMode::Create);
    let mut current_folder_id = use_signal(|| Option::<i32>::None);

    let handle_select_subfolder = move |folder_id: i32| {
        selected_subfolder.set(Some(folder_id));
        spawn(async move {
            let _ = get_folder_name(folder_id).await;
            let _ = get_notes(folder_id).await;
        });
    };

    let fetch_folders = move || {
        spawn(async move {
            is_loading.set(true);
            match get_folders().await {
                Ok(folders_from_db) => {
                    folders.set(folders_from_db);
                    is_loading.set(false);
                }
                Err(e) => {
                    loading_error.set(Some(e.to_string()));
                    is_loading.set(false);
                }
            }
        });
    };

    use_effect(move || {
        fetch_folders();
    });

    let mut submit_folder = move |_| {
        let name = new_folder_name();
        let trimmed_name = name.trim();

        if !trimmed_name.is_empty() {
            error_message.set(String::new());
            let now = Local::now().to_rfc3339();
            let parent_id_for_save = match dialog_mode() {
                DialogMode::CreateSubfolder => *current_folder_id.read(),
                _ => None,
            };

            spawn({
                let trimmed_name = trimmed_name.to_string();
                async move {
                    match dialog_mode() {
                        DialogMode::Create | DialogMode::CreateSubfolder => {
                            if let Err(e) =
                                save_folder(trimmed_name.clone(), now, parent_id_for_save).await
                            {
                                error_message.set(format!("Failed to save folder: {}", e));
                            } else {
                                new_folder_name.set(String::new());
                                show_dialog.set(false);
                                fetch_folders();
                            }
                        }
                        DialogMode::Update => {
                            if let Some(id) = *current_folder_id.read() {
                                if let Err(e) = update_folder_name(id, trimmed_name.clone()).await {
                                    error_message.set(format!("Failed to update folder: {}", e));
                                } else {
                                    new_folder_name.set(String::new());
                                    show_dialog.set(false);
                                    fetch_folders();
                                }
                            }
                        }
                    }
                }
            });
        } else {
            error_message.set(String::from("You need to enter a valid value"));
        }
    };

    let delete_folder = move |folder_id: i32| {
        spawn(async move {
            if let Err(e) = delete_folder_recursive(folder_id).await {
                log::error!("Failed to delete folder: {}", e);
            } else {
                if selected_subfolder() == Some(folder_id) {
                    selected_subfolder.set(None);
                }
                fetch_folders();
            }
        });
    };

    rsx! {
        div { class: "flex h-screen w-full bg-[var(--background)] overflow-hidden",
            // Side panel
            div { class: "w-72 border-r border-[var(--primary)] bg-[var(--surface-container)] flex flex-col h-screen",
                // Panel header
                div { class: "flex flex-row p-4 border-b border-[var(--primary)] flex-shrink-0",
                    h2 { class: "text-lg font-medium text-[var(--on-surface)]", "Folders" }
                    button {
                        class: "ml-10 md:ml-32 text-[var(--secondary)] hover:text-[var(--secondary)] cursor-pointer",
                        onclick: move |_| {
                            dialog_mode.set(DialogMode::Create);
                            new_folder_name.set(String::new());
                            error_message.set(String::new());
                            show_dialog.set(true);
                        },
                        "+ New"
                    }
                }

                // Folder list - this will scroll independently
                div { class: "flex-1 overflow-y-auto overflow-x-hidden",
                    if *is_loading.read() {
                        div { class: "text-[var(--on-surface-variant)] p-4", "Loading folders..." }
                    } else if let Some(err) = loading_error.read().as_ref() {
                        div { class: "p-4 text-[var(--error)]", "Error loading folders: {err}" }
                    } else if folders.read().is_empty() {
                        div { class: "text-[var(--on-surface-variant)] p-4",
                            "No folders yet. Click '+ New' to create one."
                        }
                    } else {
                        div { class: "divide-y divide-[var(--primary)]",
                            for folder in folders.read().iter() {
                                FolderItem {
                                    folder: folder.clone(),
                                    show_dropdown_for_folder,
                                    on_delete: delete_folder,
                                    on_update_click: move |(f_id, f_name)| {
                                        current_folder_id.set(Some(f_id));
                                        new_folder_name.set(f_name);
                                        dialog_mode.set(DialogMode::Update);
                                        error_message.set(String::new());
                                        show_dialog.set(true);
                                    },
                                    on_create_subfolder_click: move |f_id| {
                                        current_folder_id.set(Some(f_id));
                                        new_folder_name.set(String::new());
                                        dialog_mode.set(DialogMode::CreateSubfolder);
                                        error_message.set(String::new());
                                        show_dialog.set(true);
                                    },
                                    on_select_subfolder: handle_select_subfolder,
                                    expanded_folders,
                                }
                            }
                        }
                    }
                }
            }

            // Main content area
            div { class: "flex-1 overflow-hidden",
                match selected_subfolder() {
                    Some(folder_id) => rsx! {
                        div { key: "{folder_id}", class: "h-full overflow-auto",
                            EditorPage { folder_id }
                        }
                    },
                    None => rsx! {
                        div { class: "h-full flex flex-col items-center justify-center text-center p-8",
                            div { class: "max-w-md flex flex-col items-center gap-6",
                                // Icon or illustration (using a simple SVG for now)
                                svg {
                                    class: "w-24 h-24 text-[var(--primary)]",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "1.5",
                                    view_box: "0 0 24 24",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        d: "M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m2.25 0H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 00-9-9z",
                                    }
                                }
                                h1 { class: "text-3xl font-bold text-[var(--on-surface)]", "Welcome to Runorna" }
                                p { class: "text-lg text-[var(--on-surface-variant)] mb-6",
                                    "Select a folder from the sidebar to view or create notes"
                                }
                                div { class: "flex flex-col sm:flex-row gap-4",
                                    button {
                                        class: "cursor-pointer px-6 py-3 rounded-lg bg-[var(--primary)] text-[var(--on-primary)] hover:bg-[var(--primary-container)] hover:text-[var(--on-primary-container)] transition-colors",
                                        onclick: move |_| {
                                            dialog_mode.set(DialogMode::Create);
                                            new_folder_name.set(String::new());
                                            error_message.set(String::new());
                                            show_dialog.set(true);
                                        },
                                        "Create New Folder"
                                    }
                                    button { class: "cursor-pointer px-6 py-3 rounded-lg border border-[var(--outline-variant)] text-[var(--on-surface-variant)] hover:bg-[var(--surface-container-high)] transition-colors",
                                        Link { to: Route::Faq {}, "Learn more about Runorna" }
                                    }
                                }
                            }
                        }
                    },
                }
            }

            // Dialog overlay
            if show_dialog() {
                div {
                    class: "fixed inset-0 bg-black/50 backdrop-blur-lg flex items-center justify-center z-50",
                    onclick: move |_| show_dialog.set(false),

                    // Dialog container
                    div {
                        class: "bg-[var(--surface-container-low)] rounded-xl p-6 shadow-lg w-96",
                        onclick: move |e| e.stop_propagation(),

                        h2 { class: "text-xl font-semibold mb-4 text-[var(--on-surface)]",
                            {
                                match dialog_mode() {
                                    DialogMode::Create => "Create New Folder",
                                    DialogMode::Update => "Update Folder Name",
                                    DialogMode::CreateSubfolder => "Create New Subfolder",
                                }
                            }
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
                                {
                                    match dialog_mode() {
                                        DialogMode::Create => "Create",
                                        DialogMode::Update => "Update",
                                        DialogMode::CreateSubfolder => "Create Subfolder",
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}