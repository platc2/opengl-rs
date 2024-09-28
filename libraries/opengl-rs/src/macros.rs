macro_rules! define_gl_constants {
    ($t:ident :: $($name:ident),+$(,)?) => {
        $(
            pub const $name: $t = $t(gl::$name);
        )+
    };
}

// Export macros
pub(crate) use define_gl_constants;
