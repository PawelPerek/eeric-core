#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(iter_next_chunk)]

pub mod extensions;
pub mod prelude;
pub mod rv_core;

pub fn wasm_execute(input: rv_core::Instruction) -> u64 {
    let mut core = rv_core::RvCore::new_zeroed();
    core.execute(input);
    core.registers.x[10]
}
