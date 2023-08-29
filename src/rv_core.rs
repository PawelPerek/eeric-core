pub mod arbitrary_float;
pub mod instruction;
pub mod registers;
pub mod vector_engine;
mod memory;

use instruction::{Instruction, executor::Executor};
use memory::Memory;
use registers::{Registers, RegistersSnapshot};

#[derive(Clone)]
pub struct RvCore {
    registers: Registers,
    instructions: Vec<Instruction>,
    memory: Memory
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
        let program_counter = self.core.registers.pc;
        let instruction = self.core.instructions.get(program_counter as usize)?.clone();
        self.core.registers.pc += 4;
        Executor::new(&mut self.core.registers, &mut self.core.memory).execute(instruction);
        Some(self.core.registers.snapshot())
    }
}