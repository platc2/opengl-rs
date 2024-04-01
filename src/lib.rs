pub use sys::load_with;

pub mod sys {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
