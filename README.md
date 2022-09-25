# 燐字海第二版

かつて Google Sites で管理されていた [燐字海](https://sites.google.com/site/linzizihai/) を独自でホストすることにし、ついでにデータ構造などに改良を加えたもの。

## 使い方

### ページ生成

* [Rust をインストール](https://www.rust-lang.org/ja/tools/install)
* `cargo run` を実行する

ことにより、`entries/${燐字名}_${言語名}.json` のファイルが全て統合され、 燐字海本体を構成する `docs/${燐字名} - 燐字海.html` が出力される。

そして、GitHub Pages をセットアップしてあるので、このリポジトリに commit & push することで [http://jurliyuuri.github.io/2bhai_liina_baani_bhodi/](http://jurliyuuri.github.io/2bhai_liina_baani_bhodi/) からアクセスできるようになる。

### ページをいじりたいとき

`docs/` 内の他のファイルは変更されないため、画像だったりとかは `docs/` 内の適切な場所に配置することで上手くいく。

燐字記事の見た目などをいじりたいときには、`templates/linzklar.html` をいじるべきである。

言語名からの外部リンクは、 `config_links.tsv` をいじるべきである。

## 現状自動化できていること

* JSON を読み、それを元にページを出力する

## 現状自動化できていないこと

* 第五旬以降のほとんどのエントリーは燐声集から自動生成されたものであるので、燐声集と被るやつはなるべく燐声集のデータから読むようにすべきだが、していない。
