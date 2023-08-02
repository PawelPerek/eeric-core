mod instruction;
mod memory;
mod registers;

use instruction::{format, Instruction};
use memory::Memory;
use registers::Registers;

use crate::rv_core::instruction::executor::base::sll;

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
        use Instruction::*;
        use instruction::executor::*;

        match input {
            Add(args)   => base::add(args, &mut self.registers.x),
            Sub(args)   => base::sub(args, &mut self.registers.x),
            Addi(args)  => base::addi(args, &mut self.registers.x),
            Slt(args)   => base::slt(args, &mut self.registers.x),
            Slti(args)  => base::slti(args, &mut self.registers.x),
            Sltu(args)  => base::sltu(args, &mut self.registers.x),
            Sltiu(args) => base::sltiu(args, &mut self.registers.x),
            Lui(args)   => base::lui(args, &mut self.registers.x),
            Auipc(args) => base::auipc(args, &mut self.registers.x, self.registers.pc),

            And(args) => base::and(args, &mut self.registers.x),
            Or(args) => base::or(args, &mut self.registers.x),
            Xor(args) => base::xor(args, &mut self.registers.x),
            Andi(args) => base::andi(args, &mut self.registers.x),
            Ori(args) => base::ori(args, &mut self.registers.x),
            Xori(args) => base::xori(args, &mut self.registers.x),
            Sll(args) => base::sll(args, &mut self.registers.x),
            Srl(args) => base::srl(args, &mut self.registers.x),
            Sra(args) => base::sra(args, &mut self.registers.x),
            Slli(args) => base::slli(args, &mut self.registers.x),
            Srli(args) => base::srli(args, &mut self.registers.x),
            Srai(args) => base::srai(args, &mut self.registers.x),

            Ld(args) => base::ld(args, &mut self.registers.x, &self.memory),
            Lw(args) => base::lw(args, &mut self.registers.x, &self.memory),
            Lwu(args) => base::lwu(args, &mut self.registers.x, &self.memory),
            Lh(args) => base::lh(args, &mut self.registers.x, &self.memory),
            Lhu(args) => base::lhu(args, &mut self.registers.x, &self.memory),
            Lb(args) => base::lb(args, &mut self.registers.x, &self.memory),
            Lbu(args) => base::lbu(args, &mut self.registers.x, &self.memory),
            Sd(args) => base::sd(args, &self.registers.x, &mut self.memory),
            Sw(args) => base::sw(args, &self.registers.x, &mut self.memory),
            Sh(args) => base::sh(args, &self.registers.x, &mut self.memory),
            Sb(args) => base::sb(args, &self.registers.x, &mut self.memory),

            Vaddvv(args) => v::vadd::vv(args, &mut self.registers.v),
            Vaddvx(args) => v::vadd::vx(args, &mut self.registers.v, &self.registers.x),
            Vaddvi(args) => v::vadd::vi(args, &mut self.registers.v)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use format::base::*;
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
        machine.execute(Auipc(U { rd: 3, imm20: 5 }));

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
