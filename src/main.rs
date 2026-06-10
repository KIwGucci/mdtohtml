mod mycss;
mod highlight;

use clap::Parser;
use maud::{html, Markup, PreEscaped, Render, DOCTYPE};
use pulldown_cmark::{self, html::push_html, Event, Tag, TagEnd, CodeBlockKind};
use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// 変換対象のMarkdownファイルパス
    filename: Vec<PathBuf>,
}

fn main() -> MyResult<()> {
    let cli = Cli::parse();
    markdown_to_html(&cli.filename)?;
    Ok(())
}

fn markdown_to_html(targets: &[PathBuf]) -> MyResult<()> {
    for target in targets {
        if !target.is_file() {
            eprintln!("スキップ: {} はファイルではありません", target.display());
            continue;
        }
        match target.extension().and_then(|e| e.to_str()) {
            Some("md" | "markdown" | "mdown") => {
                if let Err(e) = convert_md(target) {
                    eprintln!("変換エラー [{}]: {}", target.display(), e);
                }
            }
            _ => eprintln!(
                "{} は無効です。md / markdown / mdown の拡張子を持つファイルを指定してください",
                target.display()
            ),
        }
    }
    Ok(())
}

// ── Markdown レンダラー ──────────────────────────────────────────────────────

struct Markdown<T: AsRef<str>>(T);

impl<T: AsRef<str>> Render for Markdown<T> {
    fn render(&self) -> Markup {
        let mut opts = pulldown_cmark::Options::empty();
        opts.insert(pulldown_cmark::Options::ENABLE_TABLES);
        opts.insert(pulldown_cmark::Options::ENABLE_SMART_PUNCTUATION);
        opts.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
        opts.insert(pulldown_cmark::Options::ENABLE_TASKLISTS);
        opts.insert(pulldown_cmark::Options::ENABLE_MATH);
        opts.insert(pulldown_cmark::Options::ENABLE_GFM);

        // コードブロックをシンタックスハイライト済み HTML に差し替える
        let raw_events: Vec<Event> = pulldown_cmark::Parser::new_ext(self.0.as_ref(), opts).collect();
        let events = highlight_code_blocks(raw_events);

        let mut html_out = String::new();
        push_html(&mut html_out, events.into_iter());

        PreEscaped(html_out)
    }
}

/// コードブロックのイベント列を syntect でハイライト済みの HTML に置き換える
fn highlight_code_blocks(events: Vec<Event>) -> Vec<Event> {
    let mut out: Vec<Event> = Vec::with_capacity(events.len());
    let mut i = 0;

    while i < events.len() {
        match &events[i] {
            // コードブロック開始
            Event::Start(Tag::CodeBlock(kind)) => {
                let lang = match kind {
                    CodeBlockKind::Fenced(lang) => lang.as_ref().to_owned(),
                    CodeBlockKind::Indented => String::new(),
                };
                i += 1;

                // コードブロック内のテキストを収集
                let mut code = String::new();
                while i < events.len() {
                    match &events[i] {
                        Event::Text(t) => {
                            code.push_str(t);
                            i += 1;
                        }
                        Event::End(TagEnd::CodeBlock) => {
                            i += 1;
                            break;
                        }
                        _ => {
                            i += 1;
                        }
                    }
                }

                // ハイライト済み HTML を挿入
                let highlighted = highlight::highlight_code(&code, &lang);
                out.push(Event::Html(highlighted.into()));
            }
            _ => {
                out.push(events[i].clone());
                i += 1;
            }
        }
    }
    out
}

// ── ファイル変換 ─────────────────────────────────────────────────────────────

fn convert_md(target: &Path) -> MyResult<()> {
    let mdcss = mycss::gen_mdcss();

    let mut markdownstr = String::new();
    BufReader::new(File::open(target)?).read_to_string(&mut markdownstr)?;

    // 出力先パスを組み立て
    let mut out_path = env::current_dir()?;
    out_path.push(target);
    out_path.set_extension("html");

    let filetitle = out_path
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "document".to_owned());

    let markup = html! {
        (DOCTYPE)
        html {
            head {
                meta charset="UTF-8";
                title { (filetitle) }
                style { (mdcss) }
            }
            body {
                (Markdown(&markdownstr).render())
            }
        }
    };

    let out_path_str = out_path.to_string_lossy();
    println!("生成しました: {}", out_path_str);

    File::create(out_path.as_path())?.write_all(markup.render().into_string().as_bytes())?;
    Ok(())
}
