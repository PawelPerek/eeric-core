#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(iter_next_chunk)]

mod extensions;
pub mod prelude;
mod rv_core;

pub use rv_core::instruction::{format as Format, Instruction};
pub use rv_core::registers::aliases::*;
pub use rv_core::RvCore;
