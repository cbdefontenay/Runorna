use crate::components::AccordionComponent;
use dioxus::prelude::*;

#[component]
pub fn FaqPage() -> Element {
    rsx! {
        div { class: "max-w-3xl mx-auto p-6",
            h1 { class: "text-2xl font-bold text-[var(--primary)] mb-6", "Frequently Asked Questions" }

            // FAQ Item 1
            AccordionComponent {
                accordion_title: "What is Codeor?",
                accordion_description: rsx! {
                    div {
                        p { class: "mb-2",
                            "Codeor (from French \"Code\" and \"or\" meaning gold) is a lightweight application designed to help developers store and organize their code snippets with syntax highlighting using markdown syntax."
                        }
                        p {
                            "Built with Rust and the Dioxus framework, Codeor focuses on simplicity, performance, and a clean user experience."
                        }
                    }
                },
            }

            // FAQ Item 2
            AccordionComponent {
                accordion_title: "Where is my data stored?",
                accordion_description: rsx! {
                    div {
                        p { class: "mb-2", "All your data is stored locally in a SQLite database. This means:" }
                        ul { class: "list-disc pl-5 space-y-1",
                            li { "No data is collected or sent to any cloud services" }
                            li { "Your snippets remain private on your machine" }
                            li { "You maintain full control over your data" }
                            li { "The database file can be easily backed up or migrated" }
                        }
                        p { class: "mt-2", "The database is typically located in your application data directory." }
                    }
                },
            }

            // FAQ Item 3
            AccordionComponent {
                accordion_title: "What programming languages are supported by the markdown?",
                accordion_description: rsx! {
                    div {
                        p { class: "mb-2", "Here is a small list with their usage:" }
                        ul { class: "list-disc pl-5 space-y-1",
                           li { "JavaScript: ```js" }
                            li { "C#: ```c#" }
                            li { "Rust: ```rust" }
                            li { "html: ```html" }
                            li { "css: ```css" }
                            li { "bash: ```bash" }
                            li { "c++: ```cpp" }
                            li { "SQL: ```sql" }
                            li { "Go: ```go" }
                            li { "Python: ```python" }
                            li { "Ruby: ```ruby" }
                            li { "C: ```c" }
                        }
                        p { class: "mt-2", "Languages like Kotlin, swift or TypeScript are not supported." }
                    }
                },
            }

            // FAQ Item 4
            AccordionComponent {
                accordion_title: "What platforms are supported?",
                accordion_description: rsx! {
                    div {
                        p { class: "mb-2", "Codeor is currently available for:" }
                        ul { class: "list-disc pl-5 space-y-1",
                            li { "Windows (64-bit)" }
                            li { "Linux (x86_64 and ARM)" }
                        }
                        p { class: "mt-2", "MacOS support is not currently available." }
                    }
                },
            }

            // FAQ Item 5
            AccordionComponent {
                accordion_title: "How can I contribute or suggest features?",
                accordion_description: rsx! {
                    div {
                        p { class: "mb-2", "Codeor is open source and welcomes community contributions:" }
                        ul { class: "list-disc pl-5 space-y-1",
                            li {
                                strong { "GitHub: " }
                                a {
                                    href: "https://github.com/cbdefontenay/codeor",
                                    class: "text-primary hover:underline",
                                    "github.com/cbdefontenay/codeor"
                                }
                            }
                        }
                        p { class: "mt-2",
                            "Please be aware that Codeor as it is now will not change a lot in the future, since I created it for my purposes only. But still feel free to make suggestions."
                        }
                    }
                },
            }

            // FAQ Item 6
            AccordionComponent {
                accordion_title: "What makes Codeor different from other snippet managers?",
                accordion_description: rsx! {
                    div {
                        p { class: "mb-2", "Codeor stands out by:" }
                        ul { class: "list-disc pl-5 space-y-1",
                            li { "Being completely local and private by design" }
                            li { "Using Rust for performance and reliability and powered by Dioxus" }
                            li { "Supporting markdown with syntax highlighting" }
                            li { "Being lightweight and fast" }
                            li { "Easy and simple design" }
                        }
                    }
                },
            }
        }
    }
}
