#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(iter_next_chunk)]

mod extensions;
mod prelude;
mod rv_core;

pub use rv_core::RvCore;
pub use rv_core::instruction::{format::*, Instruction};
pub use rv_core::registers::aliases::*;