mod components;
mod pages;
mod syntax;

use crate::components::NavbarComponent;
use crate::pages::HomePage;
use crate::pages::SettingsPage;
use dioxus::prelude::*;

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
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
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
