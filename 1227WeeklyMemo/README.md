# 1200RustWeeklyMemo

- [1200RustWeeklyMemo](#1200rustweeklymemo)
  - [Why I learn a rust?](#why-i-learn-a-rust)
  - [Instllation for Rust](#instllation-for-rust)
  - [Print Hello World and additional tutorial](#print-hello-world-and-additional-tutorial)
  - [Rust By Example](#rust-by-example)
    - [1. Hello World](#1-hello-world)
      - [Comments](#comments)
      - [Formatted print](#formatted-print)
        - [Rustc vs Cargo run](#rustc-vs-cargo-run)
        - [ファイルのインポート](#ファイルのインポート)

## Why I learn a rust?

2021年も終わりということで今年を振り返り、社会人になっても「何も成長していない…」と感じたので、あれこれ勉強しようと思い立った。

最初はGoでも勉強しようかな…とか思ってたんですが、どうにも調べたところRustが台頭してきている様子。

GoとRustの比較記事とか色々見たんですが、どうにもこうにも競っている土俵がそもそも違う感じが。

んでもってGoよりRustのほうが低レイヤーで動くし早いしメモリは管理できて嬉しいみたいなので、Rustのほうが伸びるという意見を信じてとりあえず触ってみようと思う。

## Instllation for Rust

[Rust をインストール](https://www.rust-lang.org/ja/tools/install)

![スクリーンショット 2021-12-27 17.45.06](./assets/README_img/スクリーンショット 2021-12-27 17.45.06.png)

なんかもにょもにょ言われた

よく見たらパスが`.profile`と`.zshrc`にしかないみたいなのでfishにもパスをぶち通さないといけないっぽい

[Rustをインストール - macOS & fish-shellの場合](https://qiita.com/ledsun/items/9af5cb594b4f9b6c5523)

毎度fishでエイリアスとか環境変数の通し方を忘れてしまう………

**あとから追記**

冷静に考えたらhomebrewで入れたほうが後々管理が楽かなとおもってぶち消して入れ直した

[rustupのアンインストール方法](https://yuzu441.hateblo.jp/entry/2017/03/22/201027)

> ```sh
> rustup self uninstall
> ```

## Print Hello World and additional tutorial

[新しいプロジェクトを作成する](https://www.rust-lang.org/ja/learn/get-started#:~:text=GEANY-,%E6%96%B0%E3%81%97%E3%81%84%E3%83%97%E3%83%AD%E3%82%B8%E3%82%A7%E3%82%AF%E3%83%88%E3%82%92%E4%BD%9C%E6%88%90%E3%81%99%E3%82%8B,-%E6%96%B0%E3%81%97%E3%81%84Rust%E9%96%8B%E7%99%BA)

かなりさらっといけました

> Rustではパッケージのことをよく「クレート」と呼びます。

これ慣れるまで非常に混乱しそう…

チュートリアルに従ってカニを喋らせた。

>  フェリスはRustコミュニティの非公式マスコットです。多くのRustプログラマは自身のことを「Rustacean」と呼びますが、これは[crustacean](https://en.wikipedia.org/wiki/Crustacean)の言葉遊びから来ています。私たちはフェリスを指し示すのに、性別を表す代名詞の代わりに「they」や「them」などの代名詞を使います。
>
> フェリスは、鉄もしくは鉄に関連することを意味する「ferrous」という形容詞に由来する名前です。錆び（Rust）は鉄の上にできることが多いので、マスコットの名前の起源として面白いですね！

## Rust By Example

> ひとつの言語について何百ページも読むのがあなたの好みに合わなければ、 Rust By Exampleにお任せください。the bookはコードをたくさんの言葉で説明しますが、 RBE (Rust By Example)はたくさんのコード例を示し、説明は最小限です。練習問題もあります！

…らしく、見た感じ一番手っ取り早く触れると思った(悪い癖…)

[Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/)

### 1. Hello World

#### Comments

`///`とか`//!`は[ドキュメント](https://doc.rust-lang.org/stable/rust-by-example/meta/doc.html)にてマークダウンをサポートするらしい。便利だね。

とかいってもこれを使う機会はそうそうなさそうなので知識として頭の片隅に置いておく程度かも。

コメントに関しては結構一般的なコメントアウトと一緒。ノリで使えます。

#### Formatted print

##### Rustc vs Cargo run

一旦ここで湧き出た疑問が`rustc`と`cargo run`の二種類が出てきてるぞ？みたいなところ気になった

rustcがコンパイルして実行ファイルを生成して、cargo runのほうはコンパイルして実行ファイルを直接実行してしまってファイルには残さないみたいな印象がある(予想)

[Rustお勉強 No.1 〜Hello Worldまで〜](https://zenn.dev/mjinno/articles/4db0f1d37e85ee0828f5#rustc-%E3%82%B3%E3%83%9E%E3%83%B3%E3%83%89)

[習慣としてのCargo](https://doc.rust-jp.rs/book-ja/ch01-03-hello-cargo.html#習慣としてのcargo)

> になるにつれて、 その価値を証明するでしょう。複数のクレートからなる複雑なプロジェクトでは、Cargoにビルドを調整してもらうのが遥かに簡単です。
>
> `hello_cargo`プロジェクトは単純ではありますが、今では、Rustのキャリアを通じて使用するであろう本物のツールを多く使用するようになりました。 事実、既存のどんなプロジェクトに取り組むにも、以下のコマンドを使用して、Gitでコードをチェックアウトし、 そのプロジェクトのディレクトリに移動し、ビルドできます:

cargoは非常に便利ツールみたいな感じがするのでこちらメインで使っていこう。

##### ファイルのインポート

気になったのがドカっと多くのファイル作ってmainから呼び出すようにして実験していきたいなとか考えた。

…そうなってくるとファイルのインポートが気になってくる

[Rustでのimport文の書き方](https://qiita.com/MCUYvSCY8E/items/01ddfc8393ddb9d03dfe)

```rust
use directory_name::file_name
```

ここわからんくて躓いた

これ考えるとファイル名は全部アンダースコアで命名した方が良いな…
