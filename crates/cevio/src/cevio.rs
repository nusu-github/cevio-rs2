use std::sync::Arc;

use anyhow::Result;
use derive_builder::Builder;
use windows::{
    core::BSTR,
    Win32::{
        Foundation::VARIANT_BOOL,
        System::Com::{
            CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_INPROC_SERVER,
            COINIT_MULTITHREADED,
        },
    },
};

use cevio_sys::{
    IServiceControl2V40, ISpeakingState2, ITalker2V40, ITalkerComponent2, ServiceControl2V40,
    Talker2V40,
};

pub enum CloseMode {
    #[doc = "編集中の場合、保存や終了キャンセルが可能"]
    Interactive = 0,

    #[doc = "強制的に終了"]
    Force = 1,
}

#[derive(Clone)]
pub struct Cevio {
    service: Arc<IServiceControl2V40>,
    talker: Arc<ITalker2V40>,
}

impl Drop for Cevio {
    fn drop(&mut self) {
        unsafe {
            CoUninitialize();
        }
    }
}

impl Cevio {
    pub fn new() -> Result<Self> {
        unsafe {
            CoInitializeEx(None, COINIT_MULTITHREADED).ok()?;

            let service: IServiceControl2V40 =
                CoCreateInstance(&ServiceControl2V40, None, CLSCTX_INPROC_SERVER)?;
            let talker: ITalker2V40 = CoCreateInstance(&Talker2V40, None, CLSCTX_INPROC_SERVER)?;

            Ok(Self {
                service: Arc::new(service),
                talker: Arc::new(talker),
            })
        }
    }

    /// CeVIO AIを起動します。起動済みなら何もしません。
    ///
    /// # Arguments
    ///
    /// * `no_wait`: trueは起動のみ行います。アクセス可能かどうかはIsHostStartedで確認します。
    /// falseは起動後に外部からアクセス可能になるまで制御を戻しません。
    ///
    /// # Examples
    ///
    /// ```
    /// use cevio::Cevio;
    ///
    /// let cevio = Cevio::new().unwrap();
    /// cevio.start(false).unwrap();
    /// ```
    pub fn start(&self, no_wait: bool) -> Result<()> {
        let result = unsafe { self.service.StartHost(VARIANT_BOOL::from(no_wait)) }?;
        match result {
            0 => Ok(()),
            1 => anyhow::bail!("インストール状態が不明。"),
            2 => anyhow::bail!("実行ファイルが見つからない。"),
            3 => anyhow::bail!("プロセスの起動に失敗しました。"),
            4 => anyhow::bail!("アプリケーション起動後、エラーにより終了しました。"),
            _ => unreachable!(),
        }
    }

    /// CeVIO AIに終了を要求します。
    ///
    /// # Arguments
    ///
    /// * `mode`: 処理モード。
    ///
    /// # Examples
    ///
    /// ```
    /// use cevio::Cevio;
    /// use cevio::CloseMode;
    ///
    /// let cevio = Cevio::new().unwrap();
    /// cevio.close(CloseMode::Interactive).unwrap();
    /// ```
    pub fn close(&self, mode: CloseMode) -> Result<()> {
        Ok(unsafe { self.service.CloseHost(mode as i32) }?)
    }

    /// 音の大きさ（0～100）を取得します。
    ///
    pub fn volume(&self) -> Result<u8> {
        Ok(unsafe { self.talker.Volume() }? as u8)
    }

    /// 話す速さ（0～100）を取得します。
    ///
    pub fn speed(&self) -> Result<u8> {
        Ok(unsafe { self.talker.Speed() }? as u8)
    }

    /// 音の高さ（0～100）を取得します。
    ///
    pub fn tone(&self) -> Result<u8> {
        Ok(unsafe { self.talker.Tone() }? as u8)
    }

    /// 抑揚（0～100）を取得します。
    ///
    pub fn tone_scale(&self) -> Result<u8> {
        Ok(unsafe { self.talker.ToneScale() }? as u8)
    }

    /// 声質（0～100）を取得します。
    ///
    pub fn alpha(&self) -> Result<u8> {
        Ok(unsafe { self.talker.Alpha() }? as u8)
    }

    fn set_volume(&self, volume: u8) -> Result<()> {
        Ok(unsafe { self.talker.SetVolume(volume as u32) }?)
    }

    fn set_speed(&self, speed: u8) -> Result<()> {
        Ok(unsafe { self.talker.SetSpeed(speed as u32) }?)
    }

    fn set_tone(&self, tone: u8) -> Result<()> {
        Ok(unsafe { self.talker.SetTone(tone as u32) }?)
    }

    fn set_tone_scale(&self, tone_scale: u8) -> Result<()> {
        Ok(unsafe { self.talker.SetToneScale(tone_scale as u32) }?)
    }

    fn set_alpha(&self, alpha: u8) -> Result<()> {
        Ok(unsafe { self.talker.SetAlpha(alpha as u32) }?)
    }

    /// 現在のキャストの感情パラメータマップを取得します。
    ///
    pub fn components(&self) -> Result<Vec<Component>> {
        let talker_components = unsafe { self.talker.Components() }?;

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

    /// キャストを取得します。
    ///
    pub fn cast(&self) -> Result<String> {
        Ok(unsafe { self.talker.Cast() }?.to_string())
    }

    fn set_cast(&self, cast: &str) -> Result<()> {
        Ok(unsafe { self.talker.SetCast(&BSTR::from(cast)) }?)
    }

    /// 利用可能なキャスト名一覧を取得します。
    ///
    pub fn available_casts(&self) -> Result<Vec<String>> {
        let strings = unsafe { self.talker.AvailableCasts() }?;

        let len = unsafe { strings.Length() }?;
        let mut casts = Vec::with_capacity(len as usize);

        for i in 0..len {
            casts.push(unsafe { strings.At(i) }?.to_string());
        }

        Ok(casts)
    }

    /// 指定したセリフの再生を開始します。
    ///
    /// # Arguments
    ///
    /// * `text` - セリフ。
    ///
    /// ## 備考
    /// - 再生終了を待たずに処理が戻ります。
    /// - 再生終了を待つには戻り値のwaitを呼び出します。
    ///
    pub fn speak(&self, text: &str) -> Result<SpeakingState> {
        let speak_state = unsafe { self.talker.Speak(&BSTR::from(text)) }?;
        Ok(SpeakingState::new(speak_state))
    }

    /// 再生を停止します。
    ///
    pub fn stop(&self) -> Result<bool> {
        Ok(unsafe { self.talker.Stop() }?.as_bool())
    }

    /// 指定したセリフの長さを取得します。
    ///
    /// # Arguments
    ///
    /// * `text` - セリフ。
    ///
    pub fn text_duration(&self, text: &str) -> Result<f64> {
        Ok(unsafe { self.talker.GetTextDuration(&BSTR::from(text)) }?)
    }

    /// 指定したセリフの音素単位のデータを取得します。
    ///
    /// # Arguments
    ///
    /// * `text` - セリフ。
    ///
    /// ## 備考
    /// - リップシンク等に利用できます。
    ///
    pub fn phonemes(&self, text: &str) -> Result<Vec<PhonemeData>> {
        let phoneme_datas = unsafe { self.talker.GetPhonemes(&BSTR::from(text)) }?;

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

    /// 指定したセリフをWAVファイルとして出力します。
    ///
    /// # Arguments
    ///
    /// * `text` - セリフ。
    /// * `path` - 出力先ファイルのパス。
    ///
    /// ## 備考
    /// - 出力形式はサンプリングレート48kHz, ビットレート16bit, モノラルです。
    ///
    pub fn output_wave_to_file(&self, text: &str, path: &str) -> Result<bool> {
        Ok(unsafe {
            self.talker
                .OutputWaveToFile(&BSTR::from(text), &BSTR::from(path))
        }?
        .as_bool())
    }

    /// キャストを設定します。
    ///
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

#[derive(Default, Builder, Debug)]
#[builder(setter(into, strip_option), default)]
pub struct Cast {
    #[doc = "キャスト名"]
    pub cast: Option<String>,

    #[doc = "音の大きさ（0～100）"]
    pub volume: Option<u8>,

    #[doc = "話す速さ（0～100）"]
    pub speed: Option<u8>,

    #[doc = "音の高さ（0～100）"]
    pub tone: Option<u8>,

    #[doc = "抑揚（0～100）"]
    pub tone_scale: Option<u8>,

    #[doc = "声質（0～100）"]
    pub alpha: Option<u8>,
}

#[derive(Debug)]
pub struct Component {
    component: ITalkerComponent2,

    #[doc = "識別子を取得します"]
    pub id: String,

    #[doc = "感情の名前を取得します"]
    pub name: String,
}

impl Component {
    fn new(component: ITalkerComponent2, id: &str, name: &str) -> Self {
        Self {
            component,
            id: id.to_string(),
            name: name.to_string(),
        }
    }

    /// 感情の値（0～100）を取得します。
    ///
    pub fn value(&self) -> Result<u8> {
        Ok(unsafe { self.component.Value() }? as u8)
    }

    /// 感情の値（0～100）を設定します。
    ///
    pub fn set_value(&self, value: u8) -> Result<()> {
        Ok(unsafe { self.component.SetValue(value as u32) }?)
    }
}

pub struct SpeakingState {
    state: ISpeakingState2,
}

impl SpeakingState {
    fn new(state: ISpeakingState2) -> Self {
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
