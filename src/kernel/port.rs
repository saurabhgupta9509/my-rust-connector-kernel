use windows_sys::Win32::System::FilterManager::*;
use windows_sys::Win32::Foundation::*;
use windows_sys::core::w;
use std::ptr;
pub struct KernelPort {
    handle: HANDLE,
}

impl KernelPort {
    pub fn connect() -> windows::core::Result<Self> {
        let mut handle = HANDLE(0);

        unsafe {
            FilterConnectCommunicationPort(
                w!("\\DlpPort"),
                0,
                ptr::null(),
                0,
                ptr::null_mut(),
                &mut handle,
            )?;
        }

        Ok(Self { handle })
    }

    pub fn send_policy<T>(&self, policy: &T) -> windows::core::Result<()> {
        unsafe {
            FilterSendMessage(
                self.handle,
                policy as *const _ as _,
                std::mem::size_of::<T>() as u32,
                ptr::null_mut(),
                0,
                ptr::null_mut(),
            )?;
        }
        Ok(())
    }
}
