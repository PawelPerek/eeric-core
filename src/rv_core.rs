mod instruction;
mod memory;
mod registers;

use instruction::{format, Instruction};
use memory::Memory;
use registers::Registers;

struct RvCore {
    registers: Registers,
    memory: Memory
}

impl RvCore {
    fn new_zeroed() -> Self {
        RvCore {
            registers: Default::default(),
            memory: Default::default(),
        }
    }

    fn new(registers: Registers, memory: Memory) -> Self {
        Self { registers, memory }
    }

    fn execute(&mut self, input: Instruction) {
        use format::{
            base::*, 
            vector::*
        };
        use Instruction::*;
        use instruction::executor::*;

        match input {
            Add(R { rd, rs1, rs2 }) => {
                let rs1 = self.registers.x[rs1];
                let rs2 = self.registers.x[rs2];
                self.registers.x[rd] = rs1 + rs2;
            }
            Sub(R { rd, rs1, rs2 }) => {
                let rs1 = self.registers.x[rs1];
                let rs2 = self.registers.x[rs2];
                self.registers.x[rd] = rs1 - rs2;
            }
            Addi(I { rd, rs, imm12 }) => {
                let rs = self.registers.x[rs];
                self.registers.x[rd] = rs + imm12;
            }
            Slt(R { rd, rs1, rs2 }) => {
                let rs1 = self.registers.x[rs1] as i64;
                let rs2 = self.registers.x[rs2] as i64;
                self.registers.x[rd] = if rs1 < rs2 { 1 } else { 0 };
            }
            Slti(I { rd, rs, imm12 }) => {
                let rs = self.registers.x[rs];
                self.registers.x[rd] = if rs < imm12 { 1 } else { 0 };
            }
            Sltu(R { rd, rs1, rs2 }) => {
                let rs1 = self.registers.x[rs1];
                let rs2 = self.registers.x[rs2];
                self.registers.x[rd] = if rs1 < rs2 { 1 } else { 0 };
            }
            Sltiu(I { rd, rs, imm12 }) => {
                let rs = self.registers.x[rs] as i64;
                self.registers.x[rd] = if rs < imm12 as i64 { 1 } else { 0 };
            }
            Lui(U { rd, imm20 }) => self.registers.x[rd] = imm20 << 12,
            Auip(U { rd, imm20 }) => {
                let pc = self.registers.pc;
                self.registers.x[rd] = pc + imm20 << 12;
            }

            And(R { rd, rs1, rs2 }) => {
                let rs1 = self.registers.x[rs1];
                let rs2 = self.registers.x[rs2];
                self.registers.x[rd] = rs1 & rs2;
            }
            Or(R { rd, rs1, rs2 }) => {
                let rs1 = self.registers.x[rs1];
                let rs2 = self.registers.x[rs2];
                self.registers.x[rd] = rs1 | rs2;
            }
            Xor(R { rd, rs1, rs2 }) => {
                let rs1 = self.registers.x[rs1];
                let rs2 = self.registers.x[rs2];
                self.registers.x[rd] = rs1 ^ rs2;
            }
            Andi(I { rd, rs, imm12 }) => {
                let rs = self.registers.x[rs];
                self.registers.x[rd] = rs & imm12;
            }
            Ori(I { rd, rs, imm12 }) => {
                let rs = self.registers.x[rs];
                self.registers.x[rd] = rs | imm12;
            }
            Xori(I { rd, rs, imm12 }) => {
                let rs = self.registers.x[rs];
                self.registers.x[rd] = rs ^ imm12;
            }
            Sll(R { rd, rs1, rs2 }) => {
                let rs1 = self.registers.x[rs1];
                let rs2 = self.registers.x[rs2] & 0b11111;
                self.registers.x[rd] = rs1 << rs2;
            }
            Srl(R { rd, rs1, rs2 }) => {
                let rs1 = self.registers.x[rs1];
                let rs2 = self.registers.x[rs2] & 0b11111;
                self.registers.x[rd] = rs1 >> rs2;
            }
            Sra(R { rd, rs1, rs2 }) => {
                let rs1 = self.registers.x[rs1] as i64;
                let rs2 = self.registers.x[rs2] as i64 & 0b11111;
                self.registers.x[rd] = (rs1 >> rs2) as u64;
            }
            Slli(I { rd, rs, imm12 }) => {
                let rs = self.registers.x[rs];
                let shamt = imm12 & 0b11111;
                self.registers.x[rd] = rs << shamt;
            }
            Srli(I { rd, rs, imm12 }) => {
                let rs = self.registers.x[rs];
                let shamt = imm12 & 0b11111;
                self.registers.x[rd] = rs >> shamt;
            }
            Srai(I { rd, rs, imm12 }) => {
                let rs = self.registers.x[rs] as i64;
                let shamt = imm12 & 0b11111;
                self.registers.x[rd] = (rs >> shamt) as u64;
            },
            Ld(I {rd, rs, imm12}) => {
                let address = (self.registers.x[rs] + imm12) as usize;
                let value = u64::from_le_bytes(self.memory.0[address .. address + 8].try_into().unwrap());
                self.registers.x[rd] = value;
            },
            Lw(I {rd, rs, imm12}) => {
                let address = (self.registers.x[rs] + imm12) as usize;
                let value = u32::from_le_bytes(self.memory.0[address .. address + 4].try_into().unwrap());
                self.registers.x[rd] = value as u64;
            },
            Lh(I {rd, rs, imm12}) => {
                let address: usize = (self.registers.x[rs] + imm12) as usize;
                let value = u16::from_le_bytes(self.memory.0[address .. address + 2].try_into().unwrap());
                self.registers.x[rd] = value as u64;
            },
            Lb(I {rd, rs, imm12}) => {
                let address: usize = (self.registers.x[rs] + imm12) as usize;
                let value = self.memory.0[address];
                self.registers.x[rd] = value as u64;
            },
            Lhu(_) => todo!(),
            Lbu(_) => todo!(),
            Sw(_) => todo!(),
            Sh(_) => todo!(),
            Sb(_) => todo!(),
            Lwu(_) => todo!(),
            Sd(_) => todo!(),

            Vaddvv(args) => vadd::vv(args, &mut self.registers.v),
            Vaddvx(args) => vadd::vx(args, &mut self.registers.v, &mut self.registers.x),
            Vaddvi(args) => vadd::vi(args, &mut self.registers.v)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use format::{
        base::*, 
        vector::*
    };
    use Instruction::*;

    #[test]
    fn add() {
        let mut machine = RvCore::new_zeroed();

        machine.registers.x[1] = 2; // rs1: 2
        machine.registers.x[2] = 3; // rs2: 3
        machine.execute(Add(R {
            rd: 3,
            rs1: 1,
            rs2: 2,
        }));

        assert_eq!(machine.registers.x[3], 5); // rd: 2 + 3 = 5
    }

    #[test]
    fn sub() {
        let mut machine = RvCore::new_zeroed();

        machine.registers.x[1] = 5; // rs1: 5
        machine.registers.x[2] = 3; // rs2: 3
        machine.execute(Sub(R {
            rd: 3,
            rs1: 1,
            rs2: 2,
        }));

        assert_eq!(machine.registers.x[3], 2); // rd: 5 - 3 = 2
    }

    #[test]
    fn addi() {
        let mut machine = RvCore::new_zeroed();

        machine.registers.x[1] = 5; // rs1: 5
        machine.execute(Addi(I {
            rd: 3,
            rs: 1,
            imm12: 3,
        }));

        assert_eq!(machine.registers.x[3], 8); // rd: 5 + 3 = 8
    }

    #[test]
    fn slt() {
        let mut machine = RvCore::new_zeroed();

        machine.registers.x[1] = 5; // rs1: 5
        machine.registers.x[2] = 3; // rs2: 3
        machine.execute(Slt(R {
            rd: 3,
            rs1: 1,
            rs2: 2,
        }));

        assert_eq!(machine.registers.x[3], 0); // rd: 0, because 5 is not less than 3
    }

    #[test]
    fn slti() {
        let mut machine = RvCore::new_zeroed();

        machine.registers.x[1] = 5; // rs1: 5
        machine.execute(Slti(I {
            rd: 3,
            rs: 1,
            imm12: 7,
        }));

        assert_eq!(machine.registers.x[3], 1); // rd: 1, because 5 is less than 7
    }

    #[test]
    fn sltu() {
        let mut machine = RvCore::new_zeroed();

        machine.registers.x[1] = 2; // rs1: 2
        machine.registers.x[2] = 3; // rs2: 3
        machine.execute(Sltu(R {
            rd: 3,
            rs1: 1,
            rs2: 2,
        }));

        assert_eq!(machine.registers.x[3], 1); // rd: 1, because 2 is less than 3
    }

    #[test]
    fn sltiu() {
        let mut machine = RvCore::new_zeroed();

        machine.registers.x[1] = 5; // rs1: 5
        machine.execute(Sltiu(I {
            rd: 3,
            rs: 1,
            imm12: 3,
        }));

        assert_eq!(machine.registers.x[3], 0); // rd: 0 because 5 is not less than 3
    }

    #[test]
    fn lui() {
        let mut machine = RvCore::new_zeroed();

        machine.execute(Lui(U { rd: 3, imm20: 5 }));

        assert_eq!(machine.registers.x[3], 5 << 12); // rd: 5 << 12
    }

    #[test]
    fn auip() {
        let mut machine = RvCore::new_zeroed();

        machine.registers.pc = 0x100;
        machine.execute(Auip(U { rd: 3, imm20: 5 }));

        assert_eq!(machine.registers.x[3], 256 + 5 << 12); // rd: 256 + (5 << 12)
    }

    fn logic_test_machine() -> RvCore {
        let mut machine = RvCore::new_zeroed();

        machine.registers.x[1] = 0b1010; // rs1: 0b1010
        machine.registers.x[2] = 0b0110; // rs2: 0b0110
        
        machine
    }

    #[test]
    fn and() {
        let mut machine = logic_test_machine();

        machine.execute(And(R {rd: 3, rs1: 1, rs2: 2}));
        assert_eq!(machine.registers.x[3], 0b0010);
    }

    #[test]
    fn or() {
        let mut machine = logic_test_machine();

        machine.execute(Or(R {rd: 3, rs1: 1, rs2: 2}));
        assert_eq!(machine.registers.x[3], 0b1110);
    }

    #[test]
    fn xor() {
        let mut machine = logic_test_machine();

        machine.execute(Xor(R {rd: 3, rs1: 1, rs2: 2}));
        assert_eq!(machine.registers.x[3], 0b1100);
    }

    #[test]
    fn andi() {
        let mut machine = logic_test_machine();

        machine.execute(Andi(I {rd: 3, rs: 1, imm12: 0b0110}));
        assert_eq!(machine.registers.x[3], 0b0010);
    }

    #[test]
    fn ori() {
        let mut machine = logic_test_machine();

        machine.execute(Ori(I {rd: 3, rs: 1, imm12: 0b0110}));
        assert_eq!(machine.registers.x[3], 0b1110);
    }
        
    #[test]
    fn xori() {
        let mut machine = logic_test_machine();

        machine.execute(Xori(I {rd: 3, rs: 1, imm12: 0b0110}));
        assert_eq!(machine.registers.x[3], 0b1100);
    }
        
    #[test]
    fn sll() {
        let mut machine = logic_test_machine();

        machine.execute(Sll(R {rd: 3, rs1: 1, rs2: 2}));
        assert_eq!(machine.registers.x[3], 0b1010000000);
    }
        
    #[test]
    fn slr() {
        let mut machine = logic_test_machine();

        machine.execute(Srl(R {rd: 3, rs1: 1, rs2: 2}));
        assert_eq!(machine.registers.x[3], 0b0);
    }

    #[test]
    fn sra() {
        let mut machine = logic_test_machine();

        machine.execute(Sra(R {rd: 3, rs1: 1, rs2: 2}));
        assert_eq!(machine.registers.x[3], 0b00);
    }

    #[test]
    fn slli() {
        let mut machine = logic_test_machine();

        machine.execute(Slli(I {rd: 3, rs: 1, imm12: 3}));
        assert_eq!(machine.registers.x[3], 0b1010000);
    }    

    #[test]
    fn srli() {
        let mut machine = logic_test_machine();

        machine.execute(Srli(I {rd: 3, rs: 1, imm12: 2}));
        assert_eq!(machine.registers.x[3], 0b10);
    }     

    #[test]
    fn srai() {
        let mut machine = logic_test_machine();

        machine.execute(Srai(I {rd: 3, rs: 1, imm12: 2}));
        assert_eq!(machine.registers.x[3], 0b10);
    }      

        
}
