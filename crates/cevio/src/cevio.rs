//! `CeVIO` AI APIのメイン実装
//!
//! `このモジュールはCeVIO` AIとの通信を行うメインの構造体と機能を提供します。
//! COM（Component Object Model）を使用してCeVIO AIと安全に通信し、
//! 音声合成、パラメータ制御、キャスト管理などの機能を提供します。

use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use parking_lot::Mutex;

use derive_builder::Builder;
use windows::{
    core::BSTR,
    Win32::{
        Foundation::VARIANT_BOOL,
        System::Com::{CoCreateInstance, CLSCTX_INPROC_SERVER},
    },
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    com_manager::ComGuard,
    error::{CevioError, Result},
    parameter::{Alpha, Speed, Tone, ToneScale, VoicePreset, Volume},
};
use cevio_sys::{
    IServiceControl2V40, ISpeakingState2, ITalker2V40, ITalkerComponent2, ServiceControl2V40,
    Talker2V40,
};

/// `CeVIO` AI初期化設定
///
/// `CeVIO` AIの初期化時に使用する設定を定義します。
/// ビルダーパターンで構築可能です。
///
/// # Example
///
/// ```no_run
/// use cevio::{CevioConfigBuilder, Volume, Speed};
///
/// let config = CevioConfigBuilder::default()
///     .start_host(true)
///     .initial_cast("さとうささら")
///     .initial_volume(Volume::new(80).unwrap())
///     .initial_speed(Speed::new(60).unwrap())
///     .build()
///     .unwrap();
/// ```
#[derive(Default, Builder, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[builder(setter(into, strip_option), default)]
pub struct CevioConfig {
    /// `CeVIO` AIを自動起動するか
    ///
    /// `true`の場合、CeVIO AIが起動していなければ自動的に起動します。
    /// 既に起動済みの場合は何もしません。
    pub start_host: bool,

    /// `起動時に待機しない（start_hostがtrueの場合のみ有効`）
    ///
    /// - `true`: 起動のみ行い、アクセス可能になるのを待たずに戻ります
    /// - `false`: 起動後、外部からアクセス可能になるまで待機します
    pub no_wait: bool,

    /// 初期キャスト
    ///
    /// 初期化時に設定するキャスト名。
    /// 利用可能なキャストは `available_casts()` で取得できます。
    pub initial_cast: Option<String>,

    /// 初期音量（0～100）
    pub initial_volume: Option<Volume>,

    /// 初期速度（0～100）
    pub initial_speed: Option<Speed>,

    /// 初期トーン（音の高さ）（0～100）
    pub initial_tone: Option<Tone>,

    /// 初期トーンスケール（抑揚）（0～100）
    pub initial_tone_scale: Option<ToneScale>,

    /// 初期アルファ（声質）（0～100）
    pub initial_alpha: Option<Alpha>,

    /// COM操作のタイムアウト時間（デフォルト: 5秒）
    pub operation_timeout: Option<Duration>,
}

/// `CeVIO` AI終了モード
///
/// `CeVIO` AIに終了を要求する際の処理モードを指定します。
pub enum CloseMode {
    /// 編集中の場合、保存や終了キャンセルが可能
    ///
    /// `CeVIO` AIが編集中の場合、ユーザーに保存確認ダイアログが表示され、
    /// 保存するか終了をキャンセルできます。
    Interactive = 0,

    /// 強制的に終了
    ///
    /// 編集中の内容を破棄して強制的に終了します。
    Force = 1,
}

/// `StartHostの結果コード`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
enum HostStartResult {
    /// 成功（起動済みの場合も含む）
    Succeeded,
    /// インストール状態が不明
    NotRegistered,
    /// 実行ファイルが見つからない
    FileNotFound,
    /// プロセスの起動に失敗
    StartingFailed,
    /// アプリケーション起動後、エラーにより終了
    HostError,
}

impl HostStartResult {
    const fn from_i32(value: i32) -> Self {
        match value {
            0 => Self::Succeeded,
            1 => Self::NotRegistered,
            2 => Self::FileNotFound,
            3 => Self::StartingFailed,
            // CeVIO APIの実装を確認したところ、1-4の値が返される
            _ => Self::HostError,
        }
    }
}

/// `CeVIO` AI制御インターフェース
///
/// `CeVIO` AIと通信するためのメインインターフェースです。
/// トーク機能（音声合成）と制御機能を提供します。
///
/// # Thread Safety
///
/// 内部でArcとMutexを使用しているため、スレッド間で安全に共有できます。
///
/// # Example
///
/// ```no_run
/// use cevio::{Cevio, CastBuilder};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // CeVIO AIの初期化
/// let cevio = Cevio::new()?;
///
/// // CeVIO AIを起動
/// cevio.start(false)?;
///
/// // キャストを設定
/// let cast = CastBuilder::default()
///     .cast("さとうささら")
///     .with_defaults()
///     .build()?;
/// cevio.apply_cast(&cast)?;
///
/// // 音声合成
/// let state = cevio.speak("こんにちは")?;
/// state.wait()?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct Cevio {
    service: Arc<Mutex<IServiceControl2V40>>,
    talker: Arc<Mutex<ITalker2V40>>,
    _com_guard: Arc<ComGuard>,
}

impl Cevio {
    /// `CeVIO` AIインスタンスを作成します。
    ///
    /// `COM初期化とCeVIO` AIのCOMオブジェクトを作成します。
    /// `CeVIO` AIが起動していない場合でもインスタンスは作成されます。
    ///
    /// # Errors
    ///
    /// - COM初期化に失敗した場合
    /// - COMオブジェクトの作成に失敗した場合
    pub fn new() -> Result<Self> {
        let com_guard = ComGuard::new()?;

        unsafe {
            let service: IServiceControl2V40 =
                CoCreateInstance(&ServiceControl2V40, None, CLSCTX_INPROC_SERVER)?;
            let talker: ITalker2V40 = CoCreateInstance(&Talker2V40, None, CLSCTX_INPROC_SERVER)?;

            Ok(Self {
                service: Arc::new(Mutex::new(service)),
                talker: Arc::new(Mutex::new(talker)),
                _com_guard: Arc::new(com_guard),
            })
        }
    }

    /// `設定を指定してCeVIO` AIインスタンスを作成します。
    ///
    /// `指定された設定に基づいてCeVIO` AIを初期化します。
    ///
    /// # Arguments
    ///
    /// * `config` - 初期化設定
    ///
    /// # Errors
    ///
    /// - インスタンス作成に失敗した場合
    /// - `CeVIO` AI起動に失敗した場合（`start_host`が`true`の場合）
    /// - パラメータ設定に失敗した場合
    pub fn with_config(config: CevioConfig) -> Result<Self> {
        let cevio = Self::new()?;

        if config.start_host {
            cevio.start(config.no_wait)?;
        }

        if let Some(cast) = config.initial_cast {
            cevio.set_cast(&cast)?;
        }

        if let Some(volume) = config.initial_volume {
            cevio.set_volume(volume)?;
        }

        if let Some(speed) = config.initial_speed {
            cevio.set_speed(speed)?;
        }

        if let Some(tone) = config.initial_tone {
            cevio.set_tone(tone)?;
        }

        if let Some(tone_scale) = config.initial_tone_scale {
            cevio.set_tone_scale(tone_scale)?;
        }

        if let Some(alpha) = config.initial_alpha {
            cevio.set_alpha(alpha)?;
        }

        Ok(cevio)
    }

    /// `CeVIO` AIを起動します。
    ///
    /// 起動済みの場合は何もしません。
    ///
    /// # Arguments
    ///
    /// * `no_wait` - `true`の場合は起動のみ行い、アクセス可能になるのを待たずに戻ります。
    ///   `false`の場合は起動後、外部からアクセス可能になるまで待機します。
    ///
    /// # Errors
    ///
    /// - `CevioError::InstallUnknown` - インストール状態が不明
    /// - `CevioError::ExecutableNotFound` - 実行ファイルが見つからない
    /// - `CevioError::ProcessStartFailed` - プロセスの起動に失敗
    /// - `CevioError::AppTerminated` - アプリケーション起動後、エラーにより終了
    pub fn start(&self, no_wait: bool) -> Result<()> {
        let result = unsafe { self.service.lock().StartHost(VARIANT_BOOL::from(no_wait)) }?;
        match HostStartResult::from_i32(result) {
            HostStartResult::Succeeded => Ok(()),
            HostStartResult::NotRegistered => Err(CevioError::InstallUnknown),
            HostStartResult::FileNotFound => Err(CevioError::ExecutableNotFound),
            HostStartResult::StartingFailed => Err(CevioError::ProcessStartFailed),
            HostStartResult::HostError => Err(CevioError::AppTerminated),
        }
    }

    /// `CeVIO` AIに終了を要求します。
    ///
    /// # Arguments
    ///
    /// * `mode` - 終了モード
    pub fn close(&self, mode: CloseMode) -> Result<()> {
        unsafe { self.service.lock().CloseHost(mode as i32) }?;
        Ok(())
    }

    /// 現在の音量を取得します。
    ///
    /// # Returns
    ///
    /// 音の大きさ（0～100）
    pub fn volume(&self) -> Result<Volume> {
        let value = unsafe { self.talker.lock().Volume() }? as u8;
        Ok(Volume::new(value).expect("CeVIO returned invalid volume"))
    }

    /// 現在の話す速さを取得します。
    ///
    /// # Returns
    ///
    /// 話す速さ（0～100）
    pub fn speed(&self) -> Result<Speed> {
        let value = unsafe { self.talker.lock().Speed() }? as u8;
        Ok(Speed::new(value).expect("CeVIO returned invalid speed"))
    }

    /// 現在の音の高さを取得します。
    ///
    /// # Returns
    ///
    /// 音の高さ（0～100）
    pub fn tone(&self) -> Result<Tone> {
        let value = unsafe { self.talker.lock().Tone() }? as u8;
        Ok(Tone::new(value).expect("CeVIO returned invalid tone"))
    }

    /// 現在の抑揚を取得します。
    ///
    /// # Returns
    ///
    /// 抑揚（0～100）
    pub fn tone_scale(&self) -> Result<ToneScale> {
        let value = unsafe { self.talker.lock().ToneScale() }? as u8;
        Ok(ToneScale::new(value).expect("CeVIO returned invalid tone scale"))
    }

    /// 現在の声質を取得します。
    ///
    /// # Returns
    ///
    /// 声質（0～100）
    pub fn alpha(&self) -> Result<Alpha> {
        let value = unsafe { self.talker.lock().Alpha() }? as u8;
        Ok(Alpha::new(value).expect("CeVIO returned invalid alpha"))
    }

    /// 音量を設定します。
    ///
    /// # Arguments
    ///
    /// * `volume` - 音の大きさ（0～100）
    fn set_volume(&self, volume: Volume) -> Result<()> {
        Ok(unsafe { self.talker.lock().SetVolume(u32::from(volume.get())) }?)
    }

    /// 話す速さを設定します。
    ///
    /// # Arguments
    ///
    /// * `speed` - 話す速さ（0～100）
    fn set_speed(&self, speed: Speed) -> Result<()> {
        Ok(unsafe { self.talker.lock().SetSpeed(u32::from(speed.get())) }?)
    }

    /// 音の高さを設定します。
    ///
    /// # Arguments
    ///
    /// * `tone` - 音の高さ（0～100）
    fn set_tone(&self, tone: Tone) -> Result<()> {
        Ok(unsafe { self.talker.lock().SetTone(u32::from(tone.get())) }?)
    }

    /// 抑揚を設定します。
    ///
    /// # Arguments
    ///
    /// * `tone_scale` - 抑揚（0～100）
    fn set_tone_scale(&self, tone_scale: ToneScale) -> Result<()> {
        Ok(unsafe { self.talker.lock().SetToneScale(u32::from(tone_scale.get())) }?)
    }

    /// 声質を設定します。
    ///
    /// # Arguments
    ///
    /// * `alpha` - 声質（0～100）
    fn set_alpha(&self, alpha: Alpha) -> Result<()> {
        Ok(unsafe { self.talker.lock().SetAlpha(u32::from(alpha.get())) }?)
    }

    /// 現在のキャストの感情パラメータマップを取得します。
    ///
    /// 内容はキャストによって変化します。
    ///
    /// # Returns
    ///
    /// 感情パラメータのリスト
    ///
    /// # Example
    ///
    /// - 『さとうささら』→ "普通", "元気", "怒り", "哀しみ"
    pub fn components(&self) -> Result<Vec<Component>> {
        let talker_components = unsafe { self.talker.lock().Components() }?;

        let len = unsafe { talker_components.Length() }?;
        let mut components = Vec::with_capacity(len as usize);

        for i in 0..len {
            let component = unsafe {
                let talker_component = talker_components.At(i)?;
                let id = talker_component.Id()?.to_string();
                let name = talker_component.Name()?.to_string();
                Component::new(talker_component, &id, &name)
            };
            components.push(component);
        }

        Ok(components)
    }

    /// 現在のキャストを取得します。
    ///
    /// # Returns
    ///
    /// 現在設定されているキャスト名
    pub fn cast(&self) -> Result<String> {
        Ok(unsafe { self.talker.lock().Cast() }?.to_string())
    }

    /// キャストを設定します。
    ///
    /// # Arguments
    ///
    /// * `cast` - キャスト名
    fn set_cast(&self, cast: &str) -> Result<()> {
        Ok(unsafe { self.talker.lock().SetCast(&BSTR::from(cast)) }?)
    }

    /// 利用可能なキャスト名を取得します。
    ///
    /// キャストの取り揃えは、インストールされている音源によります。
    ///
    /// # Returns
    ///
    /// 利用可能なキャスト名のリスト
    pub fn available_casts(&self) -> Result<Vec<String>> {
        let strings = unsafe { self.talker.lock().AvailableCasts() }?;

        let len = unsafe { strings.Length() }?;
        let mut casts = Vec::with_capacity(len as usize);

        for i in 0..len {
            casts.push(unsafe { strings.At(i) }?.to_string());
        }

        Ok(casts)
    }

    /// 指定したセリフの再生を開始します。
    ///
    /// 再生終了を待たずに処理が戻ります。
    /// 再生終了を待つには戻り値の `SpeakingState::wait()` を呼び出します。
    ///
    /// # Arguments
    ///
    /// * `text` - セリフ
    ///
    /// # Returns
    ///
    /// 再生状態を表すオブジェクト
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use cevio::Cevio;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let cevio = Cevio::new()?;
    /// let state = cevio.speak("こんにちは")?;
    /// state.wait()?; // 再生終了まで待機
    /// # Ok(())
    /// # }
    /// ```
    pub fn speak(&self, text: &str) -> Result<SpeakingState> {
        let speak_state = unsafe { self.talker.lock().Speak(&BSTR::from(text)) }?;
        Ok(SpeakingState::new(speak_state))
    }

    /// 再生を停止します。
    ///
    /// # Returns
    ///
    /// 成功した場合は`true`、それ以外の場合は`false`
    pub fn stop(&self) -> Result<bool> {
        Ok(unsafe { self.talker.lock().Stop() }?.as_bool())
    }

    /// 指定したセリフの長さを取得します。
    ///
    /// # Arguments
    ///
    /// * `text` - セリフ
    ///
    /// # Returns
    ///
    /// 長さ（単位は秒）
    pub fn text_duration(&self, text: &str) -> Result<f64> {
        Ok(unsafe { self.talker.lock().GetTextDuration(&BSTR::from(text)) }?)
    }

    /// 指定したセリフの音素単位のデータを取得します。
    ///
    /// リップシンク等に利用できます。
    ///
    /// # Arguments
    ///
    /// * `text` - セリフ
    ///
    /// # Returns
    ///
    /// 音素単位のデータのリスト
    pub fn phonemes(&self, text: &str) -> Result<Vec<PhonemeData>> {
        let phoneme_datas = unsafe { self.talker.lock().GetPhonemes(&BSTR::from(text)) }?;

        let len = unsafe { phoneme_datas.Length() }?;
        let mut phonemes = Vec::with_capacity(len as usize);

        for i in 0..len {
            unsafe {
                let data = phoneme_datas.At(i)?;
                let phoneme = data.Phoneme()?.to_string();
                let start_time = data.StartTime()?;
                let end_time = data.EndTime()?;
                phonemes.push(PhonemeData::new(phoneme, start_time, end_time));
            }
        }

        Ok(phonemes)
    }

    /// 指定したセリフをWAVファイルとして出力します。
    ///
    /// 出力形式：
    /// - サンプリングレート: 48kHz
    /// - ビットレート: 16bit
    /// - チャンネル: モノラル
    ///
    /// # Arguments
    ///
    /// * `text` - セリフ
    /// * `path` - 出力先パス
    ///
    /// # Returns
    ///
    /// 成功した場合は`true`、それ以外の場合は`false`
    pub fn output_wave_to_file<P: AsRef<Path>>(&self, text: &str, path: P) -> Result<bool> {
        let path_str = path.as_ref().to_string_lossy();
        Ok(unsafe {
            self.talker
                .lock()
                .OutputWaveToFile(&BSTR::from(text), &BSTR::from(path_str.as_ref()))
        }?
        .as_bool())
    }

    /// キャスト設定を一括で適用します。
    ///
    /// # Arguments
    ///
    /// * `cast` - 適用するキャスト設定
    pub fn apply_cast(&self, cast: &Cast) -> Result<()> {
        if let Some(ref cast) = cast.cast {
            self.set_cast(cast)?;
        }

        if let Some(volume) = cast.volume {
            self.set_volume(volume)?;
        }

        if let Some(speed) = cast.speed {
            self.set_speed(speed)?;
        }

        if let Some(tone) = cast.tone {
            self.set_tone(tone)?;
        }

        if let Some(tone_scale) = cast.tone_scale {
            self.set_tone_scale(tone_scale)?;
        }

        if let Some(alpha) = cast.alpha {
            self.set_alpha(alpha)?;
        }

        Ok(())
    }
}

/// キャスト設定
///
/// キャストと音声パラメータをまとめて管理する構造体です。
/// ビルダーパターンで構築可能です。
///
/// # Example
///
/// ```no_run
/// use cevio::{CastBuilder, Volume, Speed, VoicePreset};
///
/// // 個別にパラメータを設定
/// let cast = CastBuilder::default()
///     .cast("さとうささら")
///     .volume(Volume::new(80).unwrap())
///     .speed(Speed::new(60).unwrap())
///     .build()
///     .unwrap();
///
/// // デフォルト値（すべて50）で設定
/// let cast = CastBuilder::default()
///     .cast("さとうささら")
///     .with_defaults()
///     .build()
///     .unwrap();
///
/// // プリセットから設定
/// let cast = CastBuilder::default()
///     .cast("さとうささら")
///     .from_preset(VoicePreset::Happy)
///     .build()
///     .unwrap();
/// ```
#[derive(Default, Builder, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[builder(setter(into, strip_option), default)]
pub struct Cast {
    /// キャスト名
    pub cast: Option<String>,

    /// 音の大きさ（0～100）
    pub volume: Option<Volume>,

    /// 話す速さ（0～100）
    pub speed: Option<Speed>,

    /// 音の高さ（0～100）
    pub tone: Option<Tone>,

    /// 抑揚（0～100）
    pub tone_scale: Option<ToneScale>,

    /// 声質（0～100）
    pub alpha: Option<Alpha>,
}

impl CastBuilder {
    /// すべてのパラメータをデフォルト値（50）に設定
    pub fn with_defaults(&mut self) -> &mut Self {
        self.volume(Volume::new(50).unwrap())
            .speed(Speed::new(50).unwrap())
            .tone(Tone::new(50).unwrap())
            .tone_scale(ToneScale::new(50).unwrap())
            .alpha(Alpha::new(50).unwrap())
    }

    /// プリセットから設定を適用
    pub fn from_preset(&mut self, preset: VoicePreset) -> &mut Self {
        let (volume, speed, tone, tone_scale, alpha) = preset.to_parameters();
        self.volume(volume)
            .speed(speed)
            .tone(tone)
            .tone_scale(tone_scale)
            .alpha(alpha)
    }
}

/// 感情パラメータ
///
/// キャストの感情を制御するパラメータです。
/// 各キャストで利用可能な感情は異なります。
#[derive(Debug, Clone)]
pub struct Component {
    inner: ITalkerComponent2,

    /// 識別子
    pub id: String,

    /// 感情の名前（例："普通", "元気", "怒り", "哀しみ"）
    pub name: String,
}

impl Component {
    fn new(component: ITalkerComponent2, id: &str, name: &str) -> Self {
        Self {
            inner: component,
            id: id.to_string(),
            name: name.to_string(),
        }
    }

    /// 感情の値を取得します。
    ///
    /// # Returns
    ///
    /// 感情の値（0～100）
    pub fn value(&self) -> Result<u8> {
        Ok(unsafe { self.inner.Value() }? as u8)
    }

    /// 感情の値を設定します。
    ///
    /// # Arguments
    ///
    /// * `value` - 感情の値（0～100）
    ///
    /// # Errors
    ///
    /// 値が100を超える場合は `CevioError::InvalidParameter` を返します。
    pub fn set_value(&self, value: u8) -> Result<()> {
        if value > 100 {
            return Err(CevioError::InvalidParameter(format!(
                "Component value must be 0-100, got {value}"
            )));
        }
        Ok(unsafe { self.inner.SetValue(u32::from(value)) }?)
    }
}

/// 再生状態
///
/// 音声の再生状態を管理し、再生の完了を待機できます。
pub struct SpeakingState {
    state: ISpeakingState2,
}

impl SpeakingState {
    const fn new(state: ISpeakingState2) -> Self {
        Self { state }
    }

    /// 再生が完了したかどうかを取得します。
    ///
    /// # Returns
    ///
    /// 完了した場合は`true`（失敗を含む）、それ以外の場合は`false`
    pub fn is_completed(&self) -> Result<bool> {
        Ok(unsafe { self.state.IsCompleted() }?.as_bool())
    }

    /// 再生が成功したかどうかを取得します。
    ///
    /// # Returns
    ///
    /// 成功した場合は`true`、それ以外の場合は`false`
    pub fn is_succeeded(&self) -> Result<bool> {
        Ok(unsafe { self.state.IsSucceeded() }?.as_bool())
    }

    /// 再生終了を待ちます。
    ///
    /// 再生が完了するまでブロックします。
    pub fn wait(&self) -> Result<()> {
        Ok(unsafe { self.state.Wait() }?)
    }

    /// 再生終了を待ちます（タイムアウト付き）。
    ///
    /// # Arguments
    ///
    /// * `seconds` - 最大待機時間（秒）。0未満は無制限。
    pub fn wait_timeout(&self, seconds: f64) -> Result<()> {
        Ok(unsafe { self.state.Wait_2(seconds) }?)
    }
}

/// 音素データ
///
/// 音声の音素単位のタイミング情報を表します。
/// リップシンクなどの同期処理に利用できます。
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhonemeData {
    phoneme: String,
    start_time: f64,
    end_time: f64,
}

impl PhonemeData {
    /// 新しい`PhonemeData`を作成します。
    pub(crate) const fn new(phoneme: String, start_time: f64, end_time: f64) -> Self {
        Self {
            phoneme,
            start_time,
            end_time,
        }
    }

    /// 音素を取得します。
    #[must_use]
    pub fn phoneme(&self) -> &str {
        &self.phoneme
    }

    /// 開始時間（秒）を取得します。
    #[must_use]
    pub const fn start_time(&self) -> f64 {
        self.start_time
    }

    /// 終了時間（秒）を取得します。
    #[must_use]
    pub const fn end_time(&self) -> f64 {
        self.end_time
    }
}
