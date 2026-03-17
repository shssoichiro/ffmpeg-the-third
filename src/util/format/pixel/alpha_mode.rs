use libc::c_int;
use std::ffi::CString;

use crate::ffi::*;
use crate::utils;
use crate::Error;

/// Correlation between the alpha channel and color values.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum AlphaMode {
    Unspecified,
    Premultiplied,
    Straight,
}

impl AlphaMode {
    pub fn name(self) -> Option<&'static str> {
        unsafe {
            let ptr = av_alpha_mode_name(self.into());
            utils::optional_str_from_c_ptr(ptr)
        }
    }

    pub fn from_name<S: AsRef<str>>(name: S) -> Result<Self, Error> {
        let name = CString::new(name.as_ref()).unwrap();
        let ret = unsafe { av_alpha_mode_from_name(name.as_ptr()) };

        if (ret.0 as c_int) < 0 {
            Err(Error::from(ret.0 as c_int))
        } else {
            Ok(Self::from(ret))
        }
    }
}

impl From<AVAlphaMode> for AlphaMode {
    fn from(value: AVAlphaMode) -> Self {
        use AVAlphaMode as AV;

        match value {
            AV::AVALPHA_MODE_UNSPECIFIED => Self::Unspecified,
            AV::AVALPHA_MODE_PREMULTIPLIED => Self::Premultiplied,
            AV::AVALPHA_MODE_STRAIGHT => Self::Straight,
            _ => unreachable!(),
        }
    }
}

impl From<AlphaMode> for AVAlphaMode {
    fn from(value: AlphaMode) -> Self {
        use AVAlphaMode as AV;

        match value {
            AlphaMode::Unspecified => AV::AVALPHA_MODE_UNSPECIFIED,
            AlphaMode::Premultiplied => AV::AVALPHA_MODE_PREMULTIPLIED,
            AlphaMode::Straight => AV::AVALPHA_MODE_STRAIGHT,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name() {
        let tests = [
            AlphaMode::Unspecified,
            AlphaMode::Premultiplied,
            AlphaMode::Straight,
        ];

        for mode in tests {
            println!("{}", mode.name().expect("has name"));
        }
    }

    #[test]
    fn from_name() {
        assert!(AlphaMode::from_name("some_nonexistent NAME").is_err());
        assert_eq!(AlphaMode::from_name("straight"), Ok(AlphaMode::Straight));
    }
}
