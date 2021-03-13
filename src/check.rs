use bindings::windows::win32::menus_and_resources::HCURSOR;
use bindings::windows::win32::system_services::E_HANDLE;
use bindings::windows::win32::windows_and_messaging::HWND;

pub trait CheckHandle {
    fn check_handle(&self) -> windows::Result<Self>
    where
        Self: Sized;
}

macro_rules! impl_check_handle {
    ($handle_type:ty) => {
        impl CheckHandle for $handle_type {
            fn check_handle(&self) -> windows::Result<$handle_type> {
                if *self != 0 {
                    Ok(*self)
                } else {
                    windows::ErrorCode::from_thread().ok()?;
                    Err(windows::Error::fast_error(windows::ErrorCode(
                        E_HANDLE as u32,
                    )))
                }
            }
        }
    };
}

macro_rules! impl_check_handle_binding {
    ($handle_type:ty) => {
        impl CheckHandle for $handle_type {
            fn check_handle(&self) -> windows::Result<$handle_type> {
                if self.0 != 0 {
                    Ok(*self)
                } else {
                    windows::ErrorCode::from_thread().ok()?;
                    Err(windows::Error::fast_error(windows::ErrorCode(
                        E_HANDLE as u32,
                    )))
                }
            }
        }
    };
}

impl_check_handle!(isize);
impl_check_handle!(u16);
impl_check_handle_binding!(HWND);
impl_check_handle_binding!(HCURSOR);
