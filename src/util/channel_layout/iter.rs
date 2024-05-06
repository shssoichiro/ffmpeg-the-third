use ffmpeg_sys_the_third::av_channel_layout_standard;
use libc::c_void;

use super::ChannelLayout;

pub struct ChannelLayoutIter {
    opaque: *mut c_void,
}

impl ChannelLayoutIter {
    pub const fn new() -> Self {
        Self {
            opaque: std::ptr::null_mut(),
        }
    }
}

impl Default for ChannelLayoutIter {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for ChannelLayoutIter {
    type Item = ChannelLayout<'static>;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            av_channel_layout_standard(&mut self.opaque as _)
                .as_ref()
                .map(ChannelLayout::from)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn iter() {
        let iter = ChannelLayoutIter::new();
        let count = iter.count();
        println!("{count}");
        assert!(count > 0);

        let mut iter = ChannelLayoutIter::new();
        assert!(iter.all(|ch| ch.is_valid()));
    }
}
