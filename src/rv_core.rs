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

            Beq(_) => todo!(),
            Bne(_) => todo!(),
            Bge(_) => todo!(),
            Bgeu(_) => todo!(),
            Blt(_) => todo!(),
            Bltu(_) => todo!(),
            Jal(_) => todo!(),
            Jalr(_) => todo!(),
            
            Vaddvv(args) => v::vadd::vv(args, &mut self.registers.v),
            Vaddvx(args) => v::vadd::vx(args, &mut self.registers.v, &self.registers.x),
            Vaddvi(args) => v::vadd::vi(args, &mut self.registers.v),

            Vsubvv(args) => v::vsub::vv(args, &mut self.registers.v),
            Vsubvx(args) => v::vsub::vx(args, &mut self.registers.v, &self.registers.x),

            Vrsubvx(args) => v::vrsub::vx(args, &mut self.registers.v, &self.registers.x),
            Vrsubvi(args) => v::vrsub::vi(args, &mut self.registers.v),
            
            Vminuvv(args) => v::vminu::vv(args, &mut self.registers.v),
            Vminuvx(_) => todo!(),
            
            Vminvv(_) => todo!(),
            Vminvx(_) => todo!(),
            Vmaxuvv(_) => todo!(),
            Vmaxuvx(_) => todo!(),
            Vmaxvv(_) => todo!(),
            Vmaxvx(_) => todo!(),
            Vandvv(_) => todo!(),
            Vandvx(_) => todo!(),
            Vandvi(_) => todo!(),
            Vorvv(_) => todo!(),
            Vorvx(_) => todo!(),
            Vorvi(_) => todo!(),
            Vxorvv(_) => todo!(),
            Vxorvx(_) => todo!(),
            Vxorvi(_) => todo!(),
            Vrgathervv(_) => todo!(),
            Vrgathervx(_) => todo!(),
            Vrgathervi(_) => todo!(),
            Vrgatherei16vv(_) => todo!(),
            Vslideupvx(_) => todo!(),
            Vslideupvi(_) => todo!(),
            Vslidedownvx(_) => todo!(),
            Vslidedownvi(_) => todo!(),
            Vadcvvm(_) => todo!(),
            Vadcvxm(_) => todo!(),
            Vadcvim(_) => todo!(),
            Vmadcvvm(_) => todo!(),
            Vmadcvxm(_) => todo!(),
            Vmadcvim(_) => todo!(),
            Vmadcvv(_) => todo!(),
            Vmadcvx(_) => todo!(),
            Vmadcvi(_) => todo!(),
            Vsbcvvm(_) => todo!(),
            Vsbcvxm(_) => todo!(),
            Vmsbcvvm(_) => todo!(),
            Vmsbcvxm(_) => todo!(),
            Vmsbcvv(_) => todo!(),
            Vmsbcvx(_) => todo!(),
            Vmergevvm(_) => todo!(),
            Vmergevxm(_) => todo!(),
            Vmergevim(_) => todo!(),
            
            Vmvvv(args) => v::vmv::vv(args, &mut self.registers.v),
            Vmvvx(args) => v::vmv::vx(args, &mut self.registers.v, &self.registers.x),
            Vmvvi(args) => v::vmv::vi(args, &mut self.registers.v),

            Vmseqvv(_) => todo!(),
            Vmseqvx(_) => todo!(),
            Vmseqvi(_) => todo!(),
            Vmsnevv(_) => todo!(),
            Vmsnevx(_) => todo!(),
            Vmsnevi(_) => todo!(),
            Vmsltuvv(_) => todo!(),
            Vmsltuvx(_) => todo!(),
            Vmsltvv(_) => todo!(),
            Vmsltvx(_) => todo!(),
            Vmsleuvv(_) => todo!(),
            Vmsleuvx(_) => todo!(),
            Vmsleuvi(_) => todo!(),
            Vmslevv(_) => todo!(),
            Vmslevx(_) => todo!(),
            Vmslevi(_) => todo!(),
            Vmsgtuvx(_) => todo!(),
            Vmsgtuvi(_) => todo!(),
            Vmsgtvx(_) => todo!(),
            Vmsgtvi(_) => todo!(),
            Vsadduvv(_) => todo!(),
            Vsadduvx(_) => todo!(),
            Vsadduvi(_) => todo!(),
            Vsaddvv(_) => todo!(),
            Vsaddvx(_) => todo!(),
            Vsaddvi(_) => todo!(),
            Vssubuvv(_) => todo!(),
            Vssubuvx(_) => todo!(),
            Vssubvv(_) => todo!(),
            Vssubvx(_) => todo!(),
            Vsllvv(_) => todo!(),
            Vsllvx(_) => todo!(),
            Vsllvi(_) => todo!(),
            Vsmulvv(_) => todo!(),
            Vsmulvx(_) => todo!(),
            Vmv1rv(_) => todo!(),
            Vmv2rv(_) => todo!(),
            Vmv4rv(_) => todo!(),
            Vmv8rv(_) => todo!(),
            Vsrlvv(_) => todo!(),
            Vsrlvx(_) => todo!(),
            Vsrlvi(_) => todo!(),
            Vsravv(_) => todo!(),
            Vsravx(_) => todo!(),
            Vsravi(_) => todo!(),
            Vssrlvv(_) => todo!(),
            Vssrlvx(_) => todo!(),
            Vssrlvi(_) => todo!(),
            Vssravv(_) => todo!(),
            Vssravx(_) => todo!(),
            Vssravi(_) => todo!(),
            Vnsrlwv(_) => todo!(),
            Vnsrlwx(_) => todo!(),
            Vnsrlwi(_) => todo!(),
            Vnsrawv(_) => todo!(),
            Vnsrawx(_) => todo!(),
            Vnsrawi(_) => todo!(),
            Vnclipuwv(_) => todo!(),
            Vnclipuwx(_) => todo!(),
            Vnclipuwi(_) => todo!(),
            Vnclipwv(_) => todo!(),
            Vnclipwx(_) => todo!(),
            Vnclipwi(_) => todo!(),
            Vwredsumuvs(_) => todo!(),
            Vwredsumvs(_) => todo!(),
            Vredsumvs(_) => todo!(),
            Vredandvs(_) => todo!(),
            Vredorvs(_) => todo!(),
            Vredxorvs(_) => todo!(),
            Vredminuvs(_) => todo!(),
            Vredminvs(_) => todo!(),
            Vredmaxuvs(_) => todo!(),
            Vredmaxvs(_) => todo!(),
            Vaadduvv(_) => todo!(),
            Vaadduvx(_) => todo!(),
            Vaaddvv(_) => todo!(),
            Vaaddvx(_) => todo!(),
            Vasubuvv(_) => todo!(),
            Vasubuvx(_) => todo!(),
            Vasubvv(_) => todo!(),
            Vasubvx(_) => todo!(),
            Vslide1upvx(_) => todo!(),
            Vslide1downvx(_) => todo!(),
            Vmvxs(_) => todo!(),
            Vcpopm(_) => todo!(),
            Vfirstm(_) => todo!(),
            Vmvsx(_) => todo!(),
            Vzextvf8(_) => todo!(),
            Vsextvf8(_) => todo!(),
            Vzextvf4(_) => todo!(),
            Vsextvf4(_) => todo!(),
            Vzextvf2(_) => todo!(),
            Vsextvf2(_) => todo!(),
            Vmsbfm(_) => todo!(),
            Vmsofm(_) => todo!(),
            Vmsifm(_) => todo!(),
            Viotam(_) => todo!(),
            Vidv(_) => todo!(),
            Vcompressvm(_) => todo!(),
            Vmandnmm(_) => todo!(),
            Vmandmm(_) => todo!(),
            Vmormm(_) => todo!(),
            Vmxormm(_) => todo!(),
            Vmournmm(_) => todo!(),
            Vmnandmm(_) => todo!(),
            Vmnormm(_) => todo!(),
            Vmxnormm(_) => todo!(),
            Vdivuvv(_) => todo!(),
            Vdivuvx(_) => todo!(),
            Vdivvv(_) => todo!(),
            Vdivvx(_) => todo!(),
            Vremuvv(_) => todo!(),
            Vremuvx(_) => todo!(),
            Vremvv(_) => todo!(),
            Vremvx(_) => todo!(),
            Vmulhuvv(_) => todo!(),
            Vmulhuvx(_) => todo!(),
            Vmulvv(_) => todo!(),
            Vmulvx(_) => todo!(),
            Vmulhsuvv(_) => todo!(),
            Vmulhsuvx(_) => todo!(),
            Vmulhvv(_) => todo!(),
            Vmulhvx(_) => todo!(),
            Vmaddvv(_) => todo!(),
            Vmaddvx(_) => todo!(),
            Vnmsubvv(_) => todo!(),
            Vnmsubvx(_) => todo!(),
            Vmaccvv(_) => todo!(),
            Vmaccvx(_) => todo!(),
            Vnmsacvv(_) => todo!(),
            Vnmsacvx(_) => todo!(),
            Vwadduvv(_) => todo!(),
            Vwadduvx(_) => todo!(),
            Vwaddvv(_) => todo!(),
            Vwaddvx(_) => todo!(),
            Vwsubuvv(_) => todo!(),
            Vwsubuvx(_) => todo!(),
            Vwsubvv(_) => todo!(),
            Vwsubvx(_) => todo!(),
            Vwadduwv(_) => todo!(),
            Vwadduwx(_) => todo!(),
            Vwaddwv(_) => todo!(),
            Vwaddwx(_) => todo!(),
            Vwsubuwv(_) => todo!(),
            Vwsubuwx(_) => todo!(),
            Vwsubwv(_) => todo!(),
            Vwsubwx(_) => todo!(),
            Vwmuluwv(_) => todo!(),
            Vwmuluwx(_) => todo!(),
            Vwmulsuwv(_) => todo!(),
            Vwmulsuwx(_) => todo!(),
            Vwmulwv(_) => todo!(),
            Vwmulwx(_) => todo!(),
            Vwmaccuwv(_) => todo!(),
            Vwmaccuwx(_) => todo!(),
            Vwmaccwv(_) => todo!(),
            Vwmaccwx(_) => todo!(),
            Vwmaccuswx(_) => todo!(),
            Vwmaccsuwv(_) => todo!(),
            Vwmaccsuwx(_) => todo!(),
            Vfaddvv(_) => todo!(),
            Vfaddvf(_) => todo!(),
            Vfredusumvs(_) => todo!(),
            Vfsubvv(_) => todo!(),
            Vfsubvf(_) => todo!(),
            Vfredosumvs(_) => todo!(),
            Vfminvv(_) => todo!(),
            Vfminvf(_) => todo!(),
            Vfredminvs(_) => todo!(),
            Vfmaxvv(_) => todo!(),
            Vfmaxvf(_) => todo!(),
            Vfredmaxvs(_) => todo!(),
            Vfsgnjvv(_) => todo!(),
            Vfsgnjvf(_) => todo!(),
            Vfsgnjnvv(_) => todo!(),
            Vfsgnjnvf(_) => todo!(),
            Vfsgnjxvv(_) => todo!(),
            Vfsgnjxvf(_) => todo!(),
            Vfslide1upvf(_) => todo!(),
            Vfslide1downvf(_) => todo!(),
            Vfmvfs(_) => todo!(),
            Vfmvsf(_) => todo!(),
            Vfcvtxufv(_) => todo!(),
            Vfcvtxfv(_) => todo!(),
            Vfcvtfxuv(_) => todo!(),
            Vfcvtfxv(_) => todo!(),
            VfcvtRtzxufv(_) => todo!(),
            VfcvtRtzxfv(_) => todo!(),
            Vfwcvtxufv(_) => todo!(),
            Vfwcvtxfv(_) => todo!(),
            Vfwcvtfxuv(_) => todo!(),
            Vfwcvtfxv(_) => todo!(),
            Vfwcvtffv(_) => todo!(),
            VfwcvtRtzxufv(_) => todo!(),
            VfwcvtRtzxfv(_) => todo!(),
            Vfncvtxufw(_) => todo!(),
            Vfncvtxfw(_) => todo!(),
            Vfncvtfxuw(_) => todo!(),
            Vfncvtfxw(_) => todo!(),
            Vfncvtffw(_) => todo!(),
            VfncvtRodffw(_) => todo!(),
            VfncvtRtzxufw(_) => todo!(),
            VfncvtRtzxfw(_) => todo!(),
            Vfsqrtv(_) => todo!(),
            Vfrsqrt7v(_) => todo!(),
            Vfrec7v(_) => todo!(),
            Vfclassv(_) => todo!(),
            Vfmergevfm(_) => todo!(),
            Vfmvvf(_) => todo!(),
            Vmfeqvv(_) => todo!(),
            Vmfeqvf(_) => todo!(),
            Vmflevv(_) => todo!(),
            Vmflevf(_) => todo!(),
            Vmfltvv(_) => todo!(),
            Vmfltvf(_) => todo!(),
            Vmfnevv(_) => todo!(),
            Vmfnevf(_) => todo!(),
            Vmfgtvf(_) => todo!(),
            Vmfgevf(_) => todo!(),
            Vfdivvv(_) => todo!(),
            Vfdivvf(_) => todo!(),
            Vfrdirvf(_) => todo!(),
            Vfmulvv(_) => todo!(),
            Vfmulvf(_) => todo!(),
            Vfrsubvf(_) => todo!(),
            Vfmaddvv(_) => todo!(),
            Vfmaddvf(_) => todo!(),
            Vfnmaddvv(_) => todo!(),
            Vfnmaddvf(_) => todo!(),
            Vfmsubvv(_) => todo!(),
            Vfmsubvf(_) => todo!(),
            Vfnmsubvv(_) => todo!(),
            Vfnmsubvf(_) => todo!(),
            Vfmaccvv(_) => todo!(),
            Vfmaccvf(_) => todo!(),
            Vfnmaccvv(_) => todo!(),
            Vfnmaccvf(_) => todo!(),
            Vfmsacvv(_) => todo!(),
            Vfmsacvf(_) => todo!(),
            Vfnmsacvv(_) => todo!(),
            Vfnmsacvf(_) => todo!(),
            Vfwaddvv(_) => todo!(),
            Vfwaddvf(_) => todo!(),
            Vfwredusumvs(_) => todo!(),
            Vfwsubvv(_) => todo!(),
            Vfwsubvf(_) => todo!(),
            Vfwredosumvs(_) => todo!(),
            Vfwaddwv(_) => todo!(),
            Vfwaddwf(_) => todo!(),
            Vfwsubwv(_) => todo!(),
            Vfwsubwf(_) => todo!(),
            Vfwmulvv(_) => todo!(),
            Vfwmulvf(_) => todo!(),
            Vfwmaccvv(_) => todo!(),
            Vfwmaccvf(_) => todo!(),
            Vfwnmaccvv(_) => todo!(),
            Vfwnmaccvf(_) => todo!(),
            Vfwmsacvv(_) => todo!(),
            Vfwmsacvf(_) => todo!(),
            Vfwnmsacvv(_) => todo!(),
            Vfwnmsacvf(_) => todo!(),

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
