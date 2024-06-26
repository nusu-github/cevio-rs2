use std::ffi::c_void;

use windows::{
    core::{define_interface, interface_hierarchy, IUnknown, Interface, Result, Type, HRESULT},
    Win32::{
        Foundation::VARIANT_BOOL,
        System::Com::{IDispatch, IDispatch_Vtbl},
    },
};

define_interface!(
    ISpeakingState2,
    ISpeakingState2_Vtbl,
    0xF726D3E2_5A0F_4CA7_9029_D3549D594350
);
impl core::ops::Deref for ISpeakingState2 {
    type Target = IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
interface_hierarchy!(ISpeakingState2, IUnknown, IDispatch);
impl ISpeakingState2 {
    pub unsafe fn IsSucceeded(&self) -> Result<VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).IsSucceeded)(Interface::as_raw(self), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn IsCompleted(&self) -> Result<VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).IsCompleted)(Interface::as_raw(self), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn Wait(&self) -> Result<()> {
        (Interface::vtable(self).Wait)(Interface::as_raw(self)).ok()
    }
    pub unsafe fn Wait_2(&self, timeoutSeconds: f64) -> Result<()> {
        (Interface::vtable(self).Wait_2)(Interface::as_raw(self), timeoutSeconds).ok()
    }
}

#[repr(C)]
pub struct ISpeakingState2_Vtbl {
    pub base__: IDispatch_Vtbl,
    pub IsSucceeded: unsafe extern "system" fn(*mut c_void, *mut VARIANT_BOOL) -> HRESULT,
    pub IsCompleted: unsafe extern "system" fn(*mut c_void, *mut VARIANT_BOOL) -> HRESULT,
    pub Wait: unsafe extern "system" fn(*mut c_void) -> HRESULT,
    pub Wait_2: unsafe extern "system" fn(*mut c_void, f64) -> HRESULT,
}
