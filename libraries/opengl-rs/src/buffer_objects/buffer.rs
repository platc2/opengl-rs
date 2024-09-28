use crate::gl;
use raw_handle_derive::RawHandle;

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, RawHandle)]
pub struct Buffer(pub(crate) gl::GLuint);

impl Buffer {
    pub const NO_BUFFER: Buffer = Buffer(0);
}
