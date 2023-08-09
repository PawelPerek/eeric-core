mod instruction;
mod memory;
mod registers;

pub use instruction::Instruction;
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

            Beq(args) => todo!(),
            Bne(args) => todo!(),
            Bge(args) => todo!(),
            Bgeu(args) => todo!(),
            Blt(args) => todo!(),
            Bltu(args) => todo!(),
            Jal(args) => todo!(),
            Jalr(args) => todo!(),
            
            Vaddvv(args) => v::vadd::vv(args, &mut self.registers.v),
            Vaddvx(args) => v::vadd::vx(args, &mut self.registers.v, &self.registers.x),
            Vaddvi(args) => v::vadd::vi(args, &mut self.registers.v),

            Vsubvv(args) => v::vsub::vv(args, &mut self.registers.v),
            Vsubvx(args) => v::vsub::vx(args, &mut self.registers.v, &self.registers.x),

            Vrsubvx(args) => v::vrsub::vx(args, &mut self.registers.v, &self.registers.x),
            Vrsubvi(args) => v::vrsub::vi(args, &mut self.registers.v),
            
            Vminuvv(args) => v::vminu::vv(args, &mut self.registers.v),
            Vminuvx(args) => v::vminu::vx(args, &mut self.registers.v, &self.registers.x),
            
            Vminvv(args) => v::vmin::vv(args, &mut self.registers.v),
            Vminvx(args) => v::vmin::vx(args, &mut self.registers.v, &self.registers.x),

            Vmaxuvv(args) => v::vmaxu::vv(args, &mut self.registers.v),
            Vmaxuvx(args) => v::vmaxu::vx(args, &mut self.registers.v, &self.registers.x),

            Vmaxvv(args) => v::vmax::vv(args, &mut self.registers.v),
            Vmaxvx(args) => v::vmax::vx(args, &mut self.registers.v, &self.registers.x),

            Vandvv(args) => v::vand::vv(args, &mut self.registers.v),
            Vandvx(args) => v::vand::vx(args, &mut self.registers.v, &self.registers.x),
            Vandvi(args) => v::vand::vi(args, &mut self.registers.v),

            Vorvv(args) => v::vor::vv(args, &mut self.registers.v),
            Vorvx(args) => v::vor::vx(args, &mut self.registers.v, &self.registers.x),
            Vorvi(args) => v::vor::vi(args, &mut self.registers.v),

            Vxorvv(args) => v::vxor::vv(args, &mut self.registers.v),
            Vxorvx(args) => v::vxor::vx(args, &mut self.registers.v, &self.registers.x),
            Vxorvi(args) => v::vxor::vi(args, &mut self.registers.v),

            Vrgathervv(args) => todo!(),
            Vrgathervx(args) => todo!(),
            Vrgathervi(args) => todo!(),

            Vrgatherei16vv(args) => todo!(),

            Vslideupvx(args) => todo!(),
            Vslideupvi(args) => todo!(),

            Vslidedownvx(args) => todo!(),
            Vslidedownvi(args) => todo!(),

            Vadcvvm(args) => todo!(),
            Vadcvxm(args) => todo!(),
            Vadcvim(args) => todo!(),

            Vmadcvvm(args) => todo!(),
            Vmadcvxm(args) => todo!(),
            Vmadcvim(args) => todo!(),

            Vmadcvv(args) => todo!(),
            Vmadcvx(args) => todo!(),
            Vmadcvi(args) => todo!(),

            Vsbcvvm(args) => todo!(),
            Vsbcvxm(args) => todo!(),

            Vmsbcvvm(args) => todo!(),
            Vmsbcvxm(args) => todo!(),
            Vmsbcvv(args) => todo!(),
            Vmsbcvx(args) => todo!(),

            Vmergevvm(args) => todo!(),
            Vmergevxm(args) => todo!(),
            Vmergevim(args) => todo!(),
            
            Vmvvv(args) => v::vmv::vv(args, &mut self.registers.v),
            Vmvvx(args) => v::vmv::vx(args, &mut self.registers.v, &self.registers.x),
            Vmvvi(args) => v::vmv::vi(args, &mut self.registers.v),

            Vmseqvv(args) => todo!(),
            Vmseqvx(args) => todo!(),
            Vmseqvi(args) => todo!(),
            Vmsnevv(args) => todo!(),
            Vmsnevx(args) => todo!(),
            Vmsnevi(args) => todo!(),
            Vmsltuvv(args) => todo!(),
            Vmsltuvx(args) => todo!(),
            Vmsltvv(args) => todo!(),
            Vmsltvx(args) => todo!(),
            Vmsleuvv(args) => todo!(),
            Vmsleuvx(args) => todo!(),
            Vmsleuvi(args) => todo!(),
            Vmslevv(args) => todo!(),
            Vmslevx(args) => todo!(),
            Vmslevi(args) => todo!(),
            Vmsgtuvx(args) => todo!(),
            Vmsgtuvi(args) => todo!(),
            Vmsgtvx(args) => todo!(),
            Vmsgtvi(args) => todo!(),
            Vsadduvv(args) => todo!(),
            Vsadduvx(args) => todo!(),
            Vsadduvi(args) => todo!(),
            Vsaddvv(args) => todo!(),
            Vsaddvx(args) => todo!(),
            Vsaddvi(args) => todo!(),
            Vssubuvv(args) => todo!(),
            Vssubuvx(args) => todo!(),
            Vssubvv(args) => todo!(),
            Vssubvx(args) => todo!(),
            Vsllvv(args) => todo!(),
            Vsllvx(args) => todo!(),
            Vsllvi(args) => todo!(),
            Vsmulvv(args) => todo!(),
            Vsmulvx(args) => todo!(),
            Vmv1rv(args) => todo!(),
            Vmv2rv(args) => todo!(),
            Vmv4rv(args) => todo!(),
            Vmv8rv(args) => todo!(),
            Vsrlvv(args) => todo!(),
            Vsrlvx(args) => todo!(),
            Vsrlvi(args) => todo!(),
            Vsravv(args) => todo!(),
            Vsravx(args) => todo!(),
            Vsravi(args) => todo!(),
            Vssrlvv(args) => todo!(),
            Vssrlvx(args) => todo!(),
            Vssrlvi(args) => todo!(),
            Vssravv(args) => todo!(),
            Vssravx(args) => todo!(),
            Vssravi(args) => todo!(),
            Vnsrlwv(args) => todo!(),
            Vnsrlwx(args) => todo!(),
            Vnsrlwi(args) => todo!(),
            Vnsrawv(args) => todo!(),
            Vnsrawx(args) => todo!(),
            Vnsrawi(args) => todo!(),
            Vnclipuwv(args) => todo!(),
            Vnclipuwx(args) => todo!(),
            Vnclipuwi(args) => todo!(),
            Vnclipwv(args) => todo!(),
            Vnclipwx(args) => todo!(),
            Vnclipwi(args) => todo!(),
            Vwredsumuvs(args) => todo!(),
            Vwredsumvs(args) => todo!(),
            Vredsumvs(args) => todo!(),
            Vredandvs(args) => todo!(),
            Vredorvs(args) => todo!(),
            Vredxorvs(args) => todo!(),
            Vredminuvs(args) => todo!(),
            Vredminvs(args) => todo!(),
            Vredmaxuvs(args) => todo!(),
            Vredmaxvs(args) => todo!(),
            Vaadduvv(args) => todo!(),
            Vaadduvx(args) => todo!(),
            Vaaddvv(args) => todo!(),
            Vaaddvx(args) => todo!(),
            Vasubuvv(args) => todo!(),
            Vasubuvx(args) => todo!(),
            Vasubvv(args) => todo!(),
            Vasubvx(args) => todo!(),
            Vslide1upvx(args) => todo!(),
            Vslide1downvx(args) => todo!(),
            Vmvxs(args) => todo!(),
            Vcpopm(args) => todo!(),
            Vfirstm(args) => todo!(),
            Vmvsx(args) => todo!(),
            Vzextvf8(args) => todo!(),
            Vsextvf8(args) => todo!(),
            Vzextvf4(args) => todo!(),
            Vsextvf4(args) => todo!(),
            Vzextvf2(args) => todo!(),
            Vsextvf2(args) => todo!(),
            Vmsbfm(args) => todo!(),
            Vmsofm(args) => todo!(),
            Vmsifm(args) => todo!(),
            Viotam(args) => todo!(),
            Vidv(args) => todo!(),
            Vcompressvm(args) => todo!(),
            Vmandnmm(args) => todo!(),
            Vmandmm(args) => todo!(),
            Vmormm(args) => todo!(),
            Vmxormm(args) => todo!(),
            Vmournmm(args) => todo!(),
            Vmnandmm(args) => todo!(),
            Vmnormm(args) => todo!(),
            Vmxnormm(args) => todo!(),
            Vdivuvv(args) => todo!(),
            Vdivuvx(args) => todo!(),
            Vdivvv(args) => todo!(),
            Vdivvx(args) => todo!(),
            Vremuvv(args) => todo!(),
            Vremuvx(args) => todo!(),
            Vremvv(args) => todo!(),
            Vremvx(args) => todo!(),
            Vmulhuvv(args) => todo!(),
            Vmulhuvx(args) => todo!(),
            Vmulvv(args) => todo!(),
            Vmulvx(args) => todo!(),
            Vmulhsuvv(args) => todo!(),
            Vmulhsuvx(args) => todo!(),
            Vmulhvv(args) => todo!(),
            Vmulhvx(args) => todo!(),
            Vmaddvv(args) => todo!(),
            Vmaddvx(args) => todo!(),
            Vnmsubvv(args) => todo!(),
            Vnmsubvx(args) => todo!(),
            Vmaccvv(args) => todo!(),
            Vmaccvx(args) => todo!(),
            Vnmsacvv(args) => todo!(),
            Vnmsacvx(args) => todo!(),
            Vwadduvv(args) => todo!(),
            Vwadduvx(args) => todo!(),
            Vwaddvv(args) => todo!(),
            Vwaddvx(args) => todo!(),
            Vwsubuvv(args) => todo!(),
            Vwsubuvx(args) => todo!(),
            Vwsubvv(args) => todo!(),
            Vwsubvx(args) => todo!(),
            Vwadduwv(args) => todo!(),
            Vwadduwx(args) => todo!(),
            Vwaddwv(args) => todo!(),
            Vwaddwx(args) => todo!(),
            Vwsubuwv(args) => todo!(),
            Vwsubuwx(args) => todo!(),
            Vwsubwv(args) => todo!(),
            Vwsubwx(args) => todo!(),
            Vwmuluwv(args) => todo!(),
            Vwmuluwx(args) => todo!(),
            Vwmulsuwv(args) => todo!(),
            Vwmulsuwx(args) => todo!(),
            Vwmulwv(args) => todo!(),
            Vwmulwx(args) => todo!(),
            Vwmaccuwv(args) => todo!(),
            Vwmaccuwx(args) => todo!(),
            Vwmaccwv(args) => todo!(),
            Vwmaccwx(args) => todo!(),
            Vwmaccuswx(args) => todo!(),
            Vwmaccsuwv(args) => todo!(),
            Vwmaccsuwx(args) => todo!(),
            Vfaddvv(args) => todo!(),
            Vfaddvf(args) => todo!(),
            Vfredusumvs(args) => todo!(),
            Vfsubvv(args) => todo!(),
            Vfsubvf(args) => todo!(),
            Vfredosumvs(args) => todo!(),
            Vfminvv(args) => todo!(),
            Vfminvf(args) => todo!(),
            Vfredminvs(args) => todo!(),
            Vfmaxvv(args) => todo!(),
            Vfmaxvf(args) => todo!(),
            Vfredmaxvs(args) => todo!(),
            Vfsgnjvv(args) => todo!(),
            Vfsgnjvf(args) => todo!(),
            Vfsgnjnvv(args) => todo!(),
            Vfsgnjnvf(args) => todo!(),
            Vfsgnjxvv(args) => todo!(),
            Vfsgnjxvf(args) => todo!(),
            Vfslide1upvf(args) => todo!(),
            Vfslide1downvf(args) => todo!(),
            Vfmvfs(args) => todo!(),
            Vfmvsf(args) => todo!(),
            Vfcvtxufv(args) => todo!(),
            Vfcvtxfv(args) => todo!(),
            Vfcvtfxuv(args) => todo!(),
            Vfcvtfxv(args) => todo!(),
            VfcvtRtzxufv(args) => todo!(),
            VfcvtRtzxfv(args) => todo!(),
            Vfwcvtxufv(args) => todo!(),
            Vfwcvtxfv(args) => todo!(),
            Vfwcvtfxuv(args) => todo!(),
            Vfwcvtfxv(args) => todo!(),
            Vfwcvtffv(args) => todo!(),
            VfwcvtRtzxufv(args) => todo!(),
            VfwcvtRtzxfv(args) => todo!(),
            Vfncvtxufw(args) => todo!(),
            Vfncvtxfw(args) => todo!(),
            Vfncvtfxuw(args) => todo!(),
            Vfncvtfxw(args) => todo!(),
            Vfncvtffw(args) => todo!(),
            VfncvtRodffw(args) => todo!(),
            VfncvtRtzxufw(args) => todo!(),
            VfncvtRtzxfw(args) => todo!(),
            Vfsqrtv(args) => todo!(),
            Vfrsqrt7v(args) => todo!(),
            Vfrec7v(args) => todo!(),
            Vfclassv(args) => todo!(),
            Vfmergevfm(args) => todo!(),
            Vfmvvf(args) => todo!(),
            
            Vmfeqvv(args) => todo!(),
            Vmfeqvf(args) => todo!(),

            Vmflevv(args) => todo!(),
            Vmflevf(args) => todo!(),

            Vmfltvv(args) => todo!(),
            Vmfltvf(args) => todo!(),

            Vmfnevv(args) => todo!(),
            Vmfnevf(args) => todo!(),

            Vmfgtvf(args) => todo!(),

            Vmfgevf(args) => todo!(),

            Vfdivvv(args) => todo!(),
            Vfdivvf(args) => todo!(),

            Vfrdirvf(args) => todo!(),

            Vfmulvv(args) => todo!(),
            Vfmulvf(args) => todo!(),

            Vfrsubvf(args) => todo!(),

            Vfmaddvv(args) => todo!(),
            Vfmaddvf(args) => todo!(),

            Vfnmaddvv(args) => todo!(),
            Vfnmaddvf(args) => todo!(),

            Vfmsubvv(args) => todo!(),
            Vfmsubvf(args) => todo!(),

            Vfnmsubvv(args) => todo!(),
            Vfnmsubvf(args) => todo!(),

            Vfmaccvv(args) => todo!(),
            Vfmaccvf(args) => todo!(),

            Vfnmaccvv(args) => todo!(),
            Vfnmaccvf(args) => todo!(),

            Vfmsacvv(args) => todo!(),
            Vfmsacvf(args) => todo!(),

            Vfnmsacvv(args) => todo!(),
            Vfnmsacvf(args) => todo!(),

            Vfwaddvv(args) => todo!(),
            Vfwaddvf(args) => todo!(),

            Vfwredusumvs(args) => todo!(),

            Vfwsubvv(args) => todo!(),
            Vfwsubvf(args) => todo!(),

            Vfwredosumvs(args) => todo!(),

            Vfwaddwv(args) => todo!(),
            Vfwaddwf(args) => todo!(),

            Vfwsubwv(args) => todo!(),
            Vfwsubwf(args) => todo!(),

            Vfwmulvv(args) => todo!(),
            Vfwmulvf(args) => todo!(),

            Vfwmaccvv(args) => todo!(),
            Vfwmaccvf(args) => todo!(),

            Vfwnmaccvv(args) => todo!(),
            Vfwnmaccvf(args) => todo!(),

            Vfwmsacvv(args) => todo!(),
            Vfwmsacvf(args) => todo!(),

            Vfwnmsacvv(args) => todo!(),
            Vfwnmsacvf(args) => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use instruction::{format::base::*, Instruction::*};

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
