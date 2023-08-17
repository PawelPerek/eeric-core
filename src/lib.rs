#![feature(iter_next_chunk)]

pub mod rv_core;
pub mod extensions;

pub fn wasm_execute(input: rv_core::Instruction) -> u64 {
    let mut core = rv_core::RvCore::new_zeroed();
    core.execute(input);
    core.registers.x[10]
}