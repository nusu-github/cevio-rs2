# cevio-ai

CeVIO AI の非公式 Rust バインディング

## 使用方法

`Cargo.toml`に以下を追加してください：

```toml
[dependencies]
cevio-ai = { version = "0.2.0" }
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

    let cevio = CevioAI::with_config(config)?;

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
    let cevio = CevioAI::new()?;
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
