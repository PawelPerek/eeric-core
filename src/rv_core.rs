mod instruction;
mod memory;
mod registers;

use instruction::{format, Instruction};
use memory::Memory;
use registers::Registers;

struct RvCore<'a> {
    registers: Registers,
    memory: Memory,
    executor: Executor<'a>
}

impl<'a> RvCore<'a> {
    fn new_zeroed() -> RvCore<'a> {
        Machine {
            registers: Default::default(),
            memory: Default::default(),
        }
    }

    fn new(registers: Registers, memory: Memory) -> Machine {
        Machine { registers, memory }
    }

    fn execute(&mut self, input: Instruction) {
        use r#type::*;
        use Instruction::*;

        match input {
            Add(R { rd, rs1, rs2 }) => {
                let rs1 = self.registers.get(rs1);
                let rs2 = self.registers.get(rs2);
                self.registers.set(rd, rs1 + rs2);
            }
            Sub(R { rd, rs1, rs2 }) => {
                let rs1 = self.registers.get(rs1);
                let rs2 = self.registers.get(rs2);
                self.registers.set(rd, rs1 - rs2);
            }
            Addi(I { rd, rs, imm12 }) => {
                let rs = self.registers.get(rs);
                self.registers.set(rd, rs + imm12 as u32);
            }
            Slt(R { rd, rs1, rs2 }) => {
                let rs1 = self.registers.get(rs1) as i32;
                let rs2 = self.registers.get(rs2) as i32;
                self.registers.set(rd, if rs1 < rs2 { 1 } else { 0 });
            }
            Slti(I { rd, rs, imm12 }) => {
                let rs = self.registers.get(rs);
                self.registers
                    .set(rd, if rs < imm12 as u32 { 1 } else { 0 });
            }
            Sltu(R { rd, rs1, rs2 }) => {
                let rs1 = self.registers.get(rs1);
                let rs2 = self.registers.get(rs2);
                self.registers.set(rd, if rs1 < rs2 { 1 } else { 0 });
            }
            Sltiu(I { rd, rs, imm12 }) => {
                let rs = self.registers.get(rs) as i32;
                self.registers
                    .set(rd, if rs < imm12 as i32 { 1 } else { 0 });
            }
            Lui(U { rd, imm20 }) => self.registers.set(rd, imm20 << 12),
            Auip(U { rd, imm20 }) => {
                let pc = self.registers.pc;
                self.registers.set(rd, pc + imm20 << 12);
            }

            And(R { rd, rs1, rs2 }) => {
                let rs1 = self.registers.get(rs1);
                let rs2 = self.registers.get(rs2);
                self.registers.set(rd, rs1 & rs2);
            }
            Or(R { rd, rs1, rs2 }) => {
                let rs1 = self.registers.get(rs1);
                let rs2 = self.registers.get(rs2);
                self.registers.set(rd, rs1 | rs2);
            }
            Xor(R { rd, rs1, rs2 }) => {
                let rs1 = self.registers.get(rs1);
                let rs2 = self.registers.get(rs2);
                self.registers.set(rd, rs1 ^ rs2);
            }
            Andi(I { rd, rs, imm12 }) => {
                let rs = self.registers.get(rs);
                self.registers.set(rd, rs & imm12 as u32);
            }
            Ori(I { rd, rs, imm12 }) => {
                let rs = self.registers.get(rs);
                self.registers.set(rd, rs | imm12 as u32);
            }
            Xori(I { rd, rs, imm12 }) => {
                let rs = self.registers.get(rs);
                self.registers.set(rd, rs ^ imm12 as u32);
            }
            Sll(R { rd, rs1, rs2 }) => {
                let rs1 = self.registers.get(rs1);
                let rs2 = self.registers.get(rs2) & 0b11111;
                self.registers.set(rd, rs1 << rs2);
            }
            Srl(R { rd, rs1, rs2 }) => {
                let rs1 = self.registers.get(rs1);
                let rs2 = self.registers.get(rs2) & 0b11111;
                self.registers.set(rd, rs1 >> rs2);
            }
            Sra(R { rd, rs1, rs2 }) => {
                let rs1 = self.registers.get(rs1) as i32;
                let rs2 = self.registers.get(rs2) as i32 & 0b11111;
                self.registers.set(rd, (rs1 >> rs2) as u32);
            }
            Slli(I { rd, rs, imm12 }) => {
                let rs = self.registers.get(rs);
                let shamt = imm12 & 0b11111;
                self.registers.set(rd, rs << shamt);
            }
            Srli(I { rd, rs, imm12 }) => {
                let rs = self.registers.get(rs);
                let shamt = imm12 & 0b11111;
                self.registers.set(rd, rs >> shamt);
            }
            Srai(I { rd, rs, imm12 }) => {
                let rs = self.registers.get(rs) as i32;
                let shamt = imm12 & 0b11111;
                self.registers.set(rd, (rs >> shamt) as u32);
            },
            Lw(I {rd, rs, imm12}) => {
                let address = (self.registers.get(rs) + imm12 as u32) as usize;
                let value = u32::from_le_bytes(self.memory.0[address .. address + 4].try_into().unwrap());
                self.registers.set(rd, value);
            },
            Lh(I {rd, rs, imm12}) => {
                let address: usize = (self.registers.get(rs) + imm12 as u32) as usize;
                let value = u16::from_le_bytes(self.memory.0[address .. address + 2].try_into().unwrap());
                self.registers.set(rd, value as u32);
            },
            Lb(I {rd, rs, imm12}) => {
                let address: usize = (self.registers.get(rs) + imm12 as u32) as usize;
                let value = self.memory.0[address];
                self.registers.set(rd, value as u32);
            },
            Lhu(_) => todo!(),
            Lbu(_) => todo!(),
            Sw(_) => todo!(),
            Sh(_) => todo!(),
            Sb(_) => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use r#type::*;
    use Instruction::*;

    #[test]
    fn add() {
        let mut machine = Machine::new_zeros();

        machine.registers.set(1, 2); // rs1: 2
        machine.registers.set(2, 3); // rs2: 3
        machine.execute(Add(R {
            rd: 3,
            rs1: 1,
            rs2: 2,
        }));

        assert_eq!(machine.registers.get(3), 5); // rd: 2 + 3 = 5
    }

    #[test]
    fn sub() {
        let mut machine = Machine::new_zeros();

        machine.registers.set(1, 5); // rs1: 5
        machine.registers.set(2, 3); // rs2: 3
        machine.execute(Sub(R {
            rd: 3,
            rs1: 1,
            rs2: 2,
        }));

        assert_eq!(machine.registers.get(3), 2); // rd: 5 - 3 = 2
    }

    #[test]
    fn addi() {
        let mut machine = Machine::new_zeros();

        machine.registers.set(1, 5); // rs1: 5
        machine.execute(Addi(I {
            rd: 3,
            rs: 1,
            imm12: 3,
        }));

        assert_eq!(machine.registers.get(3), 8); // rd: 5 + 3 = 8
    }

    #[test]
    fn slt() {
        let mut machine = Machine::new_zeros();

        machine.registers.set(1, 5); // rs1: 5
        machine.registers.set(2, 3); // rs2: 3
        machine.execute(Slt(R {
            rd: 3,
            rs1: 1,
            rs2: 2,
        }));

        assert_eq!(machine.registers.get(3), 0); // rd: 0, because 5 is not less than 3
    }

    #[test]
    fn slti() {
        let mut machine = Machine::new_zeros();

        machine.registers.set(1, 5); // rs1: 5
        machine.execute(Slti(I {
            rd: 3,
            rs: 1,
            imm12: 7,
        }));

        assert_eq!(machine.registers.get(3), 1); // rd: 1, because 5 is less than 7
    }

    #[test]
    fn sltu() {
        let mut machine = Machine::new_zeros();

        machine.registers.set(1, 2); // rs1: 2
        machine.registers.set(2, 3); // rs2: 3
        machine.execute(Sltu(R {
            rd: 3,
            rs1: 1,
            rs2: 2,
        }));

        assert_eq!(machine.registers.get(3), 1); // rd: 1, because 2 is less than 3
    }

    #[test]
    fn sltiu() {
        let mut machine = Machine::new_zeros();

        machine.registers.set(1, 5); // rs1: 5
        machine.execute(Sltiu(I {
            rd: 3,
            rs: 1,
            imm12: 3,
        }));

        assert_eq!(machine.registers.get(3), 0); // rd: 0 because 5 is not less than 3
    }

    #[test]
    fn lui() {
        let mut machine = Machine::new_zeros();

        machine.execute(Lui(U { rd: 3, imm20: 5 }));

        assert_eq!(machine.registers.get(3), 5 << 12); // rd: 5 << 12
    }

    #[test]
    fn auip() {
        let mut machine = Machine::new_zeros();

        machine.registers.set_pc(0x100);
        machine.execute(Auip(U { rd: 3, imm20: 5 }));

        assert_eq!(machine.registers.get(3), 256 + 5 << 12); // rd: 256 + (5 << 12)
    }

    fn logic_test_machine() -> Machine {
        let mut machine = Machine::new_zeros();

        machine.registers.set(1, 0b1010); // rs1: 0b1010
        machine.registers.set(2, 0b0110); // rs2: 0b0110
        
        machine
    }

    #[test]
    fn and() {
        let mut machine = logic_test_machine();

        machine.execute(And(R {rd: 3, rs1: 1, rs2: 2}));
        assert_eq!(machine.registers.get(3), 0b0010);
    }

    #[test]
    fn or() {
        let mut machine = logic_test_machine();

        machine.execute(Or(R {rd: 3, rs1: 1, rs2: 2}));
        assert_eq!(machine.registers.get(3), 0b1110);
    }

    #[test]
    fn xor() {
        let mut machine = logic_test_machine();

        machine.execute(Xor(R {rd: 3, rs1: 1, rs2: 2}));
        assert_eq!(machine.registers.get(3), 0b1100);
    }

    #[test]
    fn andi() {
        let mut machine = logic_test_machine();

        machine.execute(Andi(I {rd: 3, rs: 1, imm12: 0b0110}));
        assert_eq!(machine.registers.get(3), 0b0010);
    }

    #[test]
    fn ori() {
        let mut machine = logic_test_machine();

        machine.execute(Ori(I {rd: 3, rs: 1, imm12: 0b0110}));
        assert_eq!(machine.registers.get(3), 0b1110);
    }
        
    #[test]
    fn xori() {
        let mut machine = logic_test_machine();

        machine.execute(Xori(I {rd: 3, rs: 1, imm12: 0b0110}));
        assert_eq!(machine.registers.get(3), 0b1100);
    }
        
    #[test]
    fn sll() {
        let mut machine = logic_test_machine();

        machine.execute(Sll(R {rd: 3, rs1: 1, rs2: 2}));
        assert_eq!(machine.registers.get(3), 0b1010000000);
    }
        
    #[test]
    fn slr() {
        let mut machine = logic_test_machine();

        machine.execute(Srl(R {rd: 3, rs1: 1, rs2: 2}));
        assert_eq!(machine.registers.get(3), 0b0);
    }

    #[test]
    fn sra() {
        let mut machine = logic_test_machine();

        machine.execute(Sra(R {rd: 3, rs1: 1, rs2: 2}));
        assert_eq!(machine.registers.get(3), 0b00);
    }

    #[test]
    fn slli() {
        let mut machine = logic_test_machine();

        machine.execute(Slli(I {rd: 3, rs: 1, imm12: 3}));
        assert_eq!(machine.registers.get(3), 0b1010000);
    }    

    #[test]
    fn srli() {
        let mut machine = logic_test_machine();

        machine.execute(Srli(I {rd: 3, rs: 1, imm12: 2}));
        assert_eq!(machine.registers.get(3), 0b10);
    }     

    #[test]
    fn srai() {
        let mut machine = logic_test_machine();

        machine.execute(Srai(I {rd: 3, rs: 1, imm12: 2}));
        assert_eq!(machine.registers.get(3), 0b10);
    }      

        
}
