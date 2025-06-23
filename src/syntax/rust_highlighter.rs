use std::collections::HashSet;

pub fn highlight_rust_code(code: &str) -> String {
    let rust_keywords: HashSet<&str> = [
        "let", "mut", "const", "fn", "struct", "enum", "impl", "trait", "match",
        "if", "else", "while", "loop", "for", "in", "use", "mod", "pub", "crate",
        "super", "self", "Self", "return", "break", "continue", "as", "ref", "type",
    ].into_iter().collect();

    code.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|token| {
                    if rust_keywords.contains(token) {
                        format!(r#"<span class="text-purple-500 font-semibold">{}</span>"#, token)
                    } else {
                        token.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect::<Vec<_>>()
        .join("\n")
}
