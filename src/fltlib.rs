use windows_sys::Win32::Foundation::*;

#[link(name = "fltlib")]
extern "system" {
    pub fn FilterConnectCommunicationPort(
        lpPortName: *const u16,
        dwOptions: u32,
        lpContext: *const core::ffi::c_void,
        dwSizeOfContext: u16,
        lpSecurityAttributes: *mut core::ffi::c_void,
        hPort: *mut HANDLE,
    ) -> NTSTATUS;

    pub fn FilterSendMessage(
        hPort: HANDLE,
        lpInBuffer: *const core::ffi::c_void,
        dwInBufferSize: u32,
        lpOutBuffer: *mut core::ffi::c_void,
        dwOutBufferSize: u32,
        lpBytesReturned: *mut u32,
    ) -> NTSTATUS;
}
