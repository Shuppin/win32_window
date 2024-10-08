use windows::Win32::Foundation::*;

use crate::error::{PalmError, PalmErrorKind, PalmResult};

pub mod gl;
pub mod window;

impl TryInto<PalmError> for windows::core::Error {
    type Error = Self;

    fn try_into(self) -> Result<PalmError, Self::Error> {
        let result = match WIN32_ERROR(self.code().0 as u32) {
            ERROR_OUTOFMEMORY | ERROR_NOT_ENOUGH_MEMORY => PalmErrorKind::NotEnoughMemory.into(),
            _ => return Err(self),
        };
        Ok(result)
    }
}

trait IntoPalmError<T> {
    fn map_palm_err(self) -> PalmResult<T>;

    fn with_err_msg(self, msg: &str) -> PalmResult<T>;
}

impl<T> IntoPalmError<T> for windows::core::Result<T> {
    fn map_palm_err(self) -> PalmResult<T> {
        self.map_err(
            |e| match <windows::core::Error as TryInto<PalmError>>::try_into(e) {
                Ok(value) => value,
                // TODO: change source of this panic to caller
                Err(e) => panic!("{}", e.message()),
            },
        )
    }

    fn with_err_msg(self, msg: &str) -> PalmResult<T> {
        self.map_err(
            |e| match <windows::core::Error as TryInto<PalmError>>::try_into(e) {
                Ok(value) => value.with_msg(msg),
                Err(_) => panic!("{}", msg),
            },
        )
    }
}
