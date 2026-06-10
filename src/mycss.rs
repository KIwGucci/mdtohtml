/// CSS を文字列として返す。
/// 色のカスタマイズは冒頭の `:root { --変数名: 値; }` を編集するだけでOK。
pub fn gen_mdcss() -> String {
    r#"
/* =====================================================================
   カラーテーマ変数 — ここだけ編集すれば全体の色が変わります
   ===================================================================== */
:root {
    /* ページ */
    --color-bg:           #f5f5f5;
    --color-text:         #333333;
    --color-text-muted:   #555555;

    /* 見出し */
    --color-heading:      #2c3e50;

    /* リンク */
    --color-link:         #3498db;

    /* 区切り線・ボーダー */
    --color-border:       #dddddd;

    /* テーブル */
    --color-th-bg:        #3498db;
    --color-th-text:      #ffffff;
    --color-tr-even:      #f2f2f2;

    /* 引用 */
    --color-blockquote:   #3498db;

    /* コードブロック背景（syntect が inline style を生成するため参考値） */
    --color-code-bg:      #ede8db;   /* base16-ocean.dark の背景色 */

    /* インラインコード */
    --color-inline-code-bg:   #1e1e1e;
    --color-inline-code-text: #d4d4d4;
}

/* =====================================================================
   ベーススタイル
   ===================================================================== */
body {
    font-family: 'Segoe UI', Arial, sans-serif;
    line-height: 1.6;
    color: var(--color-text);
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
    background-color: var(--color-bg);
}

h1, h2, h3, h4, h5, h6 {
    color: var(--color-heading);
    margin-top: 24px;
    margin-bottom: 16px;
    font-weight: 600;
}

h1 {
    font-size: 2.5em;
    border-bottom: 1px solid var(--color-border);
    padding-bottom: 10px;
}

h2 { font-size: 2em; }
h3 { font-size: 1.5em; }

a {
    color: var(--color-link);
    text-decoration: none;
}
a:hover { text-decoration: underline; }

hr {
    border: 0;
    border-top: 1px solid var(--color-border);
    margin: 20px 0;
}

img {
    max-width: 100%;
    height: auto;
}

/* =====================================================================
   引用
   ===================================================================== */
blockquote {
    border-left: 4px solid var(--color-blockquote);
    margin: 0;
    padding-left: 20px;
    color: var(--color-text-muted);
}

/* =====================================================================
   テーブル
   ===================================================================== */
table {
    border-collapse: collapse;
    width: 100%;
    margin-bottom: 16px;
}

th, td {
    text-align: left;
    padding: 12px;
    border-bottom: 1px solid var(--color-border);
}

th {
    background-color: var(--color-th-bg);
    color: var(--color-th-text);
}

tr:nth-child(even) {
    background-color: var(--color-tr-even);
}

/* =====================================================================
   コード — syntect が <pre><code> に inline style を付与するため、
   ここではレイアウトと余白だけを制御する
   ===================================================================== */

/* コードブロック全体のラッパー */
pre {
    background-color: var(--color-code-bg);
    border-radius: 6px;
    padding: 0;          /* syntect の <pre> に余白を付けない（二重余白防止） */
    overflow: auto;
    margin-bottom: 16px;
}

pre code {
    display: block;
    padding: 16px;
    overflow-x: auto;
    font-family: Consolas, 'Cascadia Code', Monaco, monospace;
    font-size: 0.9em;
    line-height: 1.5;
    /* 色は syntect の inline style が上書きするため不要 */
}

/* インラインコード（コードブロック外） */
code {
    background-color: var(--color-inline-code-bg);
    color: var(--color-inline-code-text);
    border-radius: 4px;
    font-family: Consolas, 'Cascadia Code', Monaco, monospace;
    font-size: 0.9em;
    padding: 2px 5px;
}

/* <pre> 内の <code> には上記インラインコード色を適用しない */
pre code {
    background-color: transparent;
    color: inherit;
    padding: 0;
    border-radius: 0;
}
"#
    .to_string()
}
