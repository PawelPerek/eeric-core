use prelude::*;

mod base;
mod d;
mod f;
mod m;
mod v;
mod zicsr;

use super::Instruction;

mod prelude;

pub struct Executor<'machine> {
    registers: &'machine mut Registers,
    memory: &'machine mut Memory,
}

impl<'m> Executor<'m> {
    pub fn new(registers: &'m mut Registers, memory: &'m mut Memory) -> Self {
        Self { registers, memory }
    }

    pub fn execute(&mut self, input: Instruction) {
        use Instruction::*;

        match input {
            Add(args) => base::add(args, &mut self.registers.x),
            Addw(args) => base::addw(args, &mut self.registers.x),
            Sub(args) => base::sub(args, &mut self.registers.x),
            Subw(args) => base::subw(args, &mut self.registers.x),
            Addi(args) => base::addi(args, &mut self.registers.x),
            Addiw(args) => base::addiw(args, &mut self.registers.x),
            Slt(args) => base::slt(args, &mut self.registers.x),
            Slti(args) => base::slti(args, &mut self.registers.x),
            Sltu(args) => base::sltu(args, &mut self.registers.x),
            Sltiu(args) => base::sltiu(args, &mut self.registers.x),
            Lui(args) => base::lui(args, &mut self.registers.x),
            Auipc(args) => base::auipc(args, &mut self.registers.x, self.registers.pc),
            And(args) => base::and(args, &mut self.registers.x),
            Or(args) => base::or(args, &mut self.registers.x),
            Xor(args) => base::xor(args, &mut self.registers.x),
            Andi(args) => base::andi(args, &mut self.registers.x),
            Ori(args) => base::ori(args, &mut self.registers.x),
            Xori(args) => base::xori(args, &mut self.registers.x),
            Sll(args) => base::sll(args, &mut self.registers.x),
            Sllw(args) => base::sllw(args, &mut self.registers.x),
            Srl(args) => base::srl(args, &mut self.registers.x),
            Srlw(args) => base::srlw(args, &mut self.registers.x),
            Sra(args) => base::sra(args, &mut self.registers.x),
            Sraw(args) => base::sraw(args, &mut self.registers.x),
            Slli(args) => base::slli(args, &mut self.registers.x),
            Slliw(args) => base::slliw(args, &mut self.registers.x),
            Srli(args) => base::srli(args, &mut self.registers.x),
            Srliw(args) => base::srliw(args, &mut self.registers.x),
            Srai(args) => base::srai(args, &mut self.registers.x),
            Sraiw(args) => base::sraiw(args, &mut self.registers.x),
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
            Beq(args) => base::beq(args, &self.registers.x, &mut self.registers.pc),
            Bne(args) => base::bne(args, &self.registers.x, &mut self.registers.pc),
            Bge(args) => base::bge(args, &self.registers.x, &mut self.registers.pc),
            Bgeu(args) => base::bgeu(args, &self.registers.x, &mut self.registers.pc),
            Blt(args) => base::blt(args, &self.registers.x, &mut self.registers.pc),
            Bltu(args) => base::bltu(args, &self.registers.x, &mut self.registers.pc),
            Jal(args) => base::jal(args, &mut self.registers.x, &mut self.registers.pc),
            Jalr(args) => base::jalr(args, &mut self.registers.x, &mut self.registers.pc),

            Csrrw(args) => zicsr::csrrw(args, &mut self.registers.x, &mut self.registers.c),
            Csrrs(args) => zicsr::csrrs(args, &mut self.registers.x, &mut self.registers.c),
            Csrrc(args) => zicsr::csrrc(args, &mut self.registers.x, &mut self.registers.c),
            Csrrwi(args) => zicsr::csrrwi(args, &mut self.registers.x, &mut self.registers.c),
            Csrrsi(args) => zicsr::csrrsi(args, &mut self.registers.x, &mut self.registers.c),
            Csrrci(args) => zicsr::csrrci(args, &mut self.registers.x, &mut self.registers.c),

            Mul(args) => m::mul(args, &mut self.registers.x),
            Mulh(args) => m::mulh(args, &mut self.registers.x),
            Mulhsu(args) => m::mulhsu(args, &mut self.registers.x),
            Mulhu(args) => m::mulhu(args, &mut self.registers.x),
            Div(args) => m::div(args, &mut self.registers.x),
            Divu(args) => m::divu(args, &mut self.registers.x),
            Rem(args) => m::rem(args, &mut self.registers.x),
            Remu(args) => m::remu(args, &mut self.registers.x),
            Mulw(args) => m::mulw(args, &mut self.registers.x),
            Divw(args) => m::divw(args, &mut self.registers.x),
            Divuw(args) => m::divuw(args, &mut self.registers.x),
            Remw(args) => m::remw(args, &mut self.registers.x),
            Remuw(args) => m::remuw(args, &mut self.registers.x),

            Flw(args) => f::flw(args, &self.registers.x, &mut self.registers.f, &self.memory),
            Fsw(args) => f::fsw(args, &self.registers.x, &self.registers.f, &mut self.memory),
            Fmadds(args) => f::fmadd::s(args, &mut self.registers.f),
            Fmsubs(args) => f::fmsub::s(args, &mut self.registers.f),
            Fnmsubs(args) => f::fnmsub::s(args, &mut self.registers.f),
            Fnmadds(args) => f::fnmadd::s(args, &mut self.registers.f),
            Fadds(args) => f::fadd::s(args, &mut self.registers.f),
            Fsubs(args) => f::fsub::s(args, &mut self.registers.f),
            Fmuls(args) => f::fmul::s(args, &mut self.registers.f),
            Fdivs(args) => f::fdiv::s(args, &mut self.registers.f),
            Fsqrts(args) => f::fsqrt::s(args, &mut self.registers.f),
            Fsgnjs(args) => f::fsgnj::s(args, &mut self.registers.f),
            Fsgnjns(args) => f::fsgnjn::s(args, &mut self.registers.f),
            Fsgnjxs(args) => f::fsgnjx::s(args, &mut self.registers.f),
            Fmins(args) => f::fmin::s(args, &mut self.registers.f),
            Fmaxs(args) => f::fmax::s(args, &mut self.registers.f),
            Fcvtws(args) => f::fcvt::ws(args, &mut self.registers.x, &self.registers.f),
            Fcvtwus(args) => f::fcvt::wus(args, &mut self.registers.x, &self.registers.f),
            Fmvxw(args) => f::fmv::xw(args, &mut self.registers.x, &self.registers.f),
            Feqs(args) => f::feq::s(args, &mut self.registers.x, &self.registers.f),
            Flts(args) => f::flt::s(args, &mut self.registers.x, &self.registers.f),
            Fles(args) => f::fle::s(args, &mut self.registers.x, &self.registers.f),
            Fclasss(args) => f::fclass::s(args, &mut self.registers.x, &self.registers.f),
            Fcvtsw(args) => f::fcvt::sw(args, &self.registers.x, &mut self.registers.f),
            Fcvtswu(args) => f::fcvt::swu(args, &self.registers.x, &mut self.registers.f),
            Fmvwx(args) => f::fmv::wx(args, &self.registers.x, &mut self.registers.f),
            Fcvtls(args) => f::fcvt::ls(args, &mut self.registers.x, &self.registers.f),
            Fcvtlus(args) => f::fcvt::lus(args, &mut self.registers.x, &self.registers.f),
            Fcvtsl(args) => f::fcvt::sl(args, &self.registers.x, &mut self.registers.f),
            Fcvtslu(args) => f::fcvt::slu(args, &self.registers.x, &mut self.registers.f),

            Fld(args) => d::fld(args, &self.registers.x, &mut self.registers.f, &self.memory),
            Fsd(args) => d::fsd(args, &self.registers.x, &self.registers.f, &mut self.memory),
            Fmaddd(args) => d::fmadd::d(args, &mut self.registers.f),
            Fmsubd(args) => d::fmsub::d(args, &mut self.registers.f),
            Fnmsubd(args) => d::fnmsub::d(args, &mut self.registers.f),
            Fnmaddd(args) => d::fnmadd::d(args, &mut self.registers.f),
            Faddd(args) => d::fadd::d(args, &mut self.registers.f),
            Fsubd(args) => d::fsub::d(args, &mut self.registers.f),
            Fmuld(args) => d::fmul::d(args, &mut self.registers.f),
            Fdivd(args) => d::fdiv::d(args, &mut self.registers.f),
            Fsqrtd(args) => d::fsqrt::d(args, &mut self.registers.f),
            Fsgnjd(args) => d::fsgnj::d(args, &mut self.registers.f),
            Fsgnjnd(args) => d::fsgnjn::d(args, &mut self.registers.f),
            Fsgnjxd(args) => d::fsgnjx::d(args, &mut self.registers.f),
            Fmind(args) => d::fmin::d(args, &mut self.registers.f),
            Fmaxd(args) => d::fmax::d(args, &mut self.registers.f),
            Fcvtsd(args) => d::fcvt::sd(args, &mut self.registers.f),
            Fcvtds(args) => d::fcvt::ds(args, &mut self.registers.f),
            Feqd(args) => d::feq::d(args, &mut self.registers.x, &self.registers.f),
            Fltd(args) => d::flt::d(args, &mut self.registers.x, &self.registers.f),
            Fled(args) => d::fle::d(args, &mut self.registers.x, &self.registers.f),
            Fclassd(args) => d::fclass::d(args, &mut self.registers.x, &self.registers.f),
            Fcvtwd(args) => d::fcvt::wd(args, &mut self.registers.x, &self.registers.f),
            Fcvtwud(args) => d::fcvt::wud(args, &mut self.registers.x, &self.registers.f),
            Fcvtdw(args) => d::fcvt::dw(args, &self.registers.x, &mut self.registers.f),
            Fcvtdwu(args) => d::fcvt::dwu(args, &self.registers.x, &mut self.registers.f),
            Fcvtld(args) => d::fcvt::ld(args, &mut self.registers.x, &self.registers.f),
            Fcvtlud(args) => d::fcvt::lud(args, &mut self.registers.x, &self.registers.f),
            Fmvxd(args) => d::fmv::xd(args, &mut self.registers.x, &self.registers.f),
            Fcvtdl(args) => d::fcvt::dl(args, &self.registers.x, &mut self.registers.f),
            Fcvtdlu(args) => d::fcvt::dlu(args, &self.registers.x, &mut self.registers.f),
            Fmvdx(args) => d::fmv::dx(args, &self.registers.x, &mut self.registers.f),

            Vsetvli(args) => v::vsetvli(
                args,
                &mut self.registers.x,
                &self.registers.v,
                &mut self.registers.c,
            ),
            Vsetivli(args) => v::vsetivli(
                args,
                &mut self.registers.x,
                &self.registers.v,
                &mut self.registers.c,
            ),
            Vsetvl(args) => v::vsetvl(
                args,
                &mut self.registers.x,
                &self.registers.v,
                &mut self.registers.c,
            ),

            Vlv { data: args, eew } => v::vl::v(args, SEW::new(eew).unwrap(), &self.registers.x, &mut self.registers.v, &self.memory),
            Vsv { data: args, eew } => v::vs::v(args, SEW::new(eew).unwrap(), &self.registers.x, &self.registers.v, &mut self.memory),

            Vlmv(args) => v::vlm::v(args, &mut self.registers.v, &self.memory),
            Vsmv(args) => v::vsm::v(args, &self.registers.v, &mut self.memory),

            Vlsv { data: args, eew } => v::vls::v(args, SEW::new(eew).unwrap(), &self.registers.x, &mut self.registers.v, &self.memory),
            Vssv { data: args, eew } => v::vss::v(args, SEW::new(eew).unwrap(), &self.registers.v, &mut self.memory),

            Vluxv { data: args, eew } => v::vlux::v(args, eew, &mut self.registers.v, &self.memory),
            Vloxv { data: args, eew } => v::vlox::v(args, eew, &mut self.registers.v, &self.memory),
            Vsuxv { data: args, eew } => v::vsux::v(args, eew, &self.registers.v, &mut self.memory),
            Vsoxv { data: args, eew } => v::vsox::v(args, eew, &self.registers.v, &mut self.memory),

            Vlffv { data: args, eew } => v::vlff::v(args, eew, &mut self.registers.v, &self.memory),

            Vlsegv {
                data: args,
                eew,
                nf,
            } => v::vlseg::v(args, eew, &mut self.registers.v, &self.memory),
            Vssegv {
                data: args,
                eew,
                nf,
            } => v::vsseg::v(args, eew, &self.registers.v, &mut self.memory),

            Vlssegv {
                data: args,
                eew,
                nf,
            } => v::vlsseg::v(args, eew, &mut self.registers.v, &self.memory),
            Vsssegv {
                data: args,
                eew,
                nf,
            } => v::vssseg::v(args, eew, &self.registers.v, &mut self.memory),

            Vluxsegv {
                data: args,
                eew,
                nf,
            } => v::vluxseg::v(args, eew, &mut self.registers.v, &self.memory),
            Vloxsegv {
                data: args,
                eew,
                nf,
            } => v::vloxseg::v(args, eew, &mut self.registers.v, &self.memory),
            Vsuxsegv {
                data: args,
                eew,
                nf,
            } => v::vsuxseg::v(args, eew, &self.registers.v, &mut self.memory),
            Vsoxsegv {
                data: args,
                eew,
                nf,
            } => v::vsoxseg::v(args, eew, &self.registers.v, &mut self.memory),

            Vlrv {
                data: args,
                eew,
                nf,
            } => v::vlr::v(args, eew, &mut self.registers.v, &self.memory),
            Vsrv {
                data: args,
                eew,
                nf,
            } => v::vsr::v(args, eew, &self.registers.v, &mut self.memory),

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

            Vrgathervv(args) => v::vrgather::vv(args, &mut self.registers.v),
            Vrgathervx(args) => v::vrgather::vx(args, &mut self.registers.v, &self.registers.x),
            Vrgathervi(args) => v::vrgather::vi(args, &mut self.registers.v),

            Vrgatherei16vv(args) => v::vrgatherei16::vv(args, &mut self.registers.v),

            Vslideupvx(args) => v::vslideup::vx(args, &mut self.registers.v, &self.registers.x),
            Vslideupvi(args) => v::vslideup::vi(args, &mut self.registers.v),

            Vslidedownvx(args) => v::vslidedown::vx(args, &mut self.registers.v, &self.registers.x),
            Vslidedownvi(args) => v::vslidedown::vi(args, &mut self.registers.v),

            Vadcvvm(args) => v::vadc::vvm(args, &mut self.registers.v),
            Vadcvxm(args) => v::vadc::vxm(args, &mut self.registers.v, &self.registers.x),
            Vadcvim(args) => v::vadc::vim(args, &mut self.registers.v),

            Vmadcvvm(args) => v::vmadc::vvm(args, &mut self.registers.v),
            Vmadcvxm(args) => v::vmadc::vxm(args, &mut self.registers.v, &self.registers.x),
            Vmadcvim(args) => v::vmadc::vim(args, &mut self.registers.v),

            Vmadcvv(args) => v::vmadc::vv(args, &mut self.registers.v),
            Vmadcvx(args) => v::vmadc::vx(args, &mut self.registers.v, &self.registers.x),
            Vmadcvi(args) => v::vmadc::vi(args, &mut self.registers.v),

            Vsbcvvm(args) => v::vsbc::vvm(args, &mut self.registers.v),
            Vsbcvxm(args) => v::vsbc::vxm(args, &mut self.registers.v, &self.registers.x),

            Vmsbcvvm(args) => v::vmsbc::vvm(args, &mut self.registers.v),
            Vmsbcvxm(args) => v::vmsbc::vxm(args, &mut self.registers.v, &self.registers.x),

            Vmsbcvv(args) => v::vmsbc::vv(args, &mut self.registers.v),
            Vmsbcvx(args) => v::vmsbc::vx(args, &mut self.registers.v, &self.registers.x),

            Vmergevvm(args) => v::vmerge::vvm(args, &mut self.registers.v),
            Vmergevxm(args) => v::vmerge::vxm(args, &mut self.registers.v, &self.registers.x),
            Vmergevim(args) => v::vmerge::vim(args, &mut self.registers.v),

            Vmvvv(args) => v::vmv::vv(args, &mut self.registers.v),
            Vmvvx(args) => v::vmv::vx(args, &mut self.registers.v, &self.registers.x),
            Vmvvi(args) => v::vmv::vi(args, &mut self.registers.v),

            Vmseqvv(args) => v::vmseq::vv(args, &mut self.registers.v),
            Vmseqvx(args) => v::vmseq::vx(args, &mut self.registers.v, &self.registers.x),
            Vmseqvi(args) => v::vmseq::vi(args, &mut self.registers.v),

            Vmsnevv(args) => v::vmsne::vv(args, &mut self.registers.v),
            Vmsnevx(args) => v::vmsne::vx(args, &mut self.registers.v, &self.registers.x),
            Vmsnevi(args) => v::vmsne::vi(args, &mut self.registers.v),

            Vmsltuvv(args) => v::vmsltu::vv(args, &mut self.registers.v),
            Vmsltuvx(args) => v::vmsltu::vx(args, &mut self.registers.v, &self.registers.x),

            Vmsltvv(args) => v::vmslt::vv(args, &mut self.registers.v),
            Vmsltvx(args) => v::vmslt::vx(args, &mut self.registers.v, &self.registers.x),

            Vmsleuvv(args) => v::vmsleu::vv(args, &mut self.registers.v),
            Vmsleuvx(args) => v::vmsleu::vx(args, &mut self.registers.v, &self.registers.x),
            Vmsleuvi(args) => v::vmsleu::vi(args, &mut self.registers.v),

            Vmslevv(args) => v::vmsle::vv(args, &mut self.registers.v),
            Vmslevx(args) => v::vmsle::vx(args, &mut self.registers.v, &self.registers.x),
            Vmslevi(args) => v::vmsle::vi(args, &mut self.registers.v),

            Vmsgtuvx(args) => v::vmsgtu::vx(args, &mut self.registers.v, &self.registers.x),
            Vmsgtuvi(args) => v::vmsgtu::vi(args, &mut self.registers.v),

            Vmsgtvx(args) => v::vmsgt::vx(args, &mut self.registers.v, &self.registers.x),
            Vmsgtvi(args) => v::vmsgt::vi(args, &mut self.registers.v),

            Vsadduvv(args) => v::vsaddu::vv(args, &mut self.registers.v),
            Vsadduvx(args) => v::vsaddu::vx(args, &mut self.registers.v, &self.registers.x),
            Vsadduvi(args) => v::vsaddu::vi(args, &mut self.registers.v),

            Vsaddvv(args) => v::vsadd::vv(args, &mut self.registers.v),
            Vsaddvx(args) => v::vsadd::vx(args, &mut self.registers.v, &self.registers.x),
            Vsaddvi(args) => v::vsadd::vi(args, &mut self.registers.v),

            Vssubuvv(args) => v::vssubu::vv(args, &mut self.registers.v),
            Vssubuvx(args) => v::vssubu::vx(args, &mut self.registers.v, &self.registers.x),

            Vssubvv(args) => v::vssub::vv(args, &mut self.registers.v),
            Vssubvx(args) => v::vssub::vx(args, &mut self.registers.v, &self.registers.x),

            Vsllvv(args) => v::vsll::vv(args, &mut self.registers.v),
            Vsllvx(args) => v::vsll::vx(args, &mut self.registers.v, &self.registers.x),
            Vsllvi(args) => v::vsll::vi(args, &mut self.registers.v),

            Vsmulvv(args) => v::vsmul::vv(args, &mut self.registers.v, &mut self.registers.c),
            Vsmulvx(args) => v::vsmul::vx(
                args,
                &mut self.registers.v,
                &self.registers.x,
                &mut self.registers.c,
            ),

            Vmv1rv(args) => v::vmv1r::v(args, &mut self.registers.v),
            Vmv2rv(args) => v::vmv2r::v(args, &mut self.registers.v),
            Vmv4rv(args) => v::vmv4r::v(args, &mut self.registers.v),
            Vmv8rv(args) => v::vmv8r::v(args, &mut self.registers.v),

            Vsrlvv(args) => v::vsrl::vv(args, &mut self.registers.v),
            Vsrlvx(args) => v::vsrl::vx(args, &mut self.registers.v, &self.registers.x),
            Vsrlvi(args) => v::vsrl::vi(args, &mut self.registers.v),

            Vsravv(args) => v::vsra::vv(args, &mut self.registers.v),
            Vsravx(args) => v::vsra::vx(args, &mut self.registers.v, &self.registers.x),
            Vsravi(args) => v::vsra::vi(args, &mut self.registers.v),

            Vssrlvv(args) => v::vssrl::vv(args, &mut self.registers.v),
            Vssrlvx(args) => v::vssrl::vx(args, &mut self.registers.v, &self.registers.x),
            Vssrlvi(args) => v::vssrl::vi(args, &mut self.registers.v),

            Vssravv(args) => v::vssra::vv(args, &mut self.registers.v),
            Vssravx(args) => v::vssra::vx(args, &mut self.registers.v, &self.registers.x),
            Vssravi(args) => v::vssra::vi(args, &mut self.registers.v),

            Vnsrlwv(args) => v::vnsrl::wv(args, &mut self.registers.v),
            Vnsrlwx(args) => v::vnsrl::wx(args, &mut self.registers.v, &self.registers.x),
            Vnsrlwi(args) => v::vnsrl::wi(args, &mut self.registers.v),

            Vnsrawv(args) => v::vnsra::wv(args, &mut self.registers.v),
            Vnsrawx(args) => v::vnsra::wx(args, &mut self.registers.v, &self.registers.x),
            Vnsrawi(args) => v::vnsra::wi(args, &mut self.registers.v),

            Vnclipuwv(args) => v::vnclipu::wv(args, &mut self.registers.v, &mut self.registers.c),
            Vnclipuwx(args) => v::vnclipu::wx(
                args,
                &mut self.registers.v,
                &self.registers.x,
                &mut self.registers.c,
            ),
            Vnclipuwi(args) => v::vnclipu::wi(args, &mut self.registers.v, &mut self.registers.c),

            Vnclipwv(args) => v::vnclip::wv(args, &mut self.registers.v, &mut self.registers.c),
            Vnclipwx(args) => v::vnclip::wx(
                args,
                &mut self.registers.v,
                &self.registers.x,
                &mut self.registers.c,
            ),
            Vnclipwi(args) => v::vnclip::wi(args, &mut self.registers.v, &mut self.registers.c),

            Vwredsumuvs(args) => v::vwredsumu::vs(args, &mut self.registers.v),
            Vwredsumvs(args) => v::vwredsum::vs(args, &mut self.registers.v),

            Vredsumvs(args) => v::vredsum::vs(args, &mut self.registers.v),
            Vredandvs(args) => v::vredand::vs(args, &mut self.registers.v),
            Vredorvs(args) => v::vredor::vs(args, &mut self.registers.v),
            Vredxorvs(args) => v::vredxor::vs(args, &mut self.registers.v),
            Vredminuvs(args) => v::vredminu::vs(args, &mut self.registers.v),
            Vredminvs(args) => v::vredmin::vs(args, &mut self.registers.v),
            Vredmaxuvs(args) => v::vredmaxu::vs(args, &mut self.registers.v),
            Vredmaxvs(args) => v::vredmax::vs(args, &mut self.registers.v),

            Vaadduvv(args) => v::vaaddu::vv(args, &mut self.registers.v, &self.registers.c),
            Vaadduvx(args) => v::vaaddu::vx(
                args,
                &mut self.registers.v,
                &self.registers.x,
                &self.registers.c,
            ),

            Vaaddvv(args) => v::vaadd::vv(args, &mut self.registers.v, &self.registers.c),
            Vaaddvx(args) => v::vaadd::vx(
                args,
                &mut self.registers.v,
                &self.registers.x,
                &self.registers.c,
            ),

            Vasubuvv(args) => v::vasubu::vv(args, &mut self.registers.v, &self.registers.c),
            Vasubuvx(args) => v::vasubu::vx(
                args,
                &mut self.registers.v,
                &self.registers.x,
                &self.registers.c,
            ),

            Vasubvv(args) => v::vasub::vv(args, &mut self.registers.v, &self.registers.c),
            Vasubvx(args) => v::vasub::vx(
                args,
                &mut self.registers.v,
                &self.registers.x,
                &self.registers.c,
            ),

            Vslide1upvx(args) => v::vslide1up::vx(args, &mut self.registers.v, &self.registers.x),

            Vslide1downvx(args) => {
                v::vslide1down::vx(args, &mut self.registers.v, &self.registers.x)
            }

            Vmvxs(args) => v::vmv::xs(args, &self.registers.v, &mut self.registers.x),
            Vcpopm(args) => v::vcpop::m(args, &self.registers.v, &mut self.registers.x),
            Vfirstm(args) => v::vfirst::m(args, &self.registers.v, &mut self.registers.x),

            Vmvsx(args) => v::vmv::sx(args, &mut self.registers.v, &self.registers.x),

            Vzextvf8(args) => v::vzext::vf8(args, &mut self.registers.v),
            Vsextvf8(args) => v::vsext::vf8(args, &mut self.registers.v),
            Vzextvf4(args) => v::vzext::vf4(args, &mut self.registers.v),
            Vsextvf4(args) => v::vsext::vf4(args, &mut self.registers.v),
            Vzextvf2(args) => v::vzext::vf2(args, &mut self.registers.v),
            Vsextvf2(args) => v::vsext::vf2(args, &mut self.registers.v),

            Vmsbfm(args) => v::vmsbf::m(args, &mut self.registers.v),
            Vmsofm(args) => v::vmsof::m(args, &mut self.registers.v),
            Vmsifm(args) => v::vmsif::m(args, &mut self.registers.v),
            Viotam(args) => v::viota::m(args, &mut self.registers.v),
            Vidv(args) => v::vid::v(args, &mut self.registers.v),

            Vcompressvm(args) => v::vcompress::vm(args, &mut self.registers.v),

            Vmandnmm(args) => v::vmandn::mm(args, &mut self.registers.v),
            Vmandmm(args) => v::vmand::mm(args, &mut self.registers.v),
            Vmormm(args) => v::vmor::mm(args, &mut self.registers.v),
            Vmxormm(args) => v::vmxor::mm(args, &mut self.registers.v),
            Vmornmm(args) => v::vmorn::mm(args, &mut self.registers.v),
            Vmnandmm(args) => v::vmnand::mm(args, &mut self.registers.v),
            Vmnormm(args) => v::vmnor::mm(args, &mut self.registers.v),
            Vmxnormm(args) => v::vmxnor::mm(args, &mut self.registers.v),

            Vdivuvv(args) => v::vdivu::vv(args, &mut self.registers.v),
            Vdivuvx(args) => v::vdivu::vx(args, &mut self.registers.v, &self.registers.x),

            Vdivvv(args) => v::vdiv::vv(args, &mut self.registers.v),
            Vdivvx(args) => v::vdiv::vx(args, &mut self.registers.v, &self.registers.x),

            Vremuvv(args) => v::vremu::vv(args, &mut self.registers.v),
            Vremuvx(args) => v::vremu::vx(args, &mut self.registers.v, &self.registers.x),

            Vremvv(args) => v::vrem::vv(args, &mut self.registers.v),
            Vremvx(args) => v::vrem::vx(args, &mut self.registers.v, &self.registers.x),

            Vmulhuvv(args) => v::vmulhu::vv(args, &mut self.registers.v),
            Vmulhuvx(args) => v::vmulhu::vx(args, &mut self.registers.v, &self.registers.x),

            Vmulvv(args) => v::vmul::vv(args, &mut self.registers.v),
            Vmulvx(args) => v::vmul::vx(args, &mut self.registers.v, &self.registers.x),

            Vmulhsuvv(args) => v::vmulhsu::vv(args, &mut self.registers.v),
            Vmulhsuvx(args) => v::vmulhsu::vx(args, &mut self.registers.v, &self.registers.x),

            Vmulhvv(args) => v::vmulh::vv(args, &mut self.registers.v),
            Vmulhvx(args) => v::vmulh::vx(args, &mut self.registers.v, &self.registers.x),

            Vmaddvv(args) => v::vmadd::vv(args, &mut self.registers.v),
            Vmaddvx(args) => v::vmadd::vx(args, &mut self.registers.v, &self.registers.x),

            Vnmsubvv(args) => v::vnmsub::vv(args, &mut self.registers.v),
            Vnmsubvx(args) => v::vnmsub::vx(args, &mut self.registers.v, &self.registers.x),

            Vmaccvv(args) => v::vmacc::vv(args, &mut self.registers.v),
            Vmaccvx(args) => v::vmacc::vx(args, &mut self.registers.v, &self.registers.x),

            Vnmsacvv(args) => v::vnmsac::vv(args, &mut self.registers.v),
            Vnmsacvx(args) => v::vnmsac::vx(args, &mut self.registers.v, &self.registers.x),

            Vwadduvv(args) => v::vwaddu::vv(args, &mut self.registers.v),
            Vwadduvx(args) => v::vwaddu::vx(args, &mut self.registers.v, &self.registers.x),

            Vwaddvv(args) => v::vwadd::vv(args, &mut self.registers.v),
            Vwaddvx(args) => v::vwadd::vx(args, &mut self.registers.v, &self.registers.x),

            Vwsubuvv(args) => v::vwsubu::vv(args, &mut self.registers.v),
            Vwsubuvx(args) => v::vwsubu::vx(args, &mut self.registers.v, &self.registers.x),

            Vwsubvv(args) => v::vwsub::vv(args, &mut self.registers.v),
            Vwsubvx(args) => v::vwsub::vx(args, &mut self.registers.v, &self.registers.x),

            Vwadduwv(args) => v::vwaddu::wv(args, &mut self.registers.v),
            Vwadduwx(args) => v::vwaddu::wx(args, &mut self.registers.v, &self.registers.x),

            Vwaddwv(args) => v::vwadd::wv(args, &mut self.registers.v),
            Vwaddwx(args) => v::vwadd::wx(args, &mut self.registers.v, &self.registers.x),

            Vwsubuwv(args) => v::vwsubu::wv(args, &mut self.registers.v),
            Vwsubuwx(args) => v::vwsubu::wx(args, &mut self.registers.v, &self.registers.x),

            Vwsubwv(args) => v::vwsub::wv(args, &mut self.registers.v),
            Vwsubwx(args) => v::vwsub::wx(args, &mut self.registers.v, &self.registers.x),

            Vwmuluvv(args) => v::vwmulu::vv(args, &mut self.registers.v),
            Vwmuluvx(args) => v::vwmulu::vx(args, &mut self.registers.v, &self.registers.x),

            Vwmulsuvv(args) => v::vwmulsu::vv(args, &mut self.registers.v),
            Vwmulsuvx(args) => v::vwmulsu::vx(args, &mut self.registers.v, &self.registers.x),

            Vwmulvv(args) => v::vwmul::vv(args, &mut self.registers.v),
            Vwmulvx(args) => v::vwmul::vx(args, &mut self.registers.v, &self.registers.x),

            Vwmaccuvv(args) => v::vwmaccu::vv(args, &mut self.registers.v),
            Vwmaccuvx(args) => v::vwmaccu::vx(args, &mut self.registers.v, &self.registers.x),

            Vwmaccvv(args) => v::vwmacc::vv(args, &mut self.registers.v),
            Vwmaccvx(args) => v::vwmacc::vx(args, &mut self.registers.v, &self.registers.x),

            Vwmaccusvx(args) => v::vwmaccus::vx(args, &mut self.registers.v, &self.registers.x),

            Vwmaccsuvv(args) => v::vwmaccsu::vv(args, &mut self.registers.v),
            Vwmaccsuvx(args) => v::vwmaccsu::vx(args, &mut self.registers.v, &self.registers.x),

            Vfaddvv(args) => v::vfadd::vv(args, &mut self.registers.v),
            Vfaddvf(args) => v::vfadd::vf(args, &mut self.registers.v, &self.registers.f),

            Vfredusumvs(args) => v::vfredusum::vs(args, &mut self.registers.v),

            Vfsubvv(args) => v::vfsub::vv(args, &mut self.registers.v),
            Vfsubvf(args) => v::vfsub::vf(args, &mut self.registers.v, &self.registers.f),

            Vfredosumvs(args) => v::vfredosum::vs(args, &mut self.registers.v),

            Vfminvv(args) => v::vfmin::vv(args, &mut self.registers.v),
            Vfminvf(args) => v::vfmin::vf(args, &mut self.registers.v, &self.registers.f),

            Vfredminvs(args) => v::vfredmin::vs(args, &mut self.registers.v),

            Vfmaxvv(args) => v::vfmax::vv(args, &mut self.registers.v),
            Vfmaxvf(args) => v::vfmax::vf(args, &mut self.registers.v, &self.registers.f),

            Vfredmaxvs(args) => v::vfredmax::vs(args, &mut self.registers.v),

            Vfsgnjvv(args) => v::vfsgnj::vv(args, &mut self.registers.v),
            Vfsgnjvf(args) => v::vfsgnj::vf(args, &mut self.registers.v, &self.registers.f),

            Vfsgnjnvv(args) => v::vfsgnjn::vv(args, &mut self.registers.v),
            Vfsgnjnvf(args) => v::vfsgnjn::vf(args, &mut self.registers.v, &self.registers.f),

            Vfsgnjxvv(args) => v::vfsgnjx::vv(args, &mut self.registers.v),
            Vfsgnjxvf(args) => v::vfsgnjx::vf(args, &mut self.registers.v, &self.registers.f),

            Vfslide1upvf(args) => v::vfslide1up::vf(args, &mut self.registers.v, &self.registers.f),

            Vfslide1downvf(args) => {
                v::vfslide1down::vf(args, &mut self.registers.v, &self.registers.f)
            }

            Vfmvfs(args) => v::vfmv::fs(args, &self.registers.v, &mut self.registers.f),

            Vfmvsf(args) => v::vfmv::sf(args, &mut self.registers.v, &self.registers.f),

            Vfcvtxufv(args) => v::vfcvt::xufv(args, &mut self.registers.v),
            Vfcvtxfv(args) => v::vfcvt::xfv(args, &mut self.registers.v),
            Vfcvtfxuv(args) => v::vfcvt::fxuv(args, &mut self.registers.v),
            Vfcvtfxv(args) => v::vfcvt::fxv(args, &mut self.registers.v),
            VfcvtRtzxufv(args) => v::vfcvt::rtzxufv(args, &mut self.registers.v),
            VfcvtRtzxfv(args) => v::vfcvt::rtzxfv(args, &mut self.registers.v),

            Vfwcvtxufv(args) => v::vfwcvt::xufv(args, &mut self.registers.v),
            Vfwcvtxfv(args) => v::vfwcvt::xfv(args, &mut self.registers.v),
            Vfwcvtfxuv(args) => v::vfwcvt::fxuv(args, &mut self.registers.v),
            Vfwcvtfxv(args) => v::vfwcvt::fxv(args, &mut self.registers.v),
            Vfwcvtffv(args) => v::vfwcvt::ffv(args, &mut self.registers.v),
            VfwcvtRtzxufv(args) => v::vfwcvt::rtzxufv(args, &mut self.registers.v),
            VfwcvtRtzxfv(args) => v::vfwcvt::rtzxfv(args, &mut self.registers.v),

            Vfncvtxufw(args) => v::vfncvt::xufw(args, &mut self.registers.v),
            Vfncvtxfw(args) => v::vfncvt::xfw(args, &mut self.registers.v),
            Vfncvtfxuw(args) => v::vfncvt::fxuw(args, &mut self.registers.v),
            Vfncvtfxw(args) => v::vfncvt::fxw(args, &mut self.registers.v),
            Vfncvtffw(args) => v::vfncvt::ffw(args, &mut self.registers.v),
            VfncvtRodffw(args) => v::vfncvt::rodffw(args, &mut self.registers.v),
            VfncvtRtzxufw(args) => v::vfncvt::rtzxufw(args, &mut self.registers.v),
            VfncvtRtzxfw(args) => v::vfncvt::rtzxfw(args, &mut self.registers.v),

            Vfsqrtv(args) => v::vfsqrt::v(args, &mut self.registers.v),
            Vfrsqrt7v(args) => v::vfrsqrt7::v(args, &mut self.registers.v),
            Vfrec7v(args) => v::vfrec7::v(args, &mut self.registers.v),
            Vfclassv(args) => v::vfclass::v(args, &mut self.registers.v),

            Vfmergevfm(args) => v::vfmerge::vfm(args, &mut self.registers.v, &self.registers.f),
            Vfmvvf(args) => v::vfmv::vf(args, &mut self.registers.v, &self.registers.f),

            Vmfeqvv(args) => v::vmfeq::vv(args, &mut self.registers.v),
            Vmfeqvf(args) => v::vmfeq::vf(args, &mut self.registers.v, &self.registers.f),

            Vmflevv(args) => v::vmfle::vv(args, &mut self.registers.v),
            Vmflevf(args) => v::vmfle::vf(args, &mut self.registers.v, &self.registers.f),

            Vmfltvv(args) => v::vmflt::vv(args, &mut self.registers.v),
            Vmfltvf(args) => v::vmflt::vf(args, &mut self.registers.v, &self.registers.f),

            Vmfnevv(args) => v::vmfne::vv(args, &mut self.registers.v),
            Vmfnevf(args) => v::vmfne::vf(args, &mut self.registers.v, &self.registers.f),

            Vmfgtvf(args) => v::vmfgt::vf(args, &mut self.registers.v, &self.registers.f),

            Vmfgevf(args) => v::vmfge::vf(args, &mut self.registers.v, &self.registers.f),

            Vfdivvv(args) => v::vfdiv::vv(args, &mut self.registers.v),
            Vfdivvf(args) => v::vfdiv::vf(args, &mut self.registers.v, &self.registers.f),

            Vfrdivvf(args) => v::vfrdiv::vf(args, &mut self.registers.v, &self.registers.f),

            Vfmulvv(args) => v::vfmul::vv(args, &mut self.registers.v),
            Vfmulvf(args) => v::vfmul::vf(args, &mut self.registers.v, &self.registers.f),

            Vfrsubvf(args) => v::vfrsub::vf(args, &mut self.registers.v, &self.registers.f),

            Vfmaddvv(args) => v::vfmadd::vv(args, &mut self.registers.v),
            Vfmaddvf(args) => v::vfmadd::vf(args, &mut self.registers.v, &self.registers.f),

            Vfnmaddvv(args) => v::vfnmadd::vv(args, &mut self.registers.v),
            Vfnmaddvf(args) => v::vfnmadd::vf(args, &mut self.registers.v, &self.registers.f),

            Vfmsubvv(args) => v::vfmsub::vv(args, &mut self.registers.v),
            Vfmsubvf(args) => v::vfmsub::vf(args, &mut self.registers.v, &self.registers.f),

            Vfnmsubvv(args) => v::vfnmsub::vv(args, &mut self.registers.v),
            Vfnmsubvf(args) => v::vfnmsub::vf(args, &mut self.registers.v, &self.registers.f),

            Vfmaccvv(args) => v::vfmacc::vv(args, &mut self.registers.v),
            Vfmaccvf(args) => v::vfmacc::vf(args, &mut self.registers.v, &self.registers.f),

            Vfnmaccvv(args) => v::vfnmacc::vv(args, &mut self.registers.v),
            Vfnmaccvf(args) => v::vfnmacc::vf(args, &mut self.registers.v, &self.registers.f),

            Vfmsacvv(args) => v::vfmsac::vv(args, &mut self.registers.v),
            Vfmsacvf(args) => v::vfmsac::vf(args, &mut self.registers.v, &self.registers.f),

            Vfnmsacvv(args) => v::vfnmsac::vv(args, &mut self.registers.v),
            Vfnmsacvf(args) => v::vfnmsac::vf(args, &mut self.registers.v, &self.registers.f),

            Vfwaddvv(args) => v::vfwadd::vv(args, &mut self.registers.v),
            Vfwaddvf(args) => v::vfwadd::vf(args, &mut self.registers.v, &self.registers.f),

            Vfwredusumvs(args) => v::vfwredusum::vs(args, &mut self.registers.v),

            Vfwsubvv(args) => v::vfwsub::vv(args, &mut self.registers.v),
            Vfwsubvf(args) => v::vfwsub::vf(args, &mut self.registers.v, &self.registers.f),

            Vfwredosumvs(args) => v::vfwredosum::vs(args, &mut self.registers.v),

            Vfwaddwv(args) => v::vfwadd::wv(args, &mut self.registers.v),
            Vfwaddwf(args) => v::vfwadd::wf(args, &mut self.registers.v, &self.registers.f),

            Vfwsubwv(args) => v::vfwsub::wv(args, &mut self.registers.v),
            Vfwsubwf(args) => v::vfwsub::wf(args, &mut self.registers.v, &self.registers.f),

            Vfwmulvv(args) => v::vfmul::vv(args, &mut self.registers.v),
            Vfwmulvf(args) => v::vfmul::vf(args, &mut self.registers.v, &self.registers.f),

            Vfwmaccvv(args) => v::vfwmacc::vv(args, &mut self.registers.v),
            Vfwmaccvf(args) => v::vfwmacc::vf(args, &mut self.registers.v, &self.registers.f),

            Vfwnmaccvv(args) => v::vfwnmacc::vv(args, &mut self.registers.v),
            Vfwnmaccvf(args) => v::vfwnmacc::vf(args, &mut self.registers.v, &self.registers.f),

            Vfwmsacvv(args) => v::vfwmsac::vv(args, &mut self.registers.v),
            Vfwmsacvf(args) => v::vfwmsac::vf(args, &mut self.registers.v, &self.registers.f),

            Vfwnmsacvv(args) => v::vfwnmsac::vv(args, &mut self.registers.v),
            Vfwnmsacvf(args) => v::vfwnmsac::vf(args, &mut self.registers.v, &self.registers.f),
        }
    }
}