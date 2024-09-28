# opengl-rs

OpenGL wrappers & function pointer loader for the Rust programming language.
This library uses [gl_generator](https://github.com/brendanzab/gl-rs) to generate and load
OpenGL function pointers.

The functions in this repository have been structured according to the structure proposed in
[docs.gl](https://docs.gl/).

## Basic usage

Add the dependency

```toml
[dependencies]
opengl_rs = "0.0.0"
```

You can load the OpenGL functions and types like this:

```rust
extern crate opengl_rs as gl;
use gl::sys::*;
```

To use the functions, you need to load the function pointers.
Here's an example how to do it with SDL:

```rust
gl::load_with(|s| video_subsystem.gl_get_proc_address(s).cast::<c_void>());
```
