pub mod arbitrary_float;
pub mod instruction;
pub mod memory;
pub mod registers;
pub mod vector_engine;

use instruction::{executor::Executor, Instruction};
use memory::{Memory, MemorySnapshot};
use registers::{Registers, RegistersSnapshot};
use vector_engine::VectorEngineSnapshot;

#[derive(Clone)]
pub struct RvCore {
    registers: Registers,
    instructions: Vec<Instruction>,
    memory: Memory,
}

impl RvCore {
    pub fn new_zeroed() -> Self {
        Self {
            registers: Default::default(),
            instructions: Default::default(),
            memory: Default::default(),
        }
    }

    pub fn with_instructions(instructions: Vec<Instruction>) -> Self {
        Self {
            registers: Default::default(),
            instructions,
            memory: Default::default(),
        }
    }

    pub fn registers_snapshot(&self) -> RegistersSnapshot {
        self.registers.snapshot()
    }

    pub fn memory_snapshot(&self) -> MemorySnapshot {
        self.memory.snapshot()
    }

    pub fn vector_engine_snapshot(&self) -> VectorEngineSnapshot {
        self.registers.v.vec_engine.snapshot()
    }

    pub fn step(&mut self) -> Option<RegistersSnapshot> {
        self.run().next()
    }

    pub fn run(&mut self) -> RunningRvCore {
        RunningRvCore { core: self }
    }
}

pub struct RunningRvCore<'m> {
    core: &'m mut RvCore,
}

impl Iterator for RunningRvCore<'_> {
    type Item = RegistersSnapshot;

    fn next(&mut self) -> Option<Self::Item> {
        let instruction_pointer = self.core.registers.pc / 4;
        let instruction = self
            .core
            .instructions
            .get(instruction_pointer as usize)?
            .clone();
        Executor::new(&mut self.core.registers, &mut self.core.memory).execute(instruction);
        Some(self.core.registers.snapshot())
    }
}