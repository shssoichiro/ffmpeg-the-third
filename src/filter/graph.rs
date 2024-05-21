use std::ffi::{CStr, CString};
use std::ptr::{self, NonNull};
use std::str::from_utf8_unchecked;

use super::{Context, Filter};
use crate::ffi::*;
use crate::Error;
use libc::c_int;

pub struct Graph {
    ptr: *mut AVFilterGraph,
}

unsafe impl Send for Graph {}
unsafe impl Sync for Graph {}

impl Graph {
    pub unsafe fn wrap(ptr: *mut AVFilterGraph) -> Self {
        Graph { ptr }
    }

    pub unsafe fn as_ptr(&self) -> *const AVFilterGraph {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVFilterGraph {
        self.ptr
    }
}

impl Graph {
    pub fn new() -> Self {
        unsafe {
            let ptr = avfilter_graph_alloc();

            if ptr.is_null() {
                panic!("out of memory");
            }

            Graph::wrap(ptr)
        }
    }

    pub fn validate(&mut self) -> Result<(), Error> {
        unsafe {
            match avfilter_graph_config(self.as_mut_ptr(), ptr::null_mut()) {
                0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn add<'a, 'b>(
        &'a mut self,
        filter: &Filter,
        name: &str,
        args: &str,
    ) -> Result<Context<'b>, Error>
    where
        'a: 'b,
    {
        unsafe {
            let name = CString::new(name).unwrap();
            let args = CString::new(args).unwrap();
            let mut context = ptr::null_mut();

            match avfilter_graph_create_filter(
                &mut context as *mut *mut AVFilterContext,
                filter.as_ptr(),
                name.as_ptr(),
                args.as_ptr(),
                ptr::null_mut(),
                self.as_mut_ptr(),
            ) {
                n if n >= 0 => Ok(Context::wrap(context)),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn get<'a, 'b>(&'b mut self, name: &str) -> Option<Context<'b>>
    where
        'a: 'b,
    {
        unsafe {
            let name = CString::new(name).unwrap();
            let ptr = avfilter_graph_get_filter(self.as_mut_ptr(), name.as_ptr());

            if ptr.is_null() {
                None
            } else {
                Some(Context::wrap(ptr))
            }
        }
    }

    pub fn dump(&self) -> String {
        unsafe {
            let ptr = avfilter_graph_dump(self.as_ptr() as *mut _, ptr::null());
            let cstr = from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes());
            let string = cstr.to_owned();

            av_free(ptr as *mut _);

            string
        }
    }

    pub fn input(&mut self, name: &str, pad: usize) -> Result<Parser, Error> {
        Parser::new(self).input(name, pad)
    }

    pub fn output(&mut self, name: &str, pad: usize) -> Result<Parser, Error> {
        Parser::new(self).output(name, pad)
    }

    pub fn parse(&mut self, spec: &str) -> Result<(), Error> {
        Parser::new(self).parse(spec)
    }
}

impl Drop for Graph {
    fn drop(&mut self) {
        unsafe {
            avfilter_graph_free(&mut self.as_mut_ptr());
        }
    }
}

pub struct Parser<'a> {
    graph: &'a mut Graph,
    inputs: *mut AVFilterInOut,
    outputs: *mut AVFilterInOut,
}

impl<'a> Parser<'a> {
    pub fn new(graph: &mut Graph) -> Parser {
        Parser {
            graph,
            inputs: ptr::null_mut(),
            outputs: ptr::null_mut(),
        }
    }

    pub fn input(mut self, name: &str, pad: usize) -> Result<Self, Error> {
        unsafe {
            let mut context = self.graph.get(name).ok_or(Error::InvalidData)?;
            let mut input = NonNull::new(avfilter_inout_alloc()).expect("out of memory");

            let name = CString::new(name).unwrap();

            input.as_mut().name = av_strdup(name.as_ptr());
            input.as_mut().filter_ctx = context.as_mut_ptr();
            input.as_mut().pad_idx = pad as c_int;
            input.as_mut().next = ptr::null_mut();

            append_inout(&mut self.inputs, input);
        }

        Ok(self)
    }

    pub fn output(mut self, name: &str, pad: usize) -> Result<Self, Error> {
        unsafe {
            let mut context = self.graph.get(name).ok_or(Error::InvalidData)?;
            let mut output = NonNull::new(avfilter_inout_alloc()).expect("out of memory");

            let name = CString::new(name).unwrap();

            output.as_mut().name = av_strdup(name.as_ptr());
            output.as_mut().filter_ctx = context.as_mut_ptr();
            output.as_mut().pad_idx = pad as c_int;
            output.as_mut().next = ptr::null_mut();

            append_inout(&mut self.outputs, output);
        }

        Ok(self)
    }

    pub fn parse(mut self, spec: &str) -> Result<(), Error> {
        unsafe {
            let spec = CString::new(spec).unwrap();

            let result = avfilter_graph_parse_ptr(
                self.graph.as_mut_ptr(),
                spec.as_ptr(),
                &mut self.inputs,
                &mut self.outputs,
                ptr::null_mut(),
            );

            avfilter_inout_free(&mut self.inputs);
            avfilter_inout_free(&mut self.outputs);

            match result {
                n if n >= 0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }
}

fn append_inout(list: &mut *mut AVFilterInOut, element: NonNull<AVFilterInOut>) {
    unsafe {
        if list.is_null() {
            *list = element.as_ptr();
            return;
        }

        let mut curr = *list;
        while !(*curr).next.is_null() {
            curr = (*curr).next;
        }

        (*curr).next = element.as_ptr();
    }
}

impl<'a> Drop for Parser<'a> {
    fn drop(&mut self) {
        unsafe {
            avfilter_inout_free(&mut self.inputs);
            avfilter_inout_free(&mut self.outputs);
        }
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}
