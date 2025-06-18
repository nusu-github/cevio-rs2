//! CeVIO AI の低レベル FFI バインディング
//!
//! このクレートは、日本語音声合成ソフトウェア CeVIO AI 用の生の Windows COM インターフェースバインディングを提供します。
//! これらのバインディングは CeVIO.Talk.RemoteService2.winmd メタデータファイルから自動生成されます。
//!
//! # プラットフォームサポート
//!
//! このクレートは Windows COM インターフェースに依存するため、**Windows 専用**です。
//!
//! # 使用方法
//!
//! このクレートは生の COM インターフェースを公開します。より高レベルで安全な API については、代わりに `cevio-ai` クレートの使用を検討してください。
//!
//! # 安全性について
//!
//! このクレートが公開する COM インターフェースは本質的に unsafe です。`unsafe_send` フィーチャが有効な場合、
//! 特定のインターフェースが `Mutex` でラップされることを前提として `Send` が実装されます。

mod bindings {
    #![allow(
        non_snake_case,
        non_upper_case_globals,
        non_camel_case_types,
        dead_code,
        clippy::all
    )]
    
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::CeVIO::Talk::RemoteService2::{
    IPhonemeData2, IPhonemeDataArray2, IServiceControl2, IServiceControl2V40,
    IServiceControl2V40Part, ISpeakingState2, IStringArray2, ITalker2, ITalker2V40,
    ITalker2V40Part, ITalkerComponent2, ITalkerComponentArray2, PhonemeData2, PhonemeDataArray2,
    ServiceControl2, ServiceControl2V40, SpeakingState2, StringArray2, Talker2, Talker2V40,
    TalkerComponent2, TalkerComponentCollection2,
};

// ITalker2V40及びIServiceControl2V40がMutexでラップされることを前提としてSendを実装する
#[cfg(feature = "unsafe_send")]
use std::marker::Send;

#[cfg(feature = "unsafe_send")]
unsafe impl Send for IServiceControl2V40 {}

#[cfg(feature = "unsafe_send")]
unsafe impl Send for ITalker2V40 {}
