use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::html::{styled_line_to_highlighted_html, IncludeBackground};
use syntect::parsing::SyntaxSet;

/// Markdown のコードブロック言語名を syntect が認識できる名前に正規化する
fn normalize_lang(lang: &str) -> String {
    // 言語名の先頭にオプション（例: "rust,no_run")が付く場合があるので最初のトークンだけ使う
    let base = lang.split([',', ' ']).next().unwrap_or("").trim();
    let key = base.to_ascii_lowercase();
    match key.as_str() {
        "rs" | "rust" => "Rust".to_string(),
        "js" | "javascript" => "JavaScript".to_string(),
        "ts" | "typescript" => "TypeScript".to_string(),
        "py" | "python" => "Python".to_string(),
        "sh" | "bash" | "shell" => "Bash".to_string(),
        "toml" => "TOML".to_string(),
        "json" => "JSON".to_string(),
        "yaml" | "yml" => "YAML".to_string(),
        "html" | "htm" => "HTML".to_string(),
        "css" => "CSS".to_string(),
        "c" => "C".to_string(),
        "cpp" | "c++" => "C++".to_string(),
        "java" => "Java".to_string(),
        "go" => "Go".to_string(),
        "rb" | "ruby" => "Ruby".to_string(),
        "sql" => "SQL".to_string(),
        "xml" => "XML".to_string(),
        "md" | "markdown" => "Markdown".to_string(),
        other => other.to_string(), // そのまま渡して syntect に判断させる
    }
}

/// コードとその言語名を受け取り、ハイライト済みの HTML 文字列を返す。
/// 認識できない言語の場合はプレーンな `<pre><code>` ブロックにフォールバックする。
pub fn highlight_code(code: &str, lang: &str) -> String {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    // テーマ: "base16-ocean.dark" / "base16-eighties.dark" / "Solarized (dark)" 等から選択
    let theme = &ts.themes["Solarized (light)"];

    let norm = normalize_lang(lang);
    let syntax = ss
        .find_syntax_by_name(&norm)
        .or_else(|| ss.find_syntax_by_extension(&norm))
        .unwrap_or_else(|| ss.find_syntax_plain_text());

    let mut hl = HighlightLines::new(syntax, theme);
    let mut html = String::from("<pre><code>");

    for line in syntect::util::LinesWithEndings::from(code) {
        match hl.highlight_line(line, &ss) {
            Ok(ranges) => {
                match styled_line_to_highlighted_html(&ranges[..], IncludeBackground::No) {
                    Ok(line_html) => html.push_str(&line_html),
                    Err(_) => html.push_str(&escape_html(line)),
                }
            }
            Err(_) => html.push_str(&escape_html(line)),
        }
    }

    html.push_str("</code></pre>");
    html
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
