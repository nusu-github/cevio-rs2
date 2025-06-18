//! `CeVIO` AI APIのメイン実装
//!
//! `このモジュールはCeVIO` AIとの通信を行うメインの構造体と機能を提供します。
//! COM（Component Object Model）を使用してCeVIO AIと安全に通信し、
//! 音声合成、パラメータ制御、キャスト管理などの機能を提供します。

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

use crate::{
    com_manager::ComGuard,
    error::{CevioError, Result},
    parameter::{Alpha, Speed, Tone, ToneScale, VoicePreset, Volume},
};
use cevio_sys::{
    IServiceControl2V40, ISpeakingState2, ITalker2V40, ITalkerComponent2, ServiceControl2V40,
    Talker2V40,
};

#[derive(Default, Builder, Debug)]
#[builder(setter(into, strip_option), default)]
pub struct CevioConfig {
    #[doc = "CeVIO AIを自動起動するか"]
    pub start_host: bool,

    #[doc = "起動時に待機しない（start_hostがtrueの場合のみ有効）"]
    pub no_wait: bool,

    #[doc = "初期キャスト"]
    pub initial_cast: Option<String>,

    #[doc = "初期音量"]
    pub initial_volume: Option<Volume>,

    #[doc = "初期速度"]
    pub initial_speed: Option<Speed>,

    #[doc = "初期トーン"]
    pub initial_tone: Option<Tone>,

    #[doc = "初期トーンスケール"]
    pub initial_tone_scale: Option<ToneScale>,

    #[doc = "初期アルファ"]
    pub initial_alpha: Option<Alpha>,

    #[doc = "COM操作のタイムアウト時間（デフォルト: 5秒）"]
    pub operation_timeout: Option<Duration>,
}

pub enum CloseMode {
    #[doc = "編集中の場合、保存や終了キャンセルが可能"]
    Interactive = 0,

    #[doc = "強制的に終了"]
    Force = 1,
}

/// `StartHostの結果コード`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Clone)]
pub struct Cevio {
    service: Arc<Mutex<IServiceControl2V40>>,
    talker: Arc<Mutex<ITalker2V40>>,
    _com_guard: Arc<ComGuard>,
}

impl Cevio {
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

    pub fn close(&self, mode: CloseMode) -> Result<()> {
        unsafe { self.service.lock().CloseHost(mode as i32) }?;
        Ok(())
    }

    pub fn volume(&self) -> Result<Volume> {
        let value = unsafe { self.talker.lock().Volume() }? as u8;
        Ok(Volume::new(value).expect("CeVIO returned invalid volume"))
    }

    pub fn speed(&self) -> Result<Speed> {
        let value = unsafe { self.talker.lock().Speed() }? as u8;
        Ok(Speed::new(value).expect("CeVIO returned invalid speed"))
    }

    pub fn tone(&self) -> Result<Tone> {
        let value = unsafe { self.talker.lock().Tone() }? as u8;
        Ok(Tone::new(value).expect("CeVIO returned invalid tone"))
    }

    pub fn tone_scale(&self) -> Result<ToneScale> {
        let value = unsafe { self.talker.lock().ToneScale() }? as u8;
        Ok(ToneScale::new(value).expect("CeVIO returned invalid tone scale"))
    }

    pub fn alpha(&self) -> Result<Alpha> {
        let value = unsafe { self.talker.lock().Alpha() }? as u8;
        Ok(Alpha::new(value).expect("CeVIO returned invalid alpha"))
    }

    fn set_volume(&self, volume: Volume) -> Result<()> {
        Ok(unsafe { self.talker.lock().SetVolume(u32::from(volume.get())) }?)
    }

    fn set_speed(&self, speed: Speed) -> Result<()> {
        Ok(unsafe { self.talker.lock().SetSpeed(u32::from(speed.get())) }?)
    }

    fn set_tone(&self, tone: Tone) -> Result<()> {
        Ok(unsafe { self.talker.lock().SetTone(u32::from(tone.get())) }?)
    }

    fn set_tone_scale(&self, tone_scale: ToneScale) -> Result<()> {
        Ok(unsafe { self.talker.lock().SetToneScale(u32::from(tone_scale.get())) }?)
    }

    fn set_alpha(&self, alpha: Alpha) -> Result<()> {
        Ok(unsafe { self.talker.lock().SetAlpha(u32::from(alpha.get())) }?)
    }

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

    pub fn cast(&self) -> Result<String> {
        Ok(unsafe { self.talker.lock().Cast() }?.to_string())
    }

    fn set_cast(&self, cast: &str) -> Result<()> {
        Ok(unsafe { self.talker.lock().SetCast(&BSTR::from(cast)) }?)
    }

    pub fn available_casts(&self) -> Result<Vec<String>> {
        let strings = unsafe { self.talker.lock().AvailableCasts() }?;

        let len = unsafe { strings.Length() }?;
        let mut casts = Vec::with_capacity(len as usize);

        for i in 0..len {
            casts.push(unsafe { strings.At(i) }?.to_string());
        }

        Ok(casts)
    }

    pub fn speak(&self, text: &str) -> Result<SpeakingState> {
        let speak_state = unsafe { self.talker.lock().Speak(&BSTR::from(text)) }?;
        Ok(SpeakingState::new(speak_state))
    }

    pub fn stop(&self) -> Result<bool> {
        Ok(unsafe { self.talker.lock().Stop() }?.as_bool())
    }

    pub fn text_duration(&self, text: &str) -> Result<f64> {
        Ok(unsafe { self.talker.lock().GetTextDuration(&BSTR::from(text)) }?)
    }

    pub fn phonemes(&self, text: &str) -> Result<Vec<PhonemeData>> {
        let phoneme_datas = unsafe { self.talker.lock().GetPhonemes(&BSTR::from(text)) }?;

        let len = unsafe { phoneme_datas.Length() }?;
        let mut phonemes = Vec::with_capacity(len as usize);

        for i in 0..len {
            unsafe {
                let data = phoneme_datas.At(i)?;
                phonemes.push(PhonemeData {
                    phoneme: data.Phoneme()?.to_string(),
                    start_time: data.StartTime()?,
                    end_time: data.EndTime()?,
                });
            }
        }

        Ok(phonemes)
    }

    pub fn output_wave_to_file(&self, text: &str, path: &str) -> Result<bool> {
        Ok(unsafe {
            self.talker
                .lock()
                .OutputWaveToFile(&BSTR::from(text), &BSTR::from(path))
        }?
        .as_bool())
    }

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

#[derive(Default, Builder, Debug, Clone)]
#[builder(setter(into, strip_option), default)]
pub struct Cast {
    #[doc = "キャスト名"]
    pub cast: Option<String>,

    #[doc = "音の大きさ（0～100）"]
    pub volume: Option<Volume>,

    #[doc = "話す速さ（0～100）"]
    pub speed: Option<Speed>,

    #[doc = "音の高さ（0～100）"]
    pub tone: Option<Tone>,

    #[doc = "抑揚（0～100）"]
    pub tone_scale: Option<ToneScale>,

    #[doc = "声質（0～100）"]
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

#[derive(Debug)]
pub struct Component {
    inner: ITalkerComponent2,

    #[doc = "識別子を取得します"]
    pub id: String,

    #[doc = "感情の名前を取得します"]
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

    /// 感情の値（0～100）を取得します。
    ///
    pub fn value(&self) -> Result<u8> {
        Ok(unsafe { self.inner.Value() }? as u8)
    }

    /// 感情の値（0～100）を設定します。
    ///
    pub fn set_value(&self, value: u8) -> Result<()> {
        if value > 100 {
            return Err(CevioError::InvalidParameter(format!(
                "Component value must be 0-100, got {value}"
            )));
        }
        Ok(unsafe { self.inner.SetValue(u32::from(value)) }?)
    }
}

pub struct SpeakingState {
    state: ISpeakingState2,
}

impl SpeakingState {
    const fn new(state: ISpeakingState2) -> Self {
        Self { state }
    }

    /// 再生が完了したかどうかを取得します。
    ///
    pub fn is_completed(&self) -> Result<bool> {
        Ok(unsafe { self.state.IsCompleted() }?.as_bool())
    }

    /// 再生が成功したかどうかを取得します。
    ///
    pub fn is_succeeded(&self) -> Result<bool> {
        Ok(unsafe { self.state.IsSucceeded() }?.as_bool())
    }

    /// 再生終了を待ちます。
    ///
    pub fn wait(&self) -> Result<()> {
        Ok(unsafe { self.state.Wait() }?)
    }

    /// 再生終了を待ちます。
    ///
    /// # Arguments
    ///
    /// * `seconds` - 待ち時間（秒）。
    ///
    pub fn wait_timeout(&self, seconds: f64) -> Result<()> {
        Ok(unsafe { self.state.Wait_2(seconds) }?)
    }
}

#[derive(Debug)]
pub struct PhonemeData {
    #[doc = "音素"]
    pub phoneme: String,

    #[doc = "開始時間 (秒)"]
    pub start_time: f64,

    #[doc = "終了時間 (秒)"]
    pub end_time: f64,
}
