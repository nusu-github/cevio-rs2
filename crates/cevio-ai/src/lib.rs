//! # `CeVIO AI` Rust API
//!
//! `CeVIO AI`の機能をRustから安全に利用するためのライブラリです。
//!
//! ## 主な機能
//!
//! - **音声合成**: テキストから音声を生成
//! - **音声パラメータ制御**: 音量、速度、音程、抑揚、声質の調整
//! - **キャスト管理**: 利用可能なキャストの一覧取得と切り替え
//! - **音素データ取得**: テキストの音素情報とタイミングを取得
//! - **WAVファイル出力**: 音声をファイルに保存
//!
//! ## 基本的な使用方法
//!
//! ```rust,no_run
//! use cevio_ai::*;
//!
//! fn main() -> Result<()> {
//!     // CeVIOインスタンスを作成
//!     let cevio = Cevio::new()?;
//!     
//!     // CeVIO AIを起動
//!     cevio.start(false)?;
//!     
//!     // 音声設定
//!     let cast = CastBuilder::default()
//!         .cast("さとうささら")
//!         .volume(Volume::new(100).unwrap())
//!         .build()?;
//!     
//!     cevio.apply_cast(&cast)?;
//!     
//!     // 音声合成と再生
//!     let state = cevio.speak("こんにちは")?;
//!     state.wait()?;
//!     
//!     // 終了
//!     cevio.close(CloseMode::Interactive)?;
//!     Ok(())
//! }
//! ```
//!
//! ## 設定付きでの初期化
//!
//! ```rust,no_run
//! use cevio_ai::*;
//!
//! fn main() -> Result<()> {
//!     let config = CevioConfigBuilder::default()
//!         .start_host(true)
//!         .initial_cast("さとうささら")
//!         .initial_volume(Volume::new(80)?)
//!         .build()?;
//!         
//!     let cevio = Cevio::with_config(config)?;
//!     
//!     // 既に設定済みなのですぐに使用可能
//!     let state = cevio.speak("こんにちは")?;
//!     state.wait()?;
//!     Ok(())
//! }
//! ```

mod cevio;
mod com_manager;
mod error;
mod parameter;

pub use cevio::*;
pub use error::*;
pub use parameter::*;

#[cfg(test)]
mod tests {
    use serial_test::serial;
    use std::time::Instant;

    use super::*;

    #[test]
    #[serial]
    // 公式のサンプル
    // https://cevio.jp/guide/cevio_ai/interface/com/
    fn minimal() -> Result<()> {
        let cevio = Cevio::new()?;

        // 【CeVIO AI】起動
        cevio.start(false)?;
        println!("CeVIO AI started");

        let cast = CastBuilder::default()
            // キャスト設定
            .cast("さとうささら")
            // 音量設定
            .volume(Volume::new(100).unwrap())
            // 抑揚設定
            .tone_scale(ToneScale::new(100).unwrap())
            .build()?;
        println!("Cast built: {cast:?}");

        cevio.apply_cast(&cast)?;
        println!("Cast applied");

        // （例）再生
        let e = cevio.speak("こんにちは")?;
        println!("Speaking started");
        e.wait_timeout(30.0)?;

        // （例）音素データ取得
        let phonemes = cevio.phonemes("はじめまして")?;
        println!("{phonemes:?}");

        // 【CeVIO AI】終了
        cevio.close(CloseMode::Interactive)?;

        Ok(())
    }

    #[test]
    #[serial]
    fn full() -> Result<()> {
        let cevio = Cevio::new()?;

        // 【CeVIO AI】起動
        cevio.start(false)?;

        // キャスト一覧
        let components = cevio.available_casts()?;

        for component in components {
            let start = Instant::now();
            println!("{component:?}");

            let cast = CastBuilder::default()
                // キャスト設定
                .cast(component)
                // （例）音量設定
                .volume(Volume::new(100).unwrap())
                // （例）抑揚設定
                .tone_scale(ToneScale::new(100).unwrap())
                .build()?;

            cevio.apply_cast(&cast)?;

            // 感情パラメータ取得
            let components = cevio.components()?;
            println!("{components:?}");

            // （例）再生
            let e = cevio.speak("こんにちは")?;
            e.wait_timeout(30.0)?;

            // （例）音素データ取得
            let phonemes = cevio.phonemes("はじめまして")?;
            println!("{phonemes:?}");

            println!("{}ms", start.elapsed().as_millis());
        }

        // 【CeVIO AI】終了
        cevio.close(CloseMode::Interactive)?;

        Ok(())
    }

    #[test]
    #[serial]
    fn test_builder_patterns() -> Result<()> {
        // CevioConfigのテスト
        let config = CevioConfigBuilder::default()
            .start_host(true)
            .no_wait(false)
            .initial_cast("さとうささら")
            .initial_volume(Volume::new(80).unwrap())
            .build()?;

        let _cevio = Cevio::with_config(config)?;

        // CastBuilderの新しいメソッドのテスト
        let _cast_defaults = CastBuilder::default()
            .with_defaults()
            .cast("さとうささら")
            .build()?;

        let _cast_preset = CastBuilder::default()
            .from_preset(VoicePreset::Energetic)
            .cast("さとうささら")
            .build()?;

        Ok(())
    }
}
