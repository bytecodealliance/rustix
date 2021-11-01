use crate::io;
use std::mem::MaybeUninit;
use winapi::um::winsock2::{WSACleanup, WSAGetLastError, WSAStartup, WSADATA};

/// `WSAStartup()`—Initialize process-wide Windows support for sockets.
///
/// On Windows, it's necessary to initialize the sockets subsystem before
/// using sockets APIs. The function performs the necessary initialization.
pub fn wsa_startup() -> io::Result<WSADATA> {
    // Request version 2.2, which has been the latest version since far older
    // versions of Windows than we support here. For more information about
    // the version, see [here].
    //
    // [here]: https://docs.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-wsastartup#remarks
    let version = 0x202;
    let mut data = MaybeUninit::uninit();
    unsafe {
        let ret = WSAStartup(version, data.as_mut_ptr());
        if ret == 0 {
            Ok(data.assume_init())
        } else {
            Err(io::Error::from_raw_os_error(WSAGetLastError()))
        }
    }
}

/// `WSACleanup()`—Clean up process-wide Windows support for sockets.
///
/// In a program where `init` is called, if sockets are no longer necessary,
/// this function releases associated resources.
pub fn wsa_cleanup() -> io::Result<()> {
    unsafe {
        if WSACleanup() == 0 {
            Ok(())
        } else {
            Err(io::Error::from_raw_os_error(WSAGetLastError()))
        }
    }
}
