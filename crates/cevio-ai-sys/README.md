# cevio-ai-sys

CeVIO AI の低レベル FFI バインディング

このクレートは `windows-bindgen` を使用して自動生成された、CeVIO AI の COM インターフェースへの生の Rust バインディングを提供します。

**注意**: このクレートは直接使用することを意図していません。代わりに [`cevio-ai`](https://crates.io/crates/cevio-ai) クレートを使用してください。これは安全で使いやすい高レベル API を提供します。

## 使用方法（非推奨）

通常は直接使用せず、`cevio-ai` クレートを使用してください：

```toml
[dependencies]
cevio-ai = { version = "0.2.0" }
```

どうしても低レベル API が必要な場合：

```toml
[dependencies]
cevio-ai-sys = { version = "0.2.0" }
```

## 生成について

このクレートのバインディングは以下から自動生成されます：

- `.windows/winmd/CeVIO.Talk.RemoteService2.winmd`
- `windows-bindgen` ツール

## ライセンス

次のいずれかのライセンス:

- [Apache License, Version 2.0](https://github.com/nusu-github/cevio-rs2/blob/master/LICENSE-APACHE)
- [MIT license](https://github.com/nusu-github/cevio-rs2/blob/master/LICENSE-MIT)

## 参考リンク

- [CeVIO AI 公式サイト](https://cevio.jp/)
- [CeVIO AI ユーザーズガイド COMコンポーネントとして利用](https://cevio.jp/guide/cevio_ai/interface/com/)

## 免責事項

- 本ライブラリーは、[CeVIO プロジェクト](https://cevio.jp/)様並びに[テクノスピーチ社](https://www.techno-speech.com/)
  様、その他関係者様とは一切関係がありません。
- 「CeVIO」、「さとうささら」は株式会社フロンティアワークスの登録商標です。

## 注意事項

CeVIO AI の使用にあたっては、CeVIO AI の利用規約を遵守してください。本ライブラリを使用して生成されたコンテンツの利用に関しては、ユーザー自身の責任において行ってください。

CeVIO AI の詳細な使用方法や最新の情報については、上記の公式サイトや COM インターフェースのドキュメントを参照してください。
