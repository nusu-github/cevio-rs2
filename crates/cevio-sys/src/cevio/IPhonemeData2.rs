use std::{ffi::c_void, mem::MaybeUninit};

use windows::{
    core::{
        define_interface, interface_hierarchy, IUnknown, Interface, Result, Type, BSTR, HRESULT,
    },
    Win32::System::Com::{IDispatch, IDispatch_Vtbl},
};

define_interface!(
    IPhonemeData2,
    IPhonemeData2_Vtbl,
    0xA409140F_B7A5_48EF_893E_8B971420E05A
);
impl core::ops::Deref for IPhonemeData2 {
    type Target = IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
interface_hierarchy!(IPhonemeData2, IUnknown, IDispatch);
impl IPhonemeData2 {
    pub unsafe fn Phoneme(&self) -> Result<BSTR> {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).Phoneme)(Interface::as_raw(self), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn StartTime(&self) -> Result<f64> {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).StartTime)(Interface::as_raw(self), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn EndTime(&self) -> Result<f64> {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).EndTime)(Interface::as_raw(self), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
}

#[repr(C)]
pub struct IPhonemeData2_Vtbl {
    pub base__: IDispatch_Vtbl,
    pub Phoneme: unsafe extern "system" fn(*mut c_void, *mut MaybeUninit<BSTR>) -> HRESULT,
    pub StartTime: unsafe extern "system" fn(*mut c_void, *mut f64) -> HRESULT,
    pub EndTime: unsafe extern "system" fn(*mut c_void, *mut f64) -> HRESULT,
}

define_interface!(
    IPhonemeDataArray2,
    IPhonemeDataArray2_Vtbl,
    0x96C207D5_B547_485F_8538_96A5BE0C73B6
);
impl core::ops::Deref for IPhonemeDataArray2 {
    type Target = IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}

interface_hierarchy!(IPhonemeDataArray2, IUnknown, IDispatch);
impl IPhonemeDataArray2 {
    pub unsafe fn Length(&self) -> Result<i32> {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).Length)(Interface::as_raw(self), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn At(&self, index: i32) -> Result<IPhonemeData2> {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).At)(Interface::as_raw(self), index, &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn Duplicate(&self) -> Result<IPhonemeDataArray2> {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).Duplicate)(Interface::as_raw(self), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
}

#[repr(C)]
pub struct IPhonemeDataArray2_Vtbl {
    pub base__: IDispatch_Vtbl,
    pub Length: unsafe extern "system" fn(*mut c_void, *mut i32) -> HRESULT,
    pub At: unsafe extern "system" fn(*mut c_void, i32, *mut *mut c_void) -> HRESULT,
    pub Duplicate: unsafe extern "system" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
}
