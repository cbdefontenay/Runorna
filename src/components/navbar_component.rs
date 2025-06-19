use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn NavbarComponent() -> Element {
    let nav_items: Vec<(String, Route)> = vec![
        ("Home".to_string(),  Route::Home {}),
        ("Settings".to_string(), Route::SettingsPage {}),
    ];

    rsx! {
        nav { class: "
                hidden lg:flex flex-col w-64 h-screen fixed top-0 left-0 z-40 bg-[var(--on-primary-fixed)]  backdrop-blur-xl shadow-lg border-r border-gray-200 dark:border-gray-800 text-gray-900 dark:text-gray-100 transition-all
            ",
            // Header
            div { class: "text-[var(--primary-fixed-dim)] px-6 py-5 border-b border-gray-300 dark:border-gray-700 text-xl font-semibold",
                "FluentPanel"
            }

            // Nav Items
            ul { class: "flex flex-col gap-2 px-4 py-6",
                for (item , link) in nav_items.iter() {
                    li { class: "px-4 py-2 rounded-xl hover:bg-white/40 text-[var(--primary-fixed-dim)]  transition-colors cursor-pointer",
                        Link { class: "", to: {link.clone()}, {item.clone()} }
                    }
                }
            }
        }
        main {
                class: "pl-64",
                Outlet::<Route> {}
            }
    }
}
