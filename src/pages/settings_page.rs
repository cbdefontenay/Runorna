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
            r#"document.getElementById("dark-css").disabled = false;"#
        } else {
            r#"document.getElementById("dark-css").disabled = true;"#
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

    rsx! {
        div {
            h1 { "Settings" }
            button {
                onclick: toggle_dark_mode,
                class: "mt-4 px-4 py-2 bg-blue-500 text-white rounded",
                if dark_mode() {
                    "Switch to Light Mode"
                } else {
                    "Switch to Dark Mode"
                }
            }
        }
    }
}
