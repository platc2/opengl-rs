use super::{Buffer, Target};
use crate::{gl, gl::RawHandle};

pub fn bind_buffer(target: Target, buffer: Buffer) {
    unsafe { gl::BindBuffer(target.raw_handle(), buffer.raw_handle()); }
}
