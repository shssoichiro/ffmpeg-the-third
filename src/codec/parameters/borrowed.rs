use std::marker::PhantomData;
use std::ptr::NonNull;
#[cfg(feature = "ffmpeg_7_0")]
use std::slice;

#[cfg(feature = "ffmpeg_7_0")]
use crate::codec::packet::side_data::Type as SideDataType;
use crate::ffi::*;
use crate::AsPtr;

pub struct ParametersRef<'p> {
    ptr: NonNull<AVCodecParameters>,
    _marker: PhantomData<&'p AVCodecParameters>,
}

impl<'p> ParametersRef<'p> {
    /// # Safety
    ///
    /// Ensure that
    /// - `ptr` is either null or valid,
    /// - the shared borrow represented by `ptr` follows Rust borrow rules and
    /// - the lifetime of the returned struct is correctly bounded.
    pub unsafe fn from_raw(ptr: *const AVCodecParameters) -> Option<Self> {
        NonNull::new(ptr as *mut _).map(|ptr| Self {
            ptr,
            _marker: PhantomData,
        })
    }

    /// Exposes a pointer to the contained [`AVCodecParameters`] for FFI purposes.
    ///
    /// This is guaranteed to be a non-null pointer.
    pub fn as_ptr(&self) -> *const AVCodecParameters {
        self.ptr.as_ptr()
    }

    /// Borrow global coded side data of the requested kind, if present.
    ///
    /// The bytes belong to these codec parameters and remain valid only while
    /// this shared borrow is held. Packet-specific side data is not included.
    #[cfg(feature = "ffmpeg_7_0")]
    pub fn side_data(&self, kind: SideDataType) -> Option<&[u8]> {
        unsafe {
            let parameters = self.ptr.as_ref();
            let count = usize::try_from(parameters.nb_coded_side_data).ok()?;
            if count == 0 || parameters.coded_side_data.is_null() {
                return None;
            }
            let items = slice::from_raw_parts(parameters.coded_side_data, count);
            let item = items.iter().find(|item| item.type_ == kind.into())?;
            if item.size == 0 {
                return Some(&[]);
            }
            if item.data.is_null() {
                return None;
            }
            Some(slice::from_raw_parts(item.data, item.size))
        }
    }
}

impl<'p> AsPtr<AVCodecParameters> for ParametersRef<'p> {
    fn as_ptr(&self) -> *const AVCodecParameters {
        self.ptr.as_ptr()
    }
}

#[cfg(all(test, feature = "ffmpeg_7_1"))]
mod tests {
    use super::*;
    use crate::codec::Parameters;

    fn add_side_data(parameters: &mut Parameters, kind: SideDataType, bytes: &[u8]) {
        unsafe {
            let parameters = parameters.as_mut_ptr();
            let item = av_packet_side_data_new(
                &mut (*parameters).coded_side_data,
                &mut (*parameters).nb_coded_side_data,
                kind.into(),
                bytes.len(),
                0,
            );
            assert!(!item.is_null(), "av_packet_side_data_new failed");
            slice::from_raw_parts_mut((*item).data, bytes.len()).copy_from_slice(bytes);
        }
    }

    #[test]
    fn side_data_is_none_without_side_data() {
        let parameters = Parameters::new();
        let borrowed = unsafe { ParametersRef::from_raw(parameters.as_ptr()) }.unwrap();
        assert_eq!(borrowed.side_data(SideDataType::DisplayMatrix), None);
    }

    #[test]
    fn side_data_finds_requested_kind() {
        let mut parameters = Parameters::new();
        add_side_data(&mut parameters, SideDataType::ReplayGain, &[1, 2, 3]);
        add_side_data(&mut parameters, SideDataType::DisplayMatrix, &[4, 5, 6, 7]);
        add_side_data(&mut parameters, SideDataType::Stereo3d, &[]);

        let borrowed = unsafe { ParametersRef::from_raw(parameters.as_ptr()) }.unwrap();
        assert_eq!(
            borrowed.side_data(SideDataType::DisplayMatrix),
            Some(&[4, 5, 6, 7][..])
        );
        assert_eq!(
            borrowed.side_data(SideDataType::ReplayGain),
            Some(&[1, 2, 3][..])
        );
        assert_eq!(borrowed.side_data(SideDataType::Stereo3d), Some(&[][..]));
        assert_eq!(borrowed.side_data(SideDataType::NewExtraData), None);
    }
}
