use std::{ffi::c_void, mem::MaybeUninit};

use windows::{
    core::{
        define_interface, interface_hierarchy, IUnknown, Interface, Param, Result, Type, BSTR,
        HRESULT,
    },
    Win32::{
        Foundation::VARIANT_BOOL,
        System::Com::{IDispatch, IDispatch_Vtbl},
    },
};

use crate::{IPhonemeDataArray2, ISpeakingState2, ITalkerComponentArray2};

define_interface!(
    IStringArray2,
    IStringArray2_Vtbl,
    0x37F03F6E_9C51_4E32_BF31_761C6BC87AAC
);
impl core::ops::Deref for IStringArray2 {
    type Target = IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
interface_hierarchy!(IStringArray2, IDispatch);
impl IStringArray2 {
    pub unsafe fn Length(&self) -> Result<i32> {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).Length)(Interface::as_raw(self), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn At(&self, index: i32) -> Result<BSTR> {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).At)(Interface::as_raw(self), index, &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn Duplicate(&self) -> Result<IStringArray2> {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).Duplicate)(Interface::as_raw(self), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
}

#[repr(C)]
pub struct IStringArray2_Vtbl {
    pub base__: IDispatch_Vtbl,
    pub Length: unsafe extern "system" fn(*mut c_void, *mut i32) -> HRESULT,
    pub At: unsafe extern "system" fn(*mut c_void, i32, *mut MaybeUninit<BSTR>) -> HRESULT,
    pub Duplicate: unsafe extern "system" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
}

define_interface!(
    ITalker2,
    ITalker2_Vtbl,
    0xF5EED82F_A947_4B66_9411_4074EA111879
);
impl core::ops::Deref for ITalker2 {
    type Target = IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
interface_hierarchy!(ITalker2, IUnknown, IDispatch);
impl ITalker2 {
    pub unsafe fn GetVolume(&self) -> Result<u32> {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).GetVolume)(Interface::as_raw(self), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn SetVolume(&self, volume: u32) -> Result<()> {
        (Interface::vtable(self).SetVolume)(Interface::as_raw(self), volume).ok()
    }
    pub unsafe fn GetSpeed(&self) -> Result<u32> {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).GetSpeed)(Interface::as_raw(self), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn SetSpeed(&self, speed: u32) -> Result<()> {
        (Interface::vtable(self).SetSpeed)(Interface::as_raw(self), speed).ok()
    }
    pub unsafe fn GetTone(&self) -> Result<u32> {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).GetTone)(Interface::as_raw(self), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn SetTone(&self, tone: u32) -> Result<()> {
        (Interface::vtable(self).SetTone)(Interface::as_raw(self), tone).ok()
    }
    pub unsafe fn GetAlpha(&self) -> Result<u32> {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).GetAlpha)(Interface::as_raw(self), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn SetAlpha(&self, alpha: u32) -> Result<()> {
        (Interface::vtable(self).SetAlpha)(Interface::as_raw(self), alpha).ok()
    }
    pub unsafe fn Components(&self) -> Result<ITalkerComponentArray2> {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).Components)(Interface::as_raw(self), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn GetCast(&self) -> Result<BSTR> {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).GetCast)(Interface::as_raw(self), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn SetCast<P0>(&self, cast: P0) -> Result<()>
    where
        P0: Param<BSTR>,
    {
        (Interface::vtable(self).SetCast)(Interface::as_raw(self), cast.param().abi()).ok()
    }
    pub unsafe fn AvailableCasts(&self) -> Result<IStringArray2> {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).AvailableCasts)(Interface::as_raw(self), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn Speak<P0>(&self, text: P0) -> Result<ISpeakingState2>
    where
        P0: Param<BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).Speak)(Interface::as_raw(self), text.param().abi(), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn Stop(&self) -> Result<VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).Stop)(Interface::as_raw(self), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn GetTextDuration<P0>(&self, text: P0) -> Result<f64>
    where
        P0: Param<BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).GetTextDuration)(
            Interface::as_raw(self),
            text.param().abi(),
            &mut result__,
        )
        .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn GetPhonemes<P0>(&self, text: P0) -> Result<IPhonemeDataArray2>
    where
        P0: Param<BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).GetPhonemes)(
            Interface::as_raw(self),
            text.param().abi(),
            &mut result__,
        )
        .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn OutputWaveToFile<P0, P1>(&self, path: P0, text: P1) -> Result<VARIANT_BOOL>
    where
        P0: Param<BSTR>,
        P1: Param<BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).OutputWaveToFile)(
            Interface::as_raw(self),
            text.param().abi(),
            path.param().abi(),
            &mut result__,
        )
        .and_then(|| Type::from_abi(result__))
    }
}

#[repr(C)]
pub struct ITalker2_Vtbl {
    pub base__: IDispatch_Vtbl,
    pub GetVolume: unsafe extern "system" fn(*mut c_void, *mut u32) -> HRESULT,
    pub SetVolume: unsafe extern "system" fn(*mut c_void, u32) -> HRESULT,
    pub GetSpeed: unsafe extern "system" fn(*mut c_void, *mut u32) -> HRESULT,
    pub SetSpeed: unsafe extern "system" fn(*mut c_void, u32) -> HRESULT,
    pub GetTone: unsafe extern "system" fn(*mut c_void, *mut u32) -> HRESULT,
    pub SetTone: unsafe extern "system" fn(*mut c_void, u32) -> HRESULT,
    pub GetAlpha: unsafe extern "system" fn(*mut c_void, *mut u32) -> HRESULT,
    pub SetAlpha: unsafe extern "system" fn(*mut c_void, u32) -> HRESULT,
    pub Components: unsafe extern "system" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    pub GetCast: unsafe extern "system" fn(*mut c_void, *mut MaybeUninit<BSTR>) -> HRESULT,
    pub SetCast: unsafe extern "system" fn(*mut c_void, MaybeUninit<BSTR>) -> HRESULT,
    pub AvailableCasts: unsafe extern "system" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    pub Speak:
        unsafe extern "system" fn(*mut c_void, MaybeUninit<BSTR>, *mut *mut c_void) -> HRESULT,
    pub Stop: unsafe extern "system" fn(*mut c_void, *mut VARIANT_BOOL) -> HRESULT,
    pub GetTextDuration:
        unsafe extern "system" fn(*mut c_void, MaybeUninit<BSTR>, *mut f64) -> HRESULT,
    pub GetPhonemes:
        unsafe extern "system" fn(*mut c_void, MaybeUninit<BSTR>, *mut *mut c_void) -> HRESULT,
    pub OutputWaveToFile: unsafe extern "system" fn(
        *mut c_void,
        MaybeUninit<BSTR>,
        MaybeUninit<BSTR>,
        *mut VARIANT_BOOL,
    ) -> HRESULT,
}

define_interface!(
    ITalker2V40,
    ITalker2V40_Vtbl,
    0x38C647A8_39EC_433E_94C2_83E532F10CA1
);
impl core::ops::Deref for ITalker2V40 {
    type Target = IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
interface_hierarchy!(ITalker2V40, IUnknown, IDispatch);
impl ITalker2V40 {
    pub unsafe fn GetVolume(&self) -> Result<u32> {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).GetVolume)(Interface::as_raw(self), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn SetVolume(&self, volume: u32) -> Result<()> {
        (Interface::vtable(self).SetVolume)(Interface::as_raw(self), volume).ok()
    }
    pub unsafe fn GetSpeed(&self) -> Result<u32> {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).GetSpeed)(Interface::as_raw(self), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn SetSpeed(&self, speed: u32) -> Result<()> {
        (Interface::vtable(self).SetSpeed)(Interface::as_raw(self), speed).ok()
    }
    pub unsafe fn GetTone(&self) -> Result<u32> {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).GetTone)(Interface::as_raw(self), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn SetTone(&self, tone: u32) -> Result<()> {
        (Interface::vtable(self).SetTone)(Interface::as_raw(self), tone).ok()
    }
    pub unsafe fn GetAlpha(&self) -> Result<u32> {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).GetAlpha)(Interface::as_raw(self), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn SetAlpha(&self, alpha: u32) -> Result<()> {
        (Interface::vtable(self).SetAlpha)(Interface::as_raw(self), alpha).ok()
    }
    pub unsafe fn GetToneScale(&self) -> Result<u32> {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).GetToneScale)(Interface::as_raw(self), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn SetToneScale(&self, tonescale: u32) -> Result<()> {
        (Interface::vtable(self).SetToneScale)(Interface::as_raw(self), tonescale).ok()
    }
    pub unsafe fn Components(&self) -> Result<ITalkerComponentArray2> {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).Components)(Interface::as_raw(self), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn GetCast(&self) -> Result<BSTR> {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).GetCast)(Interface::as_raw(self), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn SetCast<P0>(&self, cast: P0) -> Result<()>
    where
        P0: Param<BSTR>,
    {
        (Interface::vtable(self).SetCast)(Interface::as_raw(self), cast.param().abi()).ok()
    }
    pub unsafe fn AvailableCasts(&self) -> Result<IStringArray2> {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).AvailableCasts)(Interface::as_raw(self), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn Speak<P0>(&self, text: P0) -> Result<ISpeakingState2>
    where
        P0: Param<BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).Speak)(Interface::as_raw(self), text.param().abi(), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn Stop(&self) -> Result<VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).Stop)(Interface::as_raw(self), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn GetTextDuration<P0>(&self, text: P0) -> Result<f64>
    where
        P0: Param<BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).GetTextDuration)(
            Interface::as_raw(self),
            text.param().abi(),
            &mut result__,
        )
        .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn GetPhonemes<P0>(&self, text: P0) -> Result<IPhonemeDataArray2>
    where
        P0: Param<BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).GetPhonemes)(
            Interface::as_raw(self),
            text.param().abi(),
            &mut result__,
        )
        .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn OutputWaveToFile<P0, P1>(&self, path: P0, text: P1) -> Result<VARIANT_BOOL>
    where
        P0: Param<BSTR>,
        P1: Param<BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).OutputWaveToFile)(
            Interface::as_raw(self),
            text.param().abi(),
            path.param().abi(),
            &mut result__,
        )
        .and_then(|| Type::from_abi(result__))
    }
}

#[repr(C)]
pub struct ITalker2V40_Vtbl {
    pub base__: IDispatch_Vtbl,
    pub GetVolume: unsafe extern "system" fn(*mut c_void, *mut u32) -> HRESULT,
    pub SetVolume: unsafe extern "system" fn(*mut c_void, u32) -> HRESULT,
    pub GetSpeed: unsafe extern "system" fn(*mut c_void, *mut u32) -> HRESULT,
    pub SetSpeed: unsafe extern "system" fn(*mut c_void, u32) -> HRESULT,
    pub GetTone: unsafe extern "system" fn(*mut c_void, *mut u32) -> HRESULT,
    pub SetTone: unsafe extern "system" fn(*mut c_void, u32) -> HRESULT,
    pub GetAlpha: unsafe extern "system" fn(*mut c_void, *mut u32) -> HRESULT,
    pub SetAlpha: unsafe extern "system" fn(*mut c_void, u32) -> HRESULT,
    pub GetToneScale: unsafe extern "system" fn(*mut c_void, *mut u32) -> HRESULT,
    pub SetToneScale: unsafe extern "system" fn(*mut c_void, u32) -> HRESULT,
    pub Components: unsafe extern "system" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    pub GetCast: unsafe extern "system" fn(*mut c_void, *mut MaybeUninit<BSTR>) -> HRESULT,
    pub SetCast: unsafe extern "system" fn(*mut c_void, MaybeUninit<BSTR>) -> HRESULT,
    pub AvailableCasts: unsafe extern "system" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    pub Speak:
        unsafe extern "system" fn(*mut c_void, MaybeUninit<BSTR>, *mut *mut c_void) -> HRESULT,
    pub Stop: unsafe extern "system" fn(*mut c_void, *mut VARIANT_BOOL) -> HRESULT,
    pub GetTextDuration:
        unsafe extern "system" fn(*mut c_void, MaybeUninit<BSTR>, *mut f64) -> HRESULT,
    pub GetPhonemes:
        unsafe extern "system" fn(*mut c_void, MaybeUninit<BSTR>, *mut *mut c_void) -> HRESULT,
    pub OutputWaveToFile: unsafe extern "system" fn(
        *mut c_void,
        MaybeUninit<BSTR>,
        MaybeUninit<BSTR>,
        *mut VARIANT_BOOL,
    ) -> HRESULT,
}

define_interface!(
    ITalker2V40Part,
    ITalker2V40Part_Vtbl,
    0xF5EED82F_A947_4B66_9411_4074EA111879
);
impl core::ops::Deref for ITalker2V40Part {
    type Target = IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
interface_hierarchy!(ITalker2V40Part, IUnknown, IDispatch);
impl ITalker2V40Part {
    pub unsafe fn GetToneScale(&self) -> Result<u32> {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).GetToneScale)(Interface::as_raw(self), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn SetToneScale(&self, tonescale: u32) -> Result<()> {
        (Interface::vtable(self).SetToneScale)(Interface::as_raw(self), tonescale).ok()
    }
}

#[repr(C)]
pub struct ITalker2V40Part_Vtbl {
    pub base__: IDispatch_Vtbl,
    pub GetToneScale: unsafe extern "system" fn(*mut c_void, *mut u32) -> HRESULT,
    pub SetToneScale: unsafe extern "system" fn(*mut c_void, u32) -> HRESULT,
}
