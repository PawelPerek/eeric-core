#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(iter_next_chunk)]

mod extensions;
mod rv_core;
pub mod prelude;

pub use rv_core::RvCore;
pub use rv_core::instruction::{
    Instruction, 
    format as Format
};
pub use rv_core::registers::aliases::*;