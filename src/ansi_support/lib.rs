pub struct AnsiSupport;

impl AnsiSupport {
    #[cfg(windows)]
    pub fn enable() -> Result<(), std::io::Error> {
        let console_handle: isize = Self::create_console_handle()?;
        Self::set_virtual_terminal_processing(console_handle)
    }

    #[cfg(windows)]
    fn create_console_handle() -> Result<isize, std::io::Error> {
        use std::ffi::OsStr;
        use std::iter::once;
        use std::os::windows::ffi::OsStrExt;

        use windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE;
        use windows_sys::Win32::Storage::FileSystem::CreateFileW;
        use windows_sys::Win32::Storage::FileSystem::FILE_GENERIC_READ;
        use windows_sys::Win32::Storage::FileSystem::FILE_GENERIC_WRITE;
        use windows_sys::Win32::Storage::FileSystem::FILE_SHARE_WRITE;
        use windows_sys::Win32::Storage::FileSystem::OPEN_EXISTING;

        unsafe {
            let console_out_name: Vec<u16> =
                OsStr::new("CONOUT$").encode_wide().chain(once(0)).collect();
            let console_handle: isize = CreateFileW(
                console_out_name.as_ptr(),
                FILE_GENERIC_READ | FILE_GENERIC_WRITE,
                FILE_SHARE_WRITE,
                std::ptr::null(),
                OPEN_EXISTING,
                0,
                0,
            );

            if console_handle == INVALID_HANDLE_VALUE {
                Err(std::io::Error::last_os_error())
            } else {
                Ok(console_handle)
            }
        }
    }

    #[cfg(windows)]
    fn set_virtual_terminal_processing(console_handle: isize) -> Result<(), std::io::Error> {
        use windows_sys::Win32::System::Console::GetConsoleMode;
        use windows_sys::Win32::System::Console::SetConsoleMode;
        use windows_sys::Win32::System::Console::ENABLE_VIRTUAL_TERMINAL_PROCESSING;

        unsafe {
            let mut console_mode: u32 = 0;
            if 0 == GetConsoleMode(console_handle, &mut console_mode) {
                return Err(std::io::Error::last_os_error());
            }

            if console_mode & ENABLE_VIRTUAL_TERMINAL_PROCESSING == 0 {
                if 0 == SetConsoleMode(
                    console_handle,
                    console_mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING,
                ) {
                    return Err(std::io::Error::last_os_error());
                }
            }
            Ok(())
        }
    }

    #[cfg(not(windows))]
    #[inline]
    pub fn enable() -> Result<(), std::io::Error> {
        Ok(())
    }
}
