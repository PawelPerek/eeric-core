pub mod arbitrary_float;
pub mod instruction;
pub mod memory;
pub mod registers;
pub mod snapshot;
pub mod vector_engine;

use derive_builder::Builder;

use instruction::{executor::Executor, Instruction};
use memory::Memory;
use registers::Registers;

use self::vector_engine::VectorEngine;

#[derive(Builder, Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[builder(build_fn(skip))]
pub struct RvCore {
    pub memory: Memory,
    pub instructions: Vec<Instruction>,
    #[builder(setter(skip))]
    pub registers: Registers,
    pub vec_engine: VectorEngine,
}

impl RvCore {
    pub fn step(&mut self) -> Option<Result<(), String>> {
        self.run().next()
    }

    pub fn run(&mut self) -> RunningRvCore {
        RunningRvCore { core: self }
    }
}

impl Default for RvCore {
    fn default() -> Self {
        let vec_engine = VectorEngine::default();

        Self {
            memory: Memory::default(),
            instructions: Vec::new(),
            registers: Registers::default(&vec_engine),
            vec_engine,
        }
    }
}

impl RvCoreBuilder {
    pub fn build(&self) -> RvCore {
        let memory = self.memory.clone().unwrap_or_default();
        let instructions = self.instructions.clone().unwrap_or_default();
        let vec_engine = self.vec_engine.unwrap_or_default();
        let registers = Registers::default(&vec_engine);

        RvCore {
            memory,
            instructions,
            vec_engine,
            registers,
        }
    }
}

pub struct RunningRvCore<'core> {
    core: &'core mut RvCore,
}

impl Iterator for RunningRvCore<'_> {
    type Item = Result<(), String>;

    fn next(&mut self) -> Option<Self::Item> {
        let instruction_pointer = self.core.registers.pc / 4;
        let instruction = self
            .core
            .instructions
            .get(instruction_pointer as usize)?
            .clone();
        
        Some(Executor::new(
            &mut self.core.registers,
            &mut self.core.memory,
            &mut self.core.vec_engine,
        ).execute(instruction))
    }
}

#[cfg(test)]
mod tests {
    use crate::rv_core::{snapshot::Snapshotable, vector_engine::VLEN};

    use super::*;

    #[test]
    fn default_has_vector_registers() {
        let core = RvCore::default();
        assert_eq!(
            core.registers.snapshot().v.len(),
            32 * VLEN::V128.byte_length()
        );
    }
}
