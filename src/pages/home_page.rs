use ammonia::clean;
use dioxus::prelude::*;
use markdown::{to_html_with_options, Options};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};
use crate::syntax::markdown_with_custom_highlighting;

#[component]
pub fn HomePage() -> Element {
    let mut user_input_markdown = use_signal(|| "".to_string());

    let safe_html = markdown_with_custom_highlighting(&user_input_markdown());

    rsx! {
        div {
            class: "min-h-screen bg-gradient-to-br from-white via-slate-100 to-slate-200 dark:from-gray-900 dark:via-gray-800 dark:to-gray-900 text-gray-800 dark:text-gray-100 p-6 flex flex-col gap-6",

            h1 {
                class: "text-3xl font-bold text-center mb-4",
                "Live Markdown Preview"
            }

            div {
                class: "flex flex-col md:flex-row gap-6",

                textarea {
                    class: "
                        w-full md:w-1/2 h-[400px] p-4 border border-gray-300 dark:border-gray-700
                        rounded-2xl shadow-sm resize-none bg-white dark:bg-gray-800
                        text-sm font-mono outline-none focus:ring-2 focus:ring-blue-400
                        transition-all
                    ",
                    value: "{user_input_markdown()}",
                    oninput: move |e| user_input_markdown.set(e.value().clone()),
                    placeholder: "Write your markdown here...",
                }

                div {
                    class: "
                        w-full md:w-1/2 p-6 border border-gray-300 dark:border-gray-700
                        rounded-2xl shadow-sm overflow-auto bg-white dark:bg-gray-800
                        prose dark:prose-invert max-w-none transition-all
                    ",
                    dangerous_inner_html: "{safe_html}",
                }
            }
        }
    }
}
