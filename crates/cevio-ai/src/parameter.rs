//! 音声パラメータ関連の型定義
//!
//! このモジュールは、CeVIO AIの音声パラメータを型安全に扱うための型を提供します。
//! すべてのパラメータは0-100の範囲に制限されており、コンパイル時に範囲チェックが行われます。
//!
//! ## 使用例
//!
//! ```rust
//! use cevio_ai::*;
//!
//! // 型安全なパラメータ作成
//! let volume = Volume::new(80).unwrap();
//! let speed = Speed::new(60).unwrap();
//!
//! // プリセットの使用
//! let (vol, spd, tone, tone_scale, alpha) = VoicePreset::Energetic.to_parameters();
//! ```

use bounded_integer::bounded_integer;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

bounded_integer! {
    /// 音の大きさ（0～100）
    pub struct Volume { 0..=100 }
}

bounded_integer! {
    /// 話す速さ（0～100）
    pub struct Speed { 0..=100 }
}

bounded_integer! {
    /// 音の高さ（0～100）
    pub struct Tone { 0..=100 }
}

bounded_integer! {
    /// 抑揚（0～100）
    pub struct ToneScale { 0..=100 }
}

bounded_integer! {
    /// 声質（0～100）
    pub struct Alpha { 0..=100 }
}

/// 音声パラメータのプリセット
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum VoicePreset {
    /// 標準設定（すべて50）
    Normal,
    /// 速い話し方（速度75）
    Fast,
    /// ゆっくりな話し方（速度25）
    Slow,
    /// 高い声（音程75）
    HighPitch,
    /// 低い声（音程25）
    LowPitch,
    /// 元気な声（音量70、速度60、音程60）
    Energetic,
    /// 落ち着いた声（音量40、速度40、音程40）
    Calm,
}

impl VoicePreset {
    /// プリセットから各パラメータ値を取得
    #[must_use]
    pub const fn to_parameters(self) -> (Volume, Speed, Tone, ToneScale, Alpha) {
        match self {
            Self::Normal => (
                Volume::new(50).unwrap(),
                Speed::new(50).unwrap(),
                Tone::new(50).unwrap(),
                ToneScale::new(50).unwrap(),
                Alpha::new(50).unwrap(),
            ),
            Self::Fast => (
                Volume::new(50).unwrap(),
                Speed::new(75).unwrap(),
                Tone::new(50).unwrap(),
                ToneScale::new(50).unwrap(),
                Alpha::new(50).unwrap(),
            ),
            Self::Slow => (
                Volume::new(50).unwrap(),
                Speed::new(25).unwrap(),
                Tone::new(50).unwrap(),
                ToneScale::new(50).unwrap(),
                Alpha::new(50).unwrap(),
            ),
            Self::HighPitch => (
                Volume::new(50).unwrap(),
                Speed::new(50).unwrap(),
                Tone::new(75).unwrap(),
                ToneScale::new(50).unwrap(),
                Alpha::new(50).unwrap(),
            ),
            Self::LowPitch => (
                Volume::new(50).unwrap(),
                Speed::new(50).unwrap(),
                Tone::new(25).unwrap(),
                ToneScale::new(50).unwrap(),
                Alpha::new(50).unwrap(),
            ),
            Self::Energetic => (
                Volume::new(70).unwrap(),
                Speed::new(60).unwrap(),
                Tone::new(60).unwrap(),
                ToneScale::new(60).unwrap(),
                Alpha::new(50).unwrap(),
            ),
            Self::Calm => (
                Volume::new(40).unwrap(),
                Speed::new(40).unwrap(),
                Tone::new(40).unwrap(),
                ToneScale::new(40).unwrap(),
                Alpha::new(50).unwrap(),
            ),
        }
    }
}
