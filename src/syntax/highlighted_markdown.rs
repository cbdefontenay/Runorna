use ammonia::clean;
use markdown::{to_html_with_options, Options};
use regex::Regex;
use crate::syntax::highlight_rust_code;

pub fn markdown_with_custom_highlighting(md: &str) -> String {
    let raw_html = to_html_with_options(md, &Options::gfm()).unwrap();

    let re = Regex::new(r#"(?s)<pre><code class="language-rust">(.*?)</code></pre>"#).unwrap();

    let highlighted_html = re.replace_all(&raw_html, |caps: &regex::Captures| {
        let code = caps[1]
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&amp;", "&");

        let highlighted = highlight_rust_code(&code);
        format!(r#"<pre class="bg-gray-900 text-white rounded-lg p-4 overflow-x-auto"><code>{}</code></pre>"#, highlighted)
    });

    ammonia::Builder::default()
        .add_tags(&["span"])
        .add_tag_attributes("span", &["class"])
        .clean(&highlighted_html)
        .to_string()
}
