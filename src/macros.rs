macro_rules! impl_for_one {
    // ref/mut with lifetime
    ($wrapper:ident, $lt:lifetime, $func:item) => {
        impl<$lt> $wrapper<$lt> {
            $func
        }
    };
    // owned without lifetime
    ($wrapper:ident, $func:item) => {
        impl $wrapper {
            $func
        }
    };
}

macro_rules! impl_for_many {
    { impl for $($wrapper:ident$(<$lt:lifetime>)?),+ {} } => {};
    {
        impl for $($wrapper:ident$(<$lt:lifetime>)?),+ {
            $func:item
            $($tt:tt)*
        }
    } => {
        $(
            $crate::macros::impl_for_one!($wrapper$(, $lt)?, $func);
        )+

        impl_for_many!{
            impl for $($wrapper$(<$lt>)?),+ {
                $($tt)*
            }
        }
    };
}

pub(crate) use impl_for_many;
pub(crate) use impl_for_one;
