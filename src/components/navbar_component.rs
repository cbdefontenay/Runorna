use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn NavbarComponent() -> Element {
    let nav_items: Vec<(String, Route)> = vec![
        ("Home".to_string(), Route::Home {}),
        ("Settings".to_string(), Route::SettingsPage {}),
    ];

    let mut mobile_menu_open = use_signal(|| false);

    rsx! {
        div {
            // Mobile menu button (hamburger)
            button {
                class: "
                    lg:hidden fixed top-4 left-4 z-50 p-3
                    bg-[var(--surface-container-high)] rounded-full shadow-lg
                    text-[var(--on-surface)] hover:bg-[var(--surface-container-highest)]
                    transition-all duration-200 hover:shadow-xl
                    focus:outline-none focus:ring-2 focus:ring-[var(--primary)]/50
                ",
                onclick: move |_| mobile_menu_open.set(!mobile_menu_open()),
                svg {
                    class: "w-6 h-6",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke: "currentColor",
                    stroke_width: "2",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        d: "M4 6h16M4 12h16M4 18h16",
                    }
                }
            }

            // Mobile overlay
            div {
                class: "
                    lg:hidden fixed inset-0 z-30 bg-black/50
                    transition-opacity duration-300 ease-in-out backdrop-blur-sm
                ",
                class: if *mobile_menu_open.read() { "opacity-100 pointer-events-auto" } else { "opacity-0 pointer-events-none" },
                onclick: move |_| mobile_menu_open.set(false),
            }

            // Sidebar nav
            nav {
                class: "
                    fixed top-0 left-0 z-40 w-72 h-screen
                    bg-[var(--surface-container-high)] backdrop-blur-lg
                    border-r border-[var(--outline-variant)/30]
                    text-[var(--on-surface)] transition-transform duration-300 ease-in-out
                    shadow-2xl rounded-tr-3xl rounded-br-3xl
                    lg:translate-x-0
                ",
                class: if *mobile_menu_open.read() { "translate-x-0" } else { "-translate-x-full lg:translate-x-0" },

                // Header with subtle gradient
                div { class: "
                        px-6 py-5 border-b border-[var(--outline-variant)/20]
                        text-xl font-semibold text-[var(--primary)]
                        bg-gradient-to-r from-[var(--primary-container)/10] to-transparent
                    ",
                    div { class: "flex items-center gap-3",
                        svg {
                            class: "w-6 h-6 text-[var(--primary)]",
                            fill: "none",
                            view_box: "0 0 24 24",
                            stroke: "currentColor",
                            stroke_width: "2",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                d: "M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4",
                            }
                        }
                        "Codeor"
                    }
                }

                // Nav Items with active state indicator
                ul { class: "flex flex-col gap-1 p-3 mt-2",
                    for (item , link) in nav_items.iter() {
                        li { class: "
                                relative px-4 py-3 mx-2 rounded-lg
                                hover:bg-[var(--primary-container)/10]
                                active:bg-[var(--primary-container)/20]
                                text-[var(--on-surface-variant)] hover:text-[var(--on-surface)]
                                transition-all duration-200
                                group
                            ",
                            Link {
                                class: "w-full block flex items-center gap-3",
                                to: link.clone(),
                                // Icon placeholder - you can replace with actual icons
                                svg {
                                    class: "
                                        w-5 h-5 text-[var(--primary)]
                                        group-hover:text-[var(--primary)]
                                        transition-colors
                                    ",
                                    fill: "none",
                                    view_box: "0 0 24 24",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    path {
                                        match item.as_str() {
                                            "Home" => {
                                                "M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"
                                            }
                                            _ => {
                                                "M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
                                            }
                                        }
                                    }
                                }
                                span { class: "flex-1", {item.clone()} }
                                // Active indicator
                                div { class: "
                                        absolute right-0 top-1/2 -translate-y-1/2
                                        w-1 h-6 bg-[var(--primary)] rounded-l-full
                                        opacity-0 group-hover:opacity-100
                                        transition-opacity
                                    " }
                            }
                        }
                    }
                }

                // Footer area with user/settings
                div { class: "absolute bottom-0 w-full p-4 border-t border-[var(--outline-variant)/20]",
                    div { class: "flex items-center gap-3 px-3 py-2 rounded-lg hover:bg-[var(--surface-container)]",
                        div { class: "w-8 h-8 rounded-full bg-[var(--primary-container)] flex items-center justify-center",
                            svg {
                                class: "w-5 h-5 text-[var(--on-primary-container)]",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke: "currentColor",
                                stroke_width: "2",
                                path { d: "M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" }
                            }
                        }
                        div { class: "text-sm",
                            div { class: "font-medium text-[var(--on-surface)]", "Developer" }
                            div { class: "text-xs text-[var(--on-surface-variant)]",
                                "admin@example.com"
                            }
                        }
                    }
                }
            }

            // Page Content
            main { class: "
                    lg:pl-72 transition-all duration-300
                    min-h-screen bg-[var(--surface-container-lowest)]
                ",
                Outlet::<Route> {}
            }
        }
    }
}