use crate::macros::impl_for_many;

use super::{Parameters, ParametersMut, ParametersRef};
use crate::codec::Id;
use crate::media;

impl_for_many! {
    impl for Parameters, ParametersRef<'p>, ParametersMut<'p> {
        pub fn medium(&self) -> media::Type {
            unsafe { (*self.as_ptr()).codec_type.into() }
        }

        pub fn id(&self) -> Id {
            unsafe { (*self.as_ptr()).codec_id.into() }
        }
    }
}
