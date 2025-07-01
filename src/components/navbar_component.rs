use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn NavbarComponent() -> Element {
    let nav_items: Vec<(String, Route, &str)> = vec![
        ("Home".to_string(), Route::Home {}, "M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"),
        ("Settings".to_string(), Route::SettingsPage {}, "M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"),
        ("FAQ".to_string(), Route::Faq {}, "M8.25 9a3.75 3.75 0 117.5 0c0 1.586-.876 2.372-1.711 2.947-.653.448-1.289.883-1.289 1.803a.75.75 0 01-1.5 0c0-1.586.876-2.372 1.711-2.947.653-.448 1.289-.883 1.289-1.803a2.25 2.25 0 10-4.5 0 .75.75 0 01-1.5 0ZM12 17.25a.75.75 0 100 1.5.75.75 0 000-1.5Z")
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

            // Slim sidebar nav
            nav {
                class: "
                    fixed top-0 left-0 z-40 w-16 h-screen
                    bg-[var(--surface-container-high)] backdrop-blur-lg
                    border-r border-[var(--outline-variant)/30]
                    text-[var(--on-surface)] transition-transform duration-300 ease-in-out
                    shadow-2xl
                    lg:translate-x-0
                    flex flex-col items-center py-4 gap-4
                ",
                class: if *mobile_menu_open.read() { "translate-x-0" } else { "-translate-x-full lg:translate-x-0" },

                // Compact nav items
                for (item , link , icon_path) in nav_items.iter() {
                    div { class: "
                            relative w-12 h-12 flex items-center justify-center
                            rounded-lg hover:bg-[var(--primary-container)/10]
                            active:bg-[var(--primary-container)/20]
                            group
                        ",
                        Link {
                            to: link.clone(),
                            class: "w-full h-full flex items-center justify-center",
                            // Tooltip container
                            div { class: "
                                    absolute left-full ml-2 px-3 py-2
                                    bg-[var(--surface-container-high)] rounded-lg shadow-lg
                                    text-sm font-medium text-[var(--on-surface)]
                                    opacity-0 group-hover:opacity-100
                                    pointer-events-none
                                    transition-opacity duration-200
                                    whitespace-nowrap
                                ",
                                {item.clone()}
                            }
                            // Icon
                            svg {
                                class: "
                                    w-6 h-6 text-[var(--on-surface-variant)]
                                    group-hover:text-[var(--on-surface)]
                                ",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke: "currentColor",
                                stroke_width: "2",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    d: "{icon_path}",
                                }
                            }
                        }
                    }
                }
            }

            // Page Content
            main { class: "
                    lg:pl-16 transition-all duration-300
                    min-h-screen bg-[var(--surface-container-lowest)]
                ",
                Outlet::<Route> {}
            }
        }
    }
}