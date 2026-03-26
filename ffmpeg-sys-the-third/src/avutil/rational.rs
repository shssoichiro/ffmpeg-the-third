use crate::AVRational;
use libc::{c_double, c_int};

pub const fn av_make_q(num: c_int, den: c_int) -> AVRational {
    AVRational { num, den }
}

pub fn av_cmp_q(a: AVRational, b: AVRational) -> c_int {
    let tmp = i64::from(a.num) * i64::from(b.den) - i64::from(b.num) * i64::from(a.den);

    if tmp != 0 {
        (((tmp ^ i64::from(a.den) ^ i64::from(b.den)) >> 63) | 1) as c_int
    } else if b.den != 0 && a.den != 0 {
        0
    } else if a.num != 0 && b.num != 0 {
        ((a.num >> 31) - (b.num >> 31)) as c_int
    } else {
        c_int::MIN
    }
}

pub fn av_q2d(a: AVRational) -> c_double {
    f64::from(a.num) / f64::from(a.den)
}

pub const fn av_inv_q(q: AVRational) -> AVRational {
    AVRational {
        num: q.den,
        den: q.num,
    }
}
