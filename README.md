# cevio-ai

CeVIO AI の非公式 Rust バインディング

## 使用方法

`Cargo.toml`に以下を追加してください：

```toml
[dependencies]
cevio-ai = { version = "0.2.1" }
```

### 基本的な使用例

```rust
use cevio_ai::*;

fn main() -> Result<()> {
    // CeVIOインスタンスを作成
    let cevio = CevioAI::new()?;

    // CeVIO AIを起動
    cevio.start(false)?;

    // 音声設定（型安全なパラメータ）
    let cast = CastBuilder::default()
        .cast("さとうささら")
        .volume(Volume::new(100).unwrap())
        .speed(Speed::new(50).unwrap())
        .tone(Tone::new(50).unwrap())
        .build()?;

    cevio.apply_cast(&cast)?;

    // 音声合成と再生
    let state = cevio.speak("こんにちは")?;
    state.wait()?;

    // 音素データを取得
    let phonemes = cevio.phonemes("はじめまして")?;
    for phoneme in &phonemes {
        println!("{}: {:.2}s - {:.2}s",
                 phoneme.phoneme(), phoneme.start_time(), phoneme.end_time());
    }

    // WAVファイルに出力
    cevio.output_wave_to_file("さようなら", "output.wav")?;

    // 終了
    cevio.close(CloseMode::Interactive)?;
    Ok(())
}
```

## API ドキュメント

詳細な API ドキュメントについては、プロジェクトディレクトリで`cargo doc --open`を実行してください。

## アーキテクチャ

本ライブラリは 2 つのクレートで構成されています：

- **`cevio-ai-sys`**: 低レベル FFI バインディング（自動生成）
- **`cevio-ai`**: 高レベル安全 API

## ライセンス

次のいずれかのライセンス:

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT license](LICENSE-MIT)

## 貢献

貢献を歓迎いたします！プルリクエストを気軽に送ってください。

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
