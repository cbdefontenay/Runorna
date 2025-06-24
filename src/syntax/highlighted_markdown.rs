use pulldown_cmark::Options;
use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag, TagEnd};
use std::sync::LazyLock;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

pub fn markdown_to_html(markdown: &str, theme_name: &str) -> String {
    static SYNTAX_SET: LazyLock<SyntaxSet> = LazyLock::new(SyntaxSet::load_defaults_newlines);
    static THEME_SET: LazyLock<ThemeSet> = LazyLock::new(ThemeSet::load_defaults);

    let theme = THEME_SET
        .themes
        .get(theme_name)
        .unwrap_or_else(|| &THEME_SET.themes["base16-eighties.dark"]);

    let mut sr = SYNTAX_SET.find_syntax_plain_text();
    let mut code = String::new();
    let mut code_block = false;

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_MATH);
    options.insert(Options::ENABLE_GFM);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    options.insert(Options::ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS);
    options.insert(Options::ENABLE_SUBSCRIPT);
    options.insert(Options::ENABLE_SUPERSCRIPT);

    let parser = Parser::new_ext(markdown, options).filter_map(|event| match event {
        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
            let lang = lang.trim();
            sr = SYNTAX_SET
                .find_syntax_by_token(&lang)
                .unwrap_or_else(|| SYNTAX_SET.find_syntax_plain_text());
            code_block = true;
            None
        }
        Event::End(TagEnd::CodeBlock) => {
            let html = highlighted_html_for_string(&code, &SYNTAX_SET, &sr, theme)
                .unwrap_or(code.clone());
            code.clear();
            code_block = false;
            Some(Event::Html(html.into()))
        }
        Event::Code(code) => Some(Event::Html(
            format!(r#"<code class="inline">{}</code>"#, code).into(),
        )),
        Event::Html(html) => Some(Event::Html(html)),
        Event::DisplayMath(text) => Some(Event::Html(
            format!(r#"<span class="math">{}</span>"#, text).into(),
        )),
        Event::Text(t) => {
            if code_block {
                code.push_str(&t);
                return None;
            }
            Some(Event::Text(t))
        }
        _ => Some(event),
    });

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output
}