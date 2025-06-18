//! エラー処理関連の型定義
//!
//! このモジュールは、CeVIO APIで発生する可能性のあるエラーを定義します。
//! すべてのエラーは`CevioError`列挙型にまとめられており、
//! `thiserror`クレートを使用して詳細なエラーメッセージを提供します。

use crate::{CastBuilderError, CevioConfigBuilderError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CevioError {
    #[error("Windows API error: {0}")]
    Windows(#[from] windows::core::Error),
    #[error("BuilderError error: {0}")]
    BuilderError(#[from] CastBuilderError),
    #[error("ConfigBuilderError error: {0}")]
    ConfigBuilderError(#[from] CevioConfigBuilderError),
    #[error("Installation state is unknown")]
    InstallUnknown,
    #[error("Executable not found")]
    ExecutableNotFound,
    #[error("Failed to start process")]
    ProcessStartFailed,
    #[error("Application terminated due to error after startup")]
    AppTerminated,
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),
}

pub type Result<T> = std::result::Result<T, CevioError>;
