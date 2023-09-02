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

#[derive(Builder, Clone, Default, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct RvCore {
    #[builder(default)]
    pub memory: Memory,
    #[builder(default)]
    pub instructions: Vec<Instruction>,
    #[builder(default)]
    pub registers: Registers,
    #[builder(default)]
    pub vec_engine: VectorEngine,
}

impl RvCore {
    pub fn step(&mut self) -> Option<()> {
        self.run().next()
    }

    pub fn run(&mut self) -> RunningRvCore {
        RunningRvCore { core: self }
    }
}

pub struct RunningRvCore<'core> {
    core: &'core mut RvCore,
}

impl Iterator for RunningRvCore<'_> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        let instruction_pointer = self.core.registers.pc / 4;
        let instruction = self
            .core
            .instructions
            .get(instruction_pointer as usize)?
            .clone();
        Executor::new(
            &mut self.core.registers,
            &mut self.core.memory,
            &mut self.core.vec_engine,
        )
        .execute(instruction);
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::RvCoreBuilder;

    #[test]
    fn all_props_have_default_values() {
        RvCoreBuilder::default().build().unwrap();
    }
}
