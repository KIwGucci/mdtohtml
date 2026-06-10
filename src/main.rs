mod mycss;
use clap::Parser;
use maud::{html, Markup, PreEscaped, Render, DOCTYPE};
use pulldown_cmark::{self, html::push_html};
use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Parser)]
#[command(author,version,about,long_about=None)]
struct Cli {
    // [ファイルパス]
    filename: Vec<PathBuf>,
}

fn main() -> MyResult<()> {
    let cli = Cli::parse();
    let readfiles = cli.filename;
    // プログラムの引数から対象ファイル名を取得
    markdown_to_html(&readfiles)?;
    Ok(())
}

fn markdown_to_html(targets: &[PathBuf]) -> MyResult<()> {
    for target in targets {
        // // markdownファイルを読み込み
        if !target.is_file() {
            continue;
        }
        let md_extention = match target.extension() {
            Some(ex) => ex.to_string_lossy().to_string(),
            None => continue,
        };
        match md_extention.as_str() {
            "md" | "markdown" | "mdown" => {
                if let Err(e) = convert_md(target) {
                    eprintln!("{}", e)
                }
            }
            _ => println!(
                "{}は無効です。md|markdown|mdownのいずれかの拡張子を含むファイルを指定してください",
                target.to_str().unwrap()
            ),
        }
    }

    Ok(())
}

/// Renders a block of Markdown using `pulldown-cmark`.
struct Markdown<T: AsRef<str>>(T);

impl<T: AsRef<str>> Render for Markdown<T> {
    fn render(&self) -> Markup {
        // Generate raw HTML
        // マークアップの時のオプション機能を設定
        let mut parseoption = pulldown_cmark::Options::empty();
        // テーブル機能
        parseoption.insert(pulldown_cmark::Options::ENABLE_TABLES);
        // ハイフンや...を連続文字を適切な形に変形してくれる
        parseoption.insert(pulldown_cmark::Options::ENABLE_SMART_PUNCTUATION);
        // 取り消し線
        parseoption.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
        // タスクリスト
        parseoption.insert(pulldown_cmark::Options::ENABLE_TASKLISTS);
        // Tex
        parseoption.insert(pulldown_cmark::Options::ENABLE_MATH);
        // GitHub Flaverd Markdown
        parseoption.insert(pulldown_cmark::Options::ENABLE_GFM);

        let mut my_html = String::new();
        let parser = pulldown_cmark::Parser::new_ext(self.0.as_ref(), parseoption);
        push_html(&mut my_html, parser);

        PreEscaped(my_html)
    }
}

fn convert_md(target: &Path) -> MyResult<()> {
    // mycssファイルのcssをstringとして取得
    let mdcss = mycss::gen_mdcss();
    let targetfile = File::open(target)?;
    // Readerを作る
    let mut reader = BufReader::new(targetfile);
    // Stringバッファを確保
    let mut markdownstr = String::new();
    // Stringバッファにファイルの内容を読み込む
    reader.read_to_string(&mut markdownstr)?;

    // htmlファイルの名前を生成
    // カレントディレクトリを取得
    let mut filename = env::current_dir()?;
    // 対象マークダウンのファイル名を追加
    filename.push(target);
    // html titleにマークダウンの拡張子抜きのファイル名を割り当て
    let filetitle = filename.file_stem();

    if let Some(ft) = filetitle {
        let filetitle = ft.to_string_lossy();
        //
        // ヘッダを生成
        let headtext = html! {
            header{
            meta charset="UTF-8";
            title {(filetitle)};
            style {(mdcss)};
            }
        };

        // markdownをhtmlに変換準備
        let markuptext = html! {
            (DOCTYPE)html{
                (headtext)
                (Markdown(&markdownstr).render())
            }
        };
        // filenameの拡張子をhtmlに変更
        filename.set_extension("html");
        let filename = filename.to_str().unwrap();
        println!("The following files have been created:\n{}", filename);
        // writerを準備
        let mut writebuffer = File::create(filename)?;
        // マークダウンをhtmlに変換レンダリング
        let outcontent = markuptext.render().into_string();
        // 書き込み
        let _wtlen = writebuffer.write(outcontent.as_bytes())?;
    };
    Ok(())
}
