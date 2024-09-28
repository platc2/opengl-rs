use crate::gl;
use crate::macros;
use raw_handle_derive::RawHandle;

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, RawHandle)]
pub struct Target(pub(crate) gl::GLuint);

impl Target {
    macros::define_gl_constants!(Target ::
        ARRAY_BUFFER,
        COPY_READ_BUFFER,
        COPY_WRITE_BUFFER,
        DRAW_INDIRECT_BUFFER,
        ELEMENT_ARRAY_BUFFER,
        PIXEL_PACK_BUFFER,
        PIXEL_UNPACK_BUFFER,
        TEXTURE_BUFFER,
        TRANSFORM_FEEDBACK,
        UNIFORM_BUFFER);

    #[cfg(feature = "GL42")]
    macros::define_gl_constants!(Target ::
        ATOMIC_COUNTER_BUFFER);

    #[cfg(feature = "GL43")]
    macros::define_gl_constants!(Target ::
        DISPATCH_INDIRECT_BUFFER,
        SHADER_STORAGE_BUFFER);

    #[cfg(feature = "GL44")]
    macros::define_gl_constants!(Target ::
        QUERY_BUFFER);
}
