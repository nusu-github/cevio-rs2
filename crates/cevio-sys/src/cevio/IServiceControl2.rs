use std::{ffi::c_void, mem::MaybeUninit};

use windows::{
    core::{
        define_interface, interface_hierarchy, IUnknown, Interface, Result, Type, BSTR, HRESULT,
    },
    Win32::{
        Foundation::VARIANT_BOOL,
        System::Com::{IDispatch, IDispatch_Vtbl},
    },
};

define_interface!(
    IServiceControl2,
    IServiceControl2_Vtbl,
    0xF726D3E2_5A0F_4CA7_9029_D3549D594350
);
impl core::ops::Deref for IServiceControl2 {
    type Target = IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
interface_hierarchy!(IServiceControl2, IUnknown, IDispatch);
impl IServiceControl2 {
    pub unsafe fn IsHostStarted(&self) -> Result<VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).IsHostStarted)(Interface::as_raw(self), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn StartHost<P0>(&self, noWait: P0) -> Result<i32>
    where
        P0: Into<VARIANT_BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).StartHost)(Interface::as_raw(self), noWait.into(), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn CloseHost(&self, nReason: i32) -> Result<()> {
        (Interface::vtable(self).CloseHost)(Interface::as_raw(self), nReason).ok()
    }
}
#[repr(C)]
pub struct IServiceControl2_Vtbl {
    pub base__: IDispatch_Vtbl,
    pub IsHostStarted: unsafe extern "system" fn(*mut c_void, *mut VARIANT_BOOL) -> HRESULT,
    pub StartHost: unsafe extern "system" fn(*mut c_void, VARIANT_BOOL, *mut i32) -> HRESULT,
    pub CloseHost: unsafe extern "system" fn(*mut c_void, i32) -> HRESULT,
}

define_interface!(
    IServiceControl2V40,
    IServiceControl2V40_Vtbl,
    0x5318F2D5_9973_4A11_A2D7_80FB823B7F9F
);
impl core::ops::Deref for IServiceControl2V40 {
    type Target = IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
interface_hierarchy!(IServiceControl2V40, IUnknown, IDispatch);
impl IServiceControl2V40 {
    pub unsafe fn HostVersion(&self) -> Result<BSTR> {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).HostVersion)(Interface::as_raw(self), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn InterfaceVersion(&self) -> Result<BSTR> {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).InterfaceVersion)(Interface::as_raw(self), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn IsHostStarted(&self) -> Result<VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).IsHostStarted)(Interface::as_raw(self), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn StartHost<P0>(&self, noWait: P0) -> Result<i32>
    where
        P0: Into<VARIANT_BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).StartHost)(Interface::as_raw(self), noWait.into(), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn CloseHost(&self, nReason: i32) -> Result<()> {
        (Interface::vtable(self).CloseHost)(Interface::as_raw(self), nReason).ok()
    }
}

#[repr(C)]
pub struct IServiceControl2V40_Vtbl {
    pub base__: IDispatch_Vtbl,
    pub HostVersion: unsafe extern "system" fn(*mut c_void, *mut MaybeUninit<BSTR>) -> HRESULT,
    pub InterfaceVersion: unsafe extern "system" fn(*mut c_void, *mut MaybeUninit<BSTR>) -> HRESULT,
    pub IsHostStarted: unsafe extern "system" fn(*mut c_void, *mut VARIANT_BOOL) -> HRESULT,
    pub StartHost: unsafe extern "system" fn(*mut c_void, VARIANT_BOOL, *mut i32) -> HRESULT,
    pub CloseHost: unsafe extern "system" fn(*mut c_void, i32) -> HRESULT,
}

define_interface!(
    IServiceControl2V40Part,
    IServiceControl2V40Part_Vtbl,
    0x0D1EF04D_07EE_4CAF_A424_E7819BE41516
);
impl core::ops::Deref for IServiceControl2V40Part {
    type Target = IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
interface_hierarchy!(IServiceControl2V40Part, IUnknown, IDispatch);
impl IServiceControl2V40Part {
    pub unsafe fn HostVersion(&self) -> Result<BSTR> {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).HostVersion)(Interface::as_raw(self), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
    pub unsafe fn InterfaceVersion(&self) -> Result<BSTR> {
        let mut result__ = core::mem::zeroed();
        (Interface::vtable(self).InterfaceVersion)(Interface::as_raw(self), &mut result__)
            .and_then(|| Type::from_abi(result__))
    }
}

#[repr(C)]
pub struct IServiceControl2V40Part_Vtbl {
    pub base__: IDispatch_Vtbl,
    pub HostVersion: unsafe extern "system" fn(*mut c_void, *mut MaybeUninit<BSTR>) -> HRESULT,
    pub InterfaceVersion: unsafe extern "system" fn(*mut c_void, *mut MaybeUninit<BSTR>) -> HRESULT,
}
