# cevio-ai

CeVIO AIの非公式Rustバインディング

## 使用方法

`Cargo.toml`に以下を追加してください：

```toml
[dependencies]
cevio-ai = { git = "https://github.com/nusu-github/cevio-rs2" }
```

### 基本的な使用例

```rust
use cevio_ai::*;

fn main() -> Result<()> {
    // CeVIOインスタンスを作成
    let cevio = Cevio::new()?;

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

### 設定を使った初期化

```rust
use cevio_ai::*;

fn main() -> Result<()> {
    let config = CevioConfigBuilder::default()
        .start_host(true)  // 自動起動
        .initial_cast("さとうささら")
        .initial_volume(Volume::new(80)?)
        .initial_speed(Speed::new(60)?)
        .build()?;

    let cevio = Cevio::with_config(config)?;

    // 既に設定済みなのですぐに使用可能
    let state = cevio.speak("こんにちは")?;
    state.wait()?;
    Ok(())
}
```

### プリセットを使用した音声設定

```rust
use cevio_ai::*;

fn main() -> Result<()> {
    let cevio = Cevio::new()?;
    cevio.start(false)?;

    // エネルギッシュなプリセットを適用
    let cast = CastBuilder::default()
        .cast("さとうささら")
        .from_preset(VoicePreset::Energetic)
        .build()?;

    cevio.apply_cast(&cast)?;

    // 利用可能なプリセット:
    // - Normal: 標準的な設定
    // - Fast: 早口
    // - Slow: ゆっくり
    // - HighPitch: 高い声
    // - LowPitch: 低い声
    // - Energetic: 元気な声
    // - Calm: 落ち着いた声

    Ok(())
}
```

## API ドキュメント

詳細なAPIドキュメントについては、プロジェクトディレクトリで`cargo doc --open`を実行してください。

## アーキテクチャ

本ライブラリは2つのクレートで構成されています：

- **`cevio-ai-sys`**: 低レベルFFIバインディング（自動生成）
- **`cevio-ai`**: 高レベル安全API

## 依存クレート

- `bounded-integer`: 型安全な範囲制限付き整数
- `derive_builder`: Builderパターンの自動生成
- `parking_lot`: 効率的な同期プリミティブ
- `thiserror`: エラー型の生成
- `windows`: Windows バインディング

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

- 本ライブラリーは、[CeVIOプロジェクト](https://cevio.jp/)様並びに[テクノスピーチ社](https://www.techno-speech.com/)
  様、その他関係者様とは一切関係がありません。
- 「CeVIO」、「さとうささら」は株式会社フロンティアワークスの登録商標です。

## 注意事項

CeVIO AIの使用にあたっては、CeVIO AIの利用規約を遵守してください。本ライブラリを使用して生成されたコンテンツの利用に関しては、ユーザー自身の責任において行ってください。

CeVIO AIの詳細な使用方法や最新の情報については、上記の公式サイトやCOMインターフェースのドキュメントを参照してください。
