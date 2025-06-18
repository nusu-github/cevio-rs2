use windows::{
    core::Result,
    Win32::{
        Foundation::{S_FALSE, S_OK},
        System::Com::{CoInitializeEx, CoUninitialize, COINIT_MULTITHREADED},
    },
};

use parking_lot::{const_mutex, Mutex};

static COM_MANAGER: std::sync::LazyLock<Mutex<ComManager>> =
    std::sync::LazyLock::new(|| const_mutex(ComManager::new()));

struct ComManager {
    ref_count: usize,
}

impl ComManager {
    const fn new() -> Self {
        Self { ref_count: 0 }
    }

    fn initialize(&mut self) -> Result<()> {
        if self.ref_count == 0 {
            unsafe {
                let hr = CoInitializeEx(None, COINIT_MULTITHREADED);
                // S_OKまたはS_FALSE（既に初期化済み）の場合は成功
                if hr != S_OK && hr != S_FALSE {
                    return Err(hr.into());
                }
            }
        }
        self.ref_count += 1;
        Ok(())
    }

    fn uninitialize(&mut self) {
        if self.ref_count > 0 {
            self.ref_count -= 1;
            if self.ref_count == 0 {
                unsafe {
                    CoUninitialize();
                }
            }
        }
    }
}

pub struct ComGuard;

impl ComGuard {
    pub fn new() -> Result<Self> {
        COM_MANAGER.lock().initialize()?;
        Ok(Self)
    }
}

impl Drop for ComGuard {
    fn drop(&mut self) {
        COM_MANAGER.lock().uninitialize();
    }
}
