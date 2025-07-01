use std::collections::HashSet;
use crate::data::Folder;
use dioxus::prelude::*;

#[component]
pub fn FolderItem(
    folder: Folder,
    show_dropdown_for_folder: Signal<Option<i32>>,
    on_delete: EventHandler<i32>,
    on_update_click: EventHandler<(i32, String)>,
    on_create_subfolder_click: EventHandler<i32>,
    on_select_subfolder: EventHandler<i32>,
    expanded_folders: Signal<HashSet<i32>>,
) -> Element {
    let folder_id = folder.id;
    let is_dropdown_open = show_dropdown_for_folder
        .read()
        .map_or(false, |id| id == folder_id);
    let is_parent_folder = folder.parent_id.is_none();
    let is_expanded = expanded_folders.read().contains(&folder_id);
    let has_children = !folder.children.is_empty();

    let toggle_expanded = move |evt: Event<MouseData>| {
        evt.stop_propagation();
        let mut expanded = expanded_folders.write();
        if expanded.contains(&folder_id) {
            expanded.remove(&folder_id);
        } else {
            expanded.insert(folder_id);
        }
    };

    let depth = if is_parent_folder { 0 } else { 1 };

    rsx! {
        div {
            class: "flex flex-col px-1",
            style: if depth == 0 { "border-bottom: 1px solid var(--primary);" } else { "" },

            div {
                class: "group flex items-center py-2 px-3 rounded-lg justify-between relative transition-colors duration-150",
                style: match depth {
                    0 => "background: var(--surface-container-low); margin-bottom: 4px;",
                    1 => "background: var(--surface-container-highest); margin: 7px 0;",
                    _ => "",
                },
                // Main content container
                div {
                    class: "flex items-center min-w-0 flex-1",
                    onclick: move |_| {
                        if depth > 0 {
                            on_select_subfolder.call(folder_id);
                        }
                    },

                    // Chevron icon for folders with children
                    if has_children {
                        button {
                            class: "cursor-pointer text-[var(--on-surface-variant)] hover:text-[var(--on-surface)] mr-2 transition-all duration-150 flex-shrink-0",
                            style: if !is_expanded { "transform: rotate(0deg);" } else { "transform: rotate(90deg);" },
                            onclick: toggle_expanded,
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                view_box: "0 0 20 20",
                                fill: "currentColor",
                                class: "w-4 h-4",
                                path {
                                    fill_rule: "evenodd",
                                    d: "M7.21 14.77a.75.75 0 01.02-1.06L11.168 10 7.23 6.29a.75.75 0 111.04-1.08l4.5 4.25a.75.75 0 010 1.08l-4.5 4.25a.75.75 0 01-1.06-.02z",
                                    clip_rule: "evenodd",
                                }
                            }
                        }
                    } else {
                        div { class: "w-6 h-4 flex-shrink-0" }
                    }

                    // Folder name
                    div { class: "min-w-0 overflow-hidden",
                        span {
                            class: if depth > 0 { "text-[var(--on-surface)] truncate hover:text-[var(--primary)] cursor-pointer block w-full" } else { "text-[var(--on-surface)] truncate block w-full" },
                            style: match depth {
                                0 => "font-weight: 500; font-size: 0.95rem;",
                                1 => "font-weight: 400; font-size: 0.9rem;",
                                _ => "",
                            },
                            "{folder.name}"
                        }
                    }
                }

                // Three dots menu
                div { class: "relative flex-shrink-0 ml-2 w-6",
                    button {
                        class: "cursor-pointer text-[var(--on-surface-variant)] hover:text-[var(--on-surface)] p-1 rounded-full hover:bg-[var(--surface-container-highest)] transition-colors duration-150",
                        onclick: move |evt| {
                            evt.stop_propagation();
                            show_dropdown_for_folder
                                .set(if is_dropdown_open { None } else { Some(folder_id) });
                        },
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            view_box: "0 0 24 24",
                            fill: "currentColor",
                            class: "w-5 h-5",
                            path {
                                fill_rule: "evenodd",
                                d: "M10.5 6a1.5 1.5 0 1 1 3 0 1.5 1.5 0 0 1 -3 0ZM10.5 12a1.5 1.5 0 1 1 3 0 1.5 1.5 0 0 1 -3 0ZM12 18a1.5 1.5 0 1 0 0-3 1.5 1.5 0 0 0 0 3Z",
                                clip_rule: "evenodd",
                            }
                        }
                    }

                    // Dropdown menu
                    if is_dropdown_open {
                        div {
                            class: "absolute right-0 mt-2 w-48 bg-[var(--surface-container-high)] rounded-md shadow-lg z-10 border border-[var(--outline-variant)] animate-fade-in",
                            onclick: move |evt| evt.stop_propagation(),

                            if is_parent_folder {
                                button {
                                    class: "cursor-pointer block w-full text-left px-4 py-2 text-sm text-[var(--on-surface)] hover:bg-[var(--surface-container-highest)] transition-colors duration-100",
                                    onclick: move |_| {
                                        on_create_subfolder_click.call(folder_id);
                                        show_dropdown_for_folder.set(None);
                                    },
                                    "Create Subfolder"
                                }
                            }
                            button {
                                class: "cursor-pointer block w-full text-left px-4 py-2 text-sm text-[var(--on-surface)] hover:bg-[var(--surface-container-highest)] transition-colors duration-100",
                                onclick: move |_| {
                                    on_update_click.call((folder_id, folder.name.clone()));
                                    show_dropdown_for_folder.set(None);
                                },
                                if is_parent_folder {
                                    "Rename Folder"
                                } else {
                                    "Rename Subfolder"
                                }
                            }

                            button {
                                class: "cursor-pointer block w-full text-left px-4 py-2 text-sm text-[var(--error)] hover:bg-[var(--error-container)] transition-colors duration-100",
                                onclick: move |_| {
                                    on_delete.call(folder_id);
                                    show_dropdown_for_folder.set(None);
                                },
                                if is_parent_folder {
                                    "Delete Folder"
                                } else {
                                    "Delete Subfolder"
                                }
                            }
                        }
                    }
                }
            }

            // Render subfolders recursively if expanded and has children
            if has_children && is_expanded {
                div {
                    class: "ml-6 pl-2 border-l-2 border-[var(--outline-variant)]",
                    style: "border-left-color: var(--primary-container); margin-bottom: 10px;",
                    for subfolder in folder.children.iter() {
                        FolderItem {
                            folder: subfolder.clone(),
                            show_dropdown_for_folder,
                            on_delete,
                            on_update_click,
                            on_create_subfolder_click,
                            on_select_subfolder: on_select_subfolder.clone(),
                            expanded_folders,
                        }
                    }
                }
            }
        }
    }
}