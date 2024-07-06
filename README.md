# cevio-rs2

CeVIO AIの非公式Rustバインディング

## 使用方法

`Cargo.toml`に以下を追加してください：

```toml
[dependencies]
cevio = { git = "https://github.com/nusu-github/cevio-rs2" }
```

基本的な使用例：

```rust
use anyhow::Result;
use cevio::{Cevio, CloseMode};

fn main() -> Result<()> {
    let cevio = Cevio::new()?;

    // CeVIO AIを起動
    cevio.start(false)?;

    // キャストと音声パラメータを設定
    let cast = CastBuilder::default()
        .cast("さとうささら")
        .volume(100)
        .build()?;
    cevio.apply_cast(&cast)?;

    // 音声を生成
    let state = cevio.speak("こんにちは")?;
    state.wait()?;

    // 音素データを取得
    let phonemes = cevio.phonemes("はじめまして")?;
    println!("{:?}", phonemes);

    // CeVIO AIを終了
    cevio.close(CloseMode::Interactive)?;

    Ok(())
}
```

## API ドキュメント

詳細なAPIドキュメントについては、プロジェクトディレクトリで`cargo doc --open`を実行してください。

## 依存クレート

このプロジェクトは以下の依存クレートを使用しています：

- `anyhow`: エラーハンドリング
- `windows-rs`: Windows APIバインディング

## ライセンス

- [Apache License, Version 2.0](LICENSE)

## 貢献

貢献を歓迎いたします！プルリクエストを気軽に送ってください。

## 参考リンク

- [CeVIO AI 公式サイト](https://cevio.jp/)
- [CeVIO AI ユーザーズガイド COMコンポーネントとして利用](https://cevio.jp/guide/cevio_ai/interface/com/)

## 免責事項

- 本ライブラリーは、[CeVIOプロジェクト](https://cevio.jp/)様並びに[テクノスピーチ社](https://www.techno-speech.com/)
  様、その他関係者様とは一切関係がありません。
- 「CeVIO」、「さとうささら」は株式会社フロンティアワークスの、登録商標です。

## 注意事項

CeVIO AIの使用にあたっては、CeVIO AIの利用規約を遵守してください。本ライブラリを使用して生成されたコンテンツの利用に関しては、ユーザー自身の責任において行ってください。

CeVIO AIの詳細な使用方法や最新の情報については、上記の公式サイトやCOMインターフェースのドキュメントを参照してください。
