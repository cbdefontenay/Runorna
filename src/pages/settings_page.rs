use crate::data::{load_latest_theme, save_theme};
use dioxus::document::eval;
use dioxus::prelude::*;

#[component]
pub fn SettingsPage() -> Element {
    let mut dark_mode = use_signal(|| false);

    use_effect({
        to_owned![dark_mode];
        move || {
            spawn(async move {
                if let Ok((mode, is_dark)) = load_latest_theme().await {
                    dark_mode.set(is_dark);
                    let js = if is_dark {
                        r#"document.getElementById("dark-css").disabled = false;"#
                    } else {
                        r#"document.getElementById("dark-css").disabled = true;"#
                    };
                    eval(js);
                }
            });
        }
    });

    let toggle_dark_mode = move |_| {
        let enabled = !dark_mode();
        dark_mode.set(enabled);

        let js = if enabled {
            r#"document.getElementById("dark-css").disabled = false; document.getElementById("main-css").disabled = true;"#
        } else {
            r#"document.getElementById("dark-css").disabled = true; document.getElementById("main-css").disabled = false;"#
        };
        eval(js);

        // Save theme to DB
        spawn({
            let mode = if enabled { "dark" } else { "light" }.to_string();
            async move {
                let _ = save_theme(mode, enabled).await;
            }
        });
    };

    let toggle_light_mode = move |_| {
        dark_mode.set(false);

        let js = r#"document.getElementById("dark-css").disabled = true; document.getElementById("main-css").disabled = false;"#;
        eval(js);

        spawn({
            async move {
                let _ = save_theme("light".to_string(), false).await;
            }
        });
    };

    rsx! {
        div { class: "min-h-screen bg-[var(--background)] text-[var(--on-background)]",
            div { class: "max-w-3xl mx-auto px-6 py-12",
                div { class: "bg-[var(--surface)] rounded-xl shadow-lg p-8",
                    div { class: "flex items-center justify-between mb-8",
                        h1 { class: "text-3xl font-bold text-[var(--primary)]", 
                            "Settings" 
                        }
                        div { class: "flex items-center space-x-4",
                            span { class: "text-[var(--on-surface-variant)]",
                                if dark_mode() {
                                    "Dark Mode"
                                } else {
                                    "Light Mode"
                                }
                            }
                            button {
                                onclick: toggle_dark_mode,
                                class: "relative inline-flex h-6 w-11 items-center rounded-full transition-colors focus:outline-none focus:ring-2 focus:ring-[var(--primary)] focus:ring-offset-2",
                                class: if dark_mode() {
                                    "bg-[var(--primary)]"
                                } else {
                                    "bg-[var(--outline-variant)]"
                                },
                                span { class: "sr-only", "Toggle dark mode" }
                                span { 
                                    class: "inline-block h-4 w-4 transform rounded-full bg-white transition-transform",
                                    class: if dark_mode() {
                                        "translate-x-6"
                                    } else {
                                        "translate-x-1"
                                    }
                                }
                            }
                        }
                    }

                    div { class: "space-y-6",
                        div { class: "border-t border-[var(--outline-variant)] pt-6",
                            h2 { class: "text-xl font-semibold text-[var(--on-surface)] mb-4", 
                                "Appearance" 
                            }
                            div { class: "grid grid-cols-2 gap-4",
                                button {
                                    onclick: toggle_light_mode,
                                    class: "rounded-lg p-4 border-2 transition-all",
                                    class: if !dark_mode() {
                                        "border-[var(--primary)] bg-[var(--primary-container)]"
                                    } else {
                                        "border-[var(--outline-variant)] hover:border-[var(--primary)]"
                                    },
                                    div { class: "flex flex-col items-center",
                                        div { class: "w-full h-24 mb-2 rounded bg-[var(--surface-bright)] border border-[var(--outline-variant)]" }
                                        span { "Light" }
                                    }
                                }
                                button {
                                    onclick: toggle_dark_mode,
                                    class: "rounded-lg p-4 border-2 transition-all",
                                    class: if dark_mode() {
                                        "border-[var(--primary)] bg-[var(--primary-container)]"
                                    } else {
                                        "border-[var(--outline-variant)] hover:border-[var(--primary)]"
                                    },
                                    div { class: "flex flex-col items-center",
                                        div { class: "w-full h-24 mb-2 rounded bg-[var(--surface-dim)] border border-[var(--outline-variant)]" }
                                        span { "Dark" }
                                    }
                                }
                            }
                        }

                        div { class: "border-t border-[var(--outline-variant)] pt-6",
                            h2 { class: "text-xl font-semibold text-[var(--on-surface)] mb-4", 
                                "Account Settings" 
                            }
                            div { class: "space-y-4",
                                div { class: "flex justify-between items-center",
                                    span { class: "text-[var(--on-surface-variant)]", "Email" }
                                    span { class: "font-medium", "user@example.com" }
                                }
                                div { class: "flex justify-between items-center",
                                    span { class: "text-[var(--on-surface-variant)]", "Account status" }
                                    span { class: "font-medium text-[var(--primary)]", "Active" }
                                }
                            }
                        }
                    }

                    div { class: "mt-8 flex justify-end",
                        button { 
                            class: "px-6 py-2 rounded-lg bg-[var(--primary)] text-[var(--on-primary)] font-medium hover:bg-[var(--surface-tint)] transition-colors",
                            "Save Changes"
                        }
                    }
                }
            }
        }
    }
}