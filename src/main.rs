mod components;
mod data;
mod helpers;
mod pages;
mod syntax;

use crate::components::NavbarComponent;
use crate::pages::HomePage;
use crate::pages::SettingsPage;
use dioxus::document::eval;
use dioxus::prelude::*;
use crate::data::load_latest_theme;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(NavbarComponent)]
    #[route("/")]
    Home {},
    #[route("/settings")]
    SettingsPage {},
}

const MAIN_CSS: Asset = asset!("/assets/main.css");
const DARK_CSS: Asset = asset!("/assets/dark.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
// const ICON: Asset = asset!("icon.ico");

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let dark_mode = use_signal(|| false);

    use_effect({
        to_owned![dark_mode];
        move || {
            spawn(async move {
                if let Ok((_mode, is_dark)) = load_latest_theme().await {
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

    rsx! {
        // document::Link { rel: "icon", href: ICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link {
            id: "dark-css",
            rel: "stylesheet",
            href: DARK_CSS,
            disabled: Some(true),
        }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        HomePage {}
    }
}

#[component]
fn Settings() -> Element {
    rsx! {
        SettingsPage {}
    }
}
