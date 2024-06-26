use std::{ffi::c_void, mem::MaybeUninit};

use windows::{
    core::{
        define_interface, interface_hierarchy, IUnknown, Interface, Param, Result, Type, BSTR,
        HRESULT,
    },
    Win32::System::Com::{IDispatch, IDispatch_Vtbl},
};

define_interface!(
    ITalkerComponent2,
    ITalkerComponent2_Vtbl,
    0x0FB8C1A2_4895_4EF1_9AC5_71CC45EC61B9
);
impl core::ops::Deref for ITalkerComponent2 {
    type Target = IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
interface_hierarchy!(ITalkerComponent2, IUnknown, IDispatch);
impl ITalkerComponent2 {
    pub unsafe fn Id(&self) -> Result<BSTR> {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).Id)(Interface::as_raw(self), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn Name(&self) -> Result<BSTR> {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).Name)(Interface::as_raw(self), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn GetValue(&self) -> Result<u32> {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).GetValue)(Interface::as_raw(self), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn SetValue(&self, value: u32) -> Result<()> {
        (Interface::vtable(self).SetValue)(Interface::as_raw(self), value).ok()
    }
}

#[repr(C)]
pub struct ITalkerComponent2_Vtbl {
    pub base__: IDispatch_Vtbl,
    pub Id: unsafe extern "system" fn(*mut c_void, *mut MaybeUninit<BSTR>) -> HRESULT,
    pub Name: unsafe extern "system" fn(*mut c_void, *mut MaybeUninit<BSTR>) -> HRESULT,
    pub GetValue: unsafe extern "system" fn(*mut c_void, *mut u32) -> HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut c_void, u32) -> HRESULT,
}

define_interface!(
    ITalkerComponentArray2,
    ITalkerComponentArray2_Vtbl,
    0x50D7AB4E_0D5E_4FCE_B809_E784C58E5355
);
impl core::ops::Deref for ITalkerComponentArray2 {
    type Target = IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
interface_hierarchy!(ITalkerComponentArray2, IUnknown, IDispatch);
impl ITalkerComponentArray2 {
    pub unsafe fn Length(&self) -> Result<i32> {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).Length)(Interface::as_raw(self), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn At(&self, index: i32) -> Result<ITalkerComponent2> {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).At)(Interface::as_raw(self), index, &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn ByName<P0>(&self, name: P0) -> Result<ITalkerComponent2>
    where
        P0: Param<BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).ByName)(Interface::as_raw(self), name.param().abi(), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn Duplicate(&self) -> Result<ITalkerComponentArray2> {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).Duplicate)(Interface::as_raw(self), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
}

#[repr(C)]
pub struct ITalkerComponentArray2_Vtbl {
    pub base__: IDispatch_Vtbl,
    pub Length: unsafe extern "system" fn(*mut c_void, *mut i32) -> HRESULT,
    pub At: unsafe extern "system" fn(*mut c_void, i32, *mut *mut c_void) -> HRESULT,
    pub ByName:
        unsafe extern "system" fn(*mut c_void, MaybeUninit<BSTR>, *mut *mut c_void) -> HRESULT,
    pub Duplicate: unsafe extern "system" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
}
