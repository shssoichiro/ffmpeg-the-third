use std::marker::PhantomData;

use crate::codec::packet;
use crate::ffi::*;
use crate::AsPtr;
use libc::c_int;

pub struct SideDataIter<'a, S> {
    stream: &'a S,
    current: c_int,
    _marker: PhantomData<&'a S>,
}

impl<'a, S> SideDataIter<'a, S> {
    pub fn new(stream: &'a S) -> Self {
        Self {
            stream,
            current: 0,
            _marker: PhantomData,
        }
    }
}

impl<'a, S: 'a + AsPtr<AVStream>> Iterator for SideDataIter<'a, S> {
    type Item = packet::SideData<'a>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            if self.current >= (*self.stream.as_ptr()).nb_side_data {
                return None;
            }

            self.current += 1;

            Some(packet::SideData::wrap(
                (*self.stream.as_ptr())
                    .side_data
                    .offset((self.current - 1) as isize),
            ))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        unsafe {
            let length = (*self.stream.as_ptr()).nb_side_data as usize;

            (
                length - self.current as usize,
                Some(length - self.current as usize),
            )
        }
    }
}

impl<'a, S: 'a + AsPtr<AVStream>> ExactSizeIterator for SideDataIter<'a, S> {}
