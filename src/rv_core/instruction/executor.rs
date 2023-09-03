use prelude::*;

mod base;
mod d;
mod f;
mod m;
mod v;
mod zicsr;

use super::Instruction;

mod prelude;

pub struct Executor<'core> {
    memory: &'core mut Memory,
    registers: &'core mut Registers,
    vec_engine: &'core mut VectorEngine,
}

impl<'m> Executor<'m> {
    pub fn new(
        registers: &'m mut Registers,
        memory: &'m mut Memory,
        vec_engine: &'m mut VectorEngine,
    ) -> Self {
        Self {
            registers,
            memory,
            vec_engine,
        }
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
                &self.vec_engine,
                &mut self.registers.c,
            ),
            Vsetivli(args) => v::vsetivli(
                args,
                &mut self.registers.x,
                &self.registers.v,
                &self.vec_engine,
                &mut self.registers.c,
            ),
            Vsetvl(args) => v::vsetvl(
                args,
                &mut self.registers.x,
                &self.registers.v,
                &self.vec_engine,
                &mut self.registers.c,
            ),

            Vlv { data: args, eew } => v::vl::v(
                args,
                SEW::try_from(eew).unwrap(),
                &self.registers.x,
                &mut self.registers.v,
                &self.vec_engine,
                &self.memory,
            ),
            Vsv { data: args, eew } => v::vs::v(
                args,
                SEW::try_from(eew).unwrap(),
                &self.registers.x,
                &self.registers.v,
                &self.vec_engine,
                &mut self.memory,
            ),

            Vlmv(args) => v::vlm::v(args, &mut self.registers.v, &self.vec_engine, &self.memory),
            Vsmv(args) => v::vsm::v(args, &self.registers.v, &self.vec_engine, &mut self.memory),

            Vlsv { data: args, eew } => v::vls::v(
                args,
                SEW::try_from(eew).unwrap(),
                &self.registers.x,
                &mut self.registers.v,
                &self.vec_engine,
                &self.memory,
            ),
            Vssv { data: args, eew } => v::vss::v(
                args,
                SEW::try_from(eew).unwrap(),
                &self.registers.v,
                &self.vec_engine,
                &self.registers.x,
                &mut self.memory,
            ),

            Vluxv { data: args, eew } => v::vlux::v(
                args,
                SEW::try_from(eew).unwrap(),
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
                &self.memory,
            ),
            Vloxv { data: args, eew } => v::vlox::v(
                args,
                SEW::try_from(eew).unwrap(),
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
                &self.memory,
            ),
            Vsuxv { data: args, eew } => v::vsux::v(
                args,
                SEW::try_from(eew).unwrap(),
                &self.registers.v,
                &self.vec_engine,
                &self.registers.x,
                &mut self.memory,
            ),
            Vsoxv { data: args, eew } => v::vsox::v(
                args,
                SEW::try_from(eew).unwrap(),
                &self.registers.v,
                &self.vec_engine,
                &self.registers.x,
                &mut self.memory,
            ),

            Vlffv { data: args, eew } => v::vlff::v(
                args,
                SEW::try_from(eew).unwrap(),
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
                &mut self.registers.c,
                &self.memory,
            ),

            Vlsegv {
                data: args,
                eew,
                nf,
            } => v::vlseg::v(
                args,
                SEW::try_from(eew).unwrap(),
                nf,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
                &self.memory,
            ),
            Vssegv {
                data: args,
                eew,
                nf,
            } => v::vsseg::v(
                args,
                eew,
                nf,
                &self.registers.v,
                &self.vec_engine,
                &mut self.memory,
            ),

            Vlssegv {
                data: args,
                eew,
                nf,
            } => v::vlsseg::v(
                args,
                SEW::try_from(eew).unwrap(),
                nf,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
                &self.memory,
            ),
            Vsssegv {
                data: args,
                eew,
                nf,
            } => v::vssseg::v(
                args,
                eew,
                nf,
                &self.registers.v,
                &self.vec_engine,
                &mut self.memory,
            ),

            Vluxsegv {
                data: args,
                eew,
                nf,
            } => v::vluxseg::v(
                args,
                SEW::try_from(eew).unwrap(),
                nf,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
                &self.memory,
            ),
            Vloxsegv {
                data: args,
                eew,
                nf,
            } => v::vloxseg::v(
                args,
                SEW::try_from(eew).unwrap(),
                nf,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
                &self.memory,
            ),
            Vsuxsegv {
                data: args,
                eew,
                nf,
            } => v::vsuxseg::v(
                args,
                SEW::try_from(eew).unwrap(),
                nf,
                &self.registers.v,
                &self.vec_engine,
                &self.registers.x,
                &mut self.memory,
            ),
            Vsoxsegv {
                data: args,
                eew,
                nf,
            } => v::vsoxseg::v(
                args,
                SEW::try_from(eew).unwrap(),
                nf,
                &self.registers.v,
                &self.vec_engine,
                &self.registers.x,
                &mut self.memory,
            ),

            Vlrv {
                data: args,
                eew,
                nf,
            } => v::vlr::v(
                args,
                eew,
                nf,
                &mut self.registers.v,
                &self.vec_engine,
                &self.memory,
            ),
            Vsrv { data: args, nf } => v::vsr::v(
                args,
                nf,
                &self.registers.v,
                &self.vec_engine,
                &mut self.memory,
            ),

            Vaddvv(args) => v::vadd::vv(args, &mut self.registers.v, &self.vec_engine),
            Vaddvx(args) => v::vadd::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),
            Vaddvi(args) => v::vadd::vi(args, &mut self.registers.v, &self.vec_engine),

            Vsubvv(args) => v::vsub::vv(args, &mut self.registers.v, &self.vec_engine),
            Vsubvx(args) => v::vsub::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vrsubvx(args) => v::vrsub::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),
            Vrsubvi(args) => v::vrsub::vi(args, &mut self.registers.v, &self.vec_engine),

            Vminuvv(args) => v::vminu::vv(args, &mut self.registers.v, &self.vec_engine),
            Vminuvx(args) => v::vminu::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vminvv(args) => v::vmin::vv(args, &mut self.registers.v, &self.vec_engine),
            Vminvx(args) => v::vmin::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vmaxuvv(args) => v::vmaxu::vv(args, &mut self.registers.v, &self.vec_engine),
            Vmaxuvx(args) => v::vmaxu::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vmaxvv(args) => v::vmax::vv(args, &mut self.registers.v, &self.vec_engine),
            Vmaxvx(args) => v::vmax::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vandvv(args) => v::vand::vv(args, &mut self.registers.v, &self.vec_engine),
            Vandvx(args) => v::vand::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),
            Vandvi(args) => v::vand::vi(args, &mut self.registers.v, &self.vec_engine),

            Vorvv(args) => v::vor::vv(args, &mut self.registers.v, &self.vec_engine),
            Vorvx(args) => v::vor::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),
            Vorvi(args) => v::vor::vi(args, &mut self.registers.v, &self.vec_engine),

            Vxorvv(args) => v::vxor::vv(args, &mut self.registers.v, &self.vec_engine),
            Vxorvx(args) => v::vxor::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),
            Vxorvi(args) => v::vxor::vi(args, &mut self.registers.v, &self.vec_engine),

            Vrgathervv(args) => v::vrgather::vv(args, &mut self.registers.v, &self.vec_engine),
            Vrgathervx(args) => v::vrgather::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),
            Vrgathervi(args) => v::vrgather::vi(args, &mut self.registers.v, &self.vec_engine),

            Vrgatherei16vv(args) => {
                v::vrgatherei16::vv(args, &mut self.registers.v, &self.vec_engine)
            }

            Vslideupvx(args) => v::vslideup::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),
            Vslideupvi(args) => v::vslideup::vi(args, &mut self.registers.v, &self.vec_engine),

            Vslidedownvx(args) => v::vslidedown::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),
            Vslidedownvi(args) => v::vslidedown::vi(args, &mut self.registers.v, &self.vec_engine),

            Vadcvvm(args) => v::vadc::vvm(args, &mut self.registers.v, &self.vec_engine),
            Vadcvxm(args) => v::vadc::vxm(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),
            Vadcvim(args) => v::vadc::vim(args, &mut self.registers.v, &self.vec_engine),

            Vmadcvvm(args) => v::vmadc::vvm(args, &mut self.registers.v, &self.vec_engine),
            Vmadcvxm(args) => v::vmadc::vxm(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),
            Vmadcvim(args) => v::vmadc::vim(args, &mut self.registers.v, &self.vec_engine),

            Vmadcvv(args) => v::vmadc::vv(args, &mut self.registers.v, &self.vec_engine),
            Vmadcvx(args) => v::vmadc::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),
            Vmadcvi(args) => v::vmadc::vi(args, &mut self.registers.v, &self.vec_engine),

            Vsbcvvm(args) => v::vsbc::vvm(args, &mut self.registers.v, &self.vec_engine),
            Vsbcvxm(args) => v::vsbc::vxm(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vmsbcvvm(args) => v::vmsbc::vvm(args, &mut self.registers.v, &self.vec_engine),
            Vmsbcvxm(args) => v::vmsbc::vxm(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vmsbcvv(args) => v::vmsbc::vv(args, &mut self.registers.v, &self.vec_engine),
            Vmsbcvx(args) => v::vmsbc::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vmergevvm(args) => v::vmerge::vvm(args, &mut self.registers.v, &self.vec_engine),
            Vmergevxm(args) => v::vmerge::vxm(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),
            Vmergevim(args) => v::vmerge::vim(args, &mut self.registers.v, &self.vec_engine),

            Vmvvv(args) => v::vmv::vv(args, &mut self.registers.v, &self.vec_engine),
            Vmvvx(args) => v::vmv::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),
            Vmvvi(args) => v::vmv::vi(args, &mut self.registers.v, &self.vec_engine),

            Vmseqvv(args) => v::vmseq::vv(args, &mut self.registers.v, &self.vec_engine),
            Vmseqvx(args) => v::vmseq::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),
            Vmseqvi(args) => v::vmseq::vi(args, &mut self.registers.v, &self.vec_engine),

            Vmsnevv(args) => v::vmsne::vv(args, &mut self.registers.v, &self.vec_engine),
            Vmsnevx(args) => v::vmsne::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),
            Vmsnevi(args) => v::vmsne::vi(args, &mut self.registers.v, &self.vec_engine),

            Vmsltuvv(args) => v::vmsltu::vv(args, &mut self.registers.v, &self.vec_engine),
            Vmsltuvx(args) => v::vmsltu::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vmsltvv(args) => v::vmslt::vv(args, &mut self.registers.v, &self.vec_engine),
            Vmsltvx(args) => v::vmslt::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vmsleuvv(args) => v::vmsleu::vv(args, &mut self.registers.v, &self.vec_engine),
            Vmsleuvx(args) => v::vmsleu::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),
            Vmsleuvi(args) => v::vmsleu::vi(args, &mut self.registers.v, &self.vec_engine),

            Vmslevv(args) => v::vmsle::vv(args, &mut self.registers.v, &self.vec_engine),
            Vmslevx(args) => v::vmsle::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),
            Vmslevi(args) => v::vmsle::vi(args, &mut self.registers.v, &self.vec_engine),

            Vmsgtuvx(args) => v::vmsgtu::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),
            Vmsgtuvi(args) => v::vmsgtu::vi(args, &mut self.registers.v, &self.vec_engine),

            Vmsgtvx(args) => v::vmsgt::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),
            Vmsgtvi(args) => v::vmsgt::vi(args, &mut self.registers.v, &self.vec_engine),

            Vsadduvv(args) => v::vsaddu::vv(args, &mut self.registers.v, &self.vec_engine),
            Vsadduvx(args) => v::vsaddu::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),
            Vsadduvi(args) => v::vsaddu::vi(args, &mut self.registers.v, &self.vec_engine),

            Vsaddvv(args) => v::vsadd::vv(args, &mut self.registers.v, &self.vec_engine),
            Vsaddvx(args) => v::vsadd::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),
            Vsaddvi(args) => v::vsadd::vi(args, &mut self.registers.v, &self.vec_engine),

            Vssubuvv(args) => v::vssubu::vv(args, &mut self.registers.v, &self.vec_engine),
            Vssubuvx(args) => v::vssubu::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vssubvv(args) => v::vssub::vv(args, &mut self.registers.v, &self.vec_engine),
            Vssubvx(args) => v::vssub::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vsllvv(args) => v::vsll::vv(args, &mut self.registers.v, &self.vec_engine),
            Vsllvx(args) => v::vsll::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),
            Vsllvi(args) => v::vsll::vi(args, &mut self.registers.v, &self.vec_engine),

            Vsmulvv(args) => v::vsmul::vv(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &mut self.registers.c,
            ),
            Vsmulvx(args) => v::vsmul::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
                &mut self.registers.c,
            ),

            Vmv1rv(args) => v::vmv1r::v(args, &mut self.registers.v, &self.vec_engine),
            Vmv2rv(args) => v::vmv2r::v(args, &mut self.registers.v, &self.vec_engine),
            Vmv4rv(args) => v::vmv4r::v(args, &mut self.registers.v, &self.vec_engine),
            Vmv8rv(args) => v::vmv8r::v(args, &mut self.registers.v, &self.vec_engine),

            Vsrlvv(args) => v::vsrl::vv(args, &mut self.registers.v, &self.vec_engine),
            Vsrlvx(args) => v::vsrl::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),
            Vsrlvi(args) => v::vsrl::vi(args, &mut self.registers.v, &self.vec_engine),

            Vsravv(args) => v::vsra::vv(args, &mut self.registers.v, &self.vec_engine),
            Vsravx(args) => v::vsra::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),
            Vsravi(args) => v::vsra::vi(args, &mut self.registers.v, &self.vec_engine),

            Vssrlvv(args) => v::vssrl::vv(args, &mut self.registers.v, &self.vec_engine),
            Vssrlvx(args) => v::vssrl::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),
            Vssrlvi(args) => v::vssrl::vi(args, &mut self.registers.v, &self.vec_engine),

            Vssravv(args) => v::vssra::vv(args, &mut self.registers.v, &self.vec_engine),
            Vssravx(args) => v::vssra::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),
            Vssravi(args) => v::vssra::vi(args, &mut self.registers.v, &self.vec_engine),

            Vnsrlwv(args) => v::vnsrl::wv(args, &mut self.registers.v, &self.vec_engine),
            Vnsrlwx(args) => v::vnsrl::wx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),
            Vnsrlwi(args) => v::vnsrl::wi(args, &mut self.registers.v, &self.vec_engine),

            Vnsrawv(args) => v::vnsra::wv(args, &mut self.registers.v, &self.vec_engine),
            Vnsrawx(args) => v::vnsra::wx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),
            Vnsrawi(args) => v::vnsra::wi(args, &mut self.registers.v, &self.vec_engine),

            Vnclipuwv(args) => v::vnclipu::wv(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &mut self.registers.c,
            ),
            Vnclipuwx(args) => v::vnclipu::wx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
                &mut self.registers.c,
            ),
            Vnclipuwi(args) => v::vnclipu::wi(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &mut self.registers.c,
            ),

            Vnclipwv(args) => v::vnclip::wv(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &mut self.registers.c,
            ),
            Vnclipwx(args) => v::vnclip::wx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
                &mut self.registers.c,
            ),
            Vnclipwi(args) => v::vnclip::wi(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &mut self.registers.c,
            ),

            Vwredsumuvs(args) => v::vwredsumu::vs(args, &mut self.registers.v, &self.vec_engine),
            Vwredsumvs(args) => v::vwredsum::vs(args, &mut self.registers.v, &self.vec_engine),

            Vredsumvs(args) => v::vredsum::vs(args, &mut self.registers.v, &self.vec_engine),
            Vredandvs(args) => v::vredand::vs(args, &mut self.registers.v, &self.vec_engine),
            Vredorvs(args) => v::vredor::vs(args, &mut self.registers.v, &self.vec_engine),
            Vredxorvs(args) => v::vredxor::vs(args, &mut self.registers.v, &self.vec_engine),
            Vredminuvs(args) => v::vredminu::vs(args, &mut self.registers.v, &self.vec_engine),
            Vredminvs(args) => v::vredmin::vs(args, &mut self.registers.v, &self.vec_engine),
            Vredmaxuvs(args) => v::vredmaxu::vs(args, &mut self.registers.v, &self.vec_engine),
            Vredmaxvs(args) => v::vredmax::vs(args, &mut self.registers.v, &self.vec_engine),

            Vaadduvv(args) => v::vaaddu::vv(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.c,
            ),
            Vaadduvx(args) => v::vaaddu::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
                &self.registers.c,
            ),

            Vaaddvv(args) => v::vaadd::vv(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.c,
            ),
            Vaaddvx(args) => v::vaadd::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
                &self.registers.c,
            ),

            Vasubuvv(args) => v::vasubu::vv(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.c,
            ),
            Vasubuvx(args) => v::vasubu::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
                &self.registers.c,
            ),

            Vasubvv(args) => v::vasub::vv(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.c,
            ),
            Vasubvx(args) => v::vasub::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
                &self.registers.c,
            ),

            Vslide1upvx(args) => v::vslide1up::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vslide1downvx(args) => v::vslide1down::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vmvxs(args) => v::vmv::xs(
                args,
                &self.registers.v,
                &self.vec_engine,
                &mut self.registers.x,
            ),
            Vcpopm(args) => v::vcpop::m(
                args,
                &self.registers.v,
                &self.vec_engine,
                &mut self.registers.x,
            ),
            Vfirstm(args) => v::vfirst::m(
                args,
                &self.registers.v,
                &self.vec_engine,
                &mut self.registers.x,
            ),

            Vmvsx(args) => v::vmv::sx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vzextvf8(args) => v::vzext::vf8(args, &mut self.registers.v, &self.vec_engine),
            Vsextvf8(args) => v::vsext::vf8(args, &mut self.registers.v, &self.vec_engine),
            Vzextvf4(args) => v::vzext::vf4(args, &mut self.registers.v, &self.vec_engine),
            Vsextvf4(args) => v::vsext::vf4(args, &mut self.registers.v, &self.vec_engine),
            Vzextvf2(args) => v::vzext::vf2(args, &mut self.registers.v, &self.vec_engine),
            Vsextvf2(args) => v::vsext::vf2(args, &mut self.registers.v, &self.vec_engine),

            Vmsbfm(args) => v::vmsbf::m(args, &mut self.registers.v, &self.vec_engine),
            Vmsofm(args) => v::vmsof::m(args, &mut self.registers.v, &self.vec_engine),
            Vmsifm(args) => v::vmsif::m(args, &mut self.registers.v, &self.vec_engine),
            Viotam(args) => v::viota::m(args, &mut self.registers.v, &self.vec_engine),
            Vidv(args) => v::vid::v(args, &mut self.registers.v, &self.vec_engine),

            Vcompressvm(args) => v::vcompress::vm(args, &mut self.registers.v, &self.vec_engine),

            Vmandnmm(args) => v::vmandn::mm(args, &mut self.registers.v, &self.vec_engine),
            Vmandmm(args) => v::vmand::mm(args, &mut self.registers.v, &self.vec_engine),
            Vmormm(args) => v::vmor::mm(args, &mut self.registers.v, &self.vec_engine),
            Vmxormm(args) => v::vmxor::mm(args, &mut self.registers.v, &self.vec_engine),
            Vmornmm(args) => v::vmorn::mm(args, &mut self.registers.v, &self.vec_engine),
            Vmnandmm(args) => v::vmnand::mm(args, &mut self.registers.v, &self.vec_engine),
            Vmnormm(args) => v::vmnor::mm(args, &mut self.registers.v, &self.vec_engine),
            Vmxnormm(args) => v::vmxnor::mm(args, &mut self.registers.v, &self.vec_engine),

            Vdivuvv(args) => v::vdivu::vv(args, &mut self.registers.v, &self.vec_engine),
            Vdivuvx(args) => v::vdivu::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vdivvv(args) => v::vdiv::vv(args, &mut self.registers.v, &self.vec_engine),
            Vdivvx(args) => v::vdiv::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vremuvv(args) => v::vremu::vv(args, &mut self.registers.v, &self.vec_engine),
            Vremuvx(args) => v::vremu::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vremvv(args) => v::vrem::vv(args, &mut self.registers.v, &self.vec_engine),
            Vremvx(args) => v::vrem::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vmulhuvv(args) => v::vmulhu::vv(args, &mut self.registers.v, &self.vec_engine),
            Vmulhuvx(args) => v::vmulhu::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vmulvv(args) => v::vmul::vv(args, &mut self.registers.v, &self.vec_engine),
            Vmulvx(args) => v::vmul::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vmulhsuvv(args) => v::vmulhsu::vv(args, &mut self.registers.v, &self.vec_engine),
            Vmulhsuvx(args) => v::vmulhsu::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vmulhvv(args) => v::vmulh::vv(args, &mut self.registers.v, &self.vec_engine),
            Vmulhvx(args) => v::vmulh::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vmaddvv(args) => v::vmadd::vv(args, &mut self.registers.v, &self.vec_engine),
            Vmaddvx(args) => v::vmadd::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vnmsubvv(args) => v::vnmsub::vv(args, &mut self.registers.v, &self.vec_engine),
            Vnmsubvx(args) => v::vnmsub::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vmaccvv(args) => v::vmacc::vv(args, &mut self.registers.v, &self.vec_engine),
            Vmaccvx(args) => v::vmacc::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vnmsacvv(args) => v::vnmsac::vv(args, &mut self.registers.v, &self.vec_engine),
            Vnmsacvx(args) => v::vnmsac::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vwadduvv(args) => v::vwaddu::vv(args, &mut self.registers.v, &self.vec_engine),
            Vwadduvx(args) => v::vwaddu::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vwaddvv(args) => v::vwadd::vv(args, &mut self.registers.v, &self.vec_engine),
            Vwaddvx(args) => v::vwadd::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vwsubuvv(args) => v::vwsubu::vv(args, &mut self.registers.v, &self.vec_engine),
            Vwsubuvx(args) => v::vwsubu::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vwsubvv(args) => v::vwsub::vv(args, &mut self.registers.v, &self.vec_engine),
            Vwsubvx(args) => v::vwsub::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vwadduwv(args) => v::vwaddu::wv(args, &mut self.registers.v, &self.vec_engine),
            Vwadduwx(args) => v::vwaddu::wx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vwaddwv(args) => v::vwadd::wv(args, &mut self.registers.v, &self.vec_engine),
            Vwaddwx(args) => v::vwadd::wx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vwsubuwv(args) => v::vwsubu::wv(args, &mut self.registers.v, &self.vec_engine),
            Vwsubuwx(args) => v::vwsubu::wx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vwsubwv(args) => v::vwsub::wv(args, &mut self.registers.v, &self.vec_engine),
            Vwsubwx(args) => v::vwsub::wx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vwmuluvv(args) => v::vwmulu::vv(args, &mut self.registers.v, &self.vec_engine),
            Vwmuluvx(args) => v::vwmulu::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vwmulsuvv(args) => v::vwmulsu::vv(args, &mut self.registers.v, &self.vec_engine),
            Vwmulsuvx(args) => v::vwmulsu::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vwmulvv(args) => v::vwmul::vv(args, &mut self.registers.v, &self.vec_engine),
            Vwmulvx(args) => v::vwmul::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vwmaccuvv(args) => v::vwmaccu::vv(args, &mut self.registers.v, &self.vec_engine),
            Vwmaccuvx(args) => v::vwmaccu::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vwmaccvv(args) => v::vwmacc::vv(args, &mut self.registers.v, &self.vec_engine),
            Vwmaccvx(args) => v::vwmacc::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vwmaccusvx(args) => v::vwmaccus::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vwmaccsuvv(args) => v::vwmaccsu::vv(args, &mut self.registers.v, &self.vec_engine),
            Vwmaccsuvx(args) => v::vwmaccsu::vx(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.x,
            ),

            Vfaddvv(args) => v::vfadd::vv(args, &mut self.registers.v, &self.vec_engine),
            Vfaddvf(args) => v::vfadd::vf(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.f,
            ),

            Vfredusumvs(args) => v::vfredusum::vs(args, &mut self.registers.v, &self.vec_engine),

            Vfsubvv(args) => v::vfsub::vv(args, &mut self.registers.v, &self.vec_engine),
            Vfsubvf(args) => v::vfsub::vf(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.f,
            ),

            Vfredosumvs(args) => v::vfredosum::vs(args, &mut self.registers.v, &self.vec_engine),

            Vfminvv(args) => v::vfmin::vv(args, &mut self.registers.v, &self.vec_engine),
            Vfminvf(args) => v::vfmin::vf(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.f,
            ),

            Vfredminvs(args) => v::vfredmin::vs(args, &mut self.registers.v, &self.vec_engine),

            Vfmaxvv(args) => v::vfmax::vv(args, &mut self.registers.v, &self.vec_engine),
            Vfmaxvf(args) => v::vfmax::vf(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.f,
            ),

            Vfredmaxvs(args) => v::vfredmax::vs(args, &mut self.registers.v, &self.vec_engine),

            Vfsgnjvv(args) => v::vfsgnj::vv(args, &mut self.registers.v, &self.vec_engine),
            Vfsgnjvf(args) => v::vfsgnj::vf(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.f,
            ),

            Vfsgnjnvv(args) => v::vfsgnjn::vv(args, &mut self.registers.v, &self.vec_engine),
            Vfsgnjnvf(args) => v::vfsgnjn::vf(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.f,
            ),

            Vfsgnjxvv(args) => v::vfsgnjx::vv(args, &mut self.registers.v, &self.vec_engine),
            Vfsgnjxvf(args) => v::vfsgnjx::vf(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.f,
            ),

            Vfslide1upvf(args) => v::vfslide1up::vf(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.f,
            ),

            Vfslide1downvf(args) => v::vfslide1down::vf(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.f,
            ),

            Vfmvfs(args) => v::vfmv::fs(
                args,
                &self.registers.v,
                &self.vec_engine,
                &mut self.registers.f,
            ),

            Vfmvsf(args) => v::vfmv::sf(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.f,
            ),

            Vfcvtxufv(args) => v::vfcvt::xufv(args, &mut self.registers.v, &self.vec_engine),
            Vfcvtxfv(args) => v::vfcvt::xfv(args, &mut self.registers.v, &self.vec_engine),
            Vfcvtfxuv(args) => v::vfcvt::fxuv(args, &mut self.registers.v, &self.vec_engine),
            Vfcvtfxv(args) => v::vfcvt::fxv(args, &mut self.registers.v, &self.vec_engine),
            VfcvtRtzxufv(args) => v::vfcvt::rtzxufv(args, &mut self.registers.v, &self.vec_engine),
            VfcvtRtzxfv(args) => v::vfcvt::rtzxfv(args, &mut self.registers.v, &self.vec_engine),

            Vfwcvtxufv(args) => v::vfwcvt::xufv(args, &mut self.registers.v, &self.vec_engine),
            Vfwcvtxfv(args) => v::vfwcvt::xfv(args, &mut self.registers.v, &self.vec_engine),
            Vfwcvtfxuv(args) => v::vfwcvt::fxuv(args, &mut self.registers.v, &self.vec_engine),
            Vfwcvtfxv(args) => v::vfwcvt::fxv(args, &mut self.registers.v, &self.vec_engine),
            Vfwcvtffv(args) => v::vfwcvt::ffv(args, &mut self.registers.v, &self.vec_engine),
            VfwcvtRtzxufv(args) => {
                v::vfwcvt::rtzxufv(args, &mut self.registers.v, &self.vec_engine)
            }
            VfwcvtRtzxfv(args) => v::vfwcvt::rtzxfv(args, &mut self.registers.v, &self.vec_engine),

            Vfncvtxufw(args) => v::vfncvt::xufw(args, &mut self.registers.v, &self.vec_engine),
            Vfncvtxfw(args) => v::vfncvt::xfw(args, &mut self.registers.v, &self.vec_engine),
            Vfncvtfxuw(args) => v::vfncvt::fxuw(args, &mut self.registers.v, &self.vec_engine),
            Vfncvtfxw(args) => v::vfncvt::fxw(args, &mut self.registers.v, &self.vec_engine),
            Vfncvtffw(args) => v::vfncvt::ffw(args, &mut self.registers.v, &self.vec_engine),
            VfncvtRodffw(args) => v::vfncvt::rodffw(args, &mut self.registers.v, &self.vec_engine),
            VfncvtRtzxufw(args) => {
                v::vfncvt::rtzxufw(args, &mut self.registers.v, &self.vec_engine)
            }
            VfncvtRtzxfw(args) => v::vfncvt::rtzxfw(args, &mut self.registers.v, &self.vec_engine),

            Vfsqrtv(args) => v::vfsqrt::v(args, &mut self.registers.v, &self.vec_engine),
            Vfrsqrt7v(args) => v::vfrsqrt7::v(args, &mut self.registers.v, &self.vec_engine),
            Vfrec7v(args) => v::vfrec7::v(args, &mut self.registers.v, &self.vec_engine),
            Vfclassv(args) => v::vfclass::v(args, &mut self.registers.v, &self.vec_engine),

            Vfmergevfm(args) => v::vfmerge::vfm(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.f,
            ),
            Vfmvvf(args) => v::vfmv::vf(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.f,
            ),

            Vmfeqvv(args) => v::vmfeq::vv(args, &mut self.registers.v, &self.vec_engine),
            Vmfeqvf(args) => v::vmfeq::vf(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.f,
            ),

            Vmflevv(args) => v::vmfle::vv(args, &mut self.registers.v, &self.vec_engine),
            Vmflevf(args) => v::vmfle::vf(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.f,
            ),

            Vmfltvv(args) => v::vmflt::vv(args, &mut self.registers.v, &self.vec_engine),
            Vmfltvf(args) => v::vmflt::vf(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.f,
            ),

            Vmfnevv(args) => v::vmfne::vv(args, &mut self.registers.v, &self.vec_engine),
            Vmfnevf(args) => v::vmfne::vf(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.f,
            ),

            Vmfgtvf(args) => v::vmfgt::vf(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.f,
            ),

            Vmfgevf(args) => v::vmfge::vf(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.f,
            ),

            Vfdivvv(args) => v::vfdiv::vv(args, &mut self.registers.v, &self.vec_engine),
            Vfdivvf(args) => v::vfdiv::vf(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.f,
            ),

            Vfrdivvf(args) => v::vfrdiv::vf(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.f,
            ),

            Vfmulvv(args) => v::vfmul::vv(args, &mut self.registers.v, &self.vec_engine),
            Vfmulvf(args) => v::vfmul::vf(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.f,
            ),

            Vfrsubvf(args) => v::vfrsub::vf(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.f,
            ),

            Vfmaddvv(args) => v::vfmadd::vv(args, &mut self.registers.v, &self.vec_engine),
            Vfmaddvf(args) => v::vfmadd::vf(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.f,
            ),

            Vfnmaddvv(args) => v::vfnmadd::vv(args, &mut self.registers.v, &self.vec_engine),
            Vfnmaddvf(args) => v::vfnmadd::vf(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.f,
            ),

            Vfmsubvv(args) => v::vfmsub::vv(args, &mut self.registers.v, &self.vec_engine),
            Vfmsubvf(args) => v::vfmsub::vf(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.f,
            ),

            Vfnmsubvv(args) => v::vfnmsub::vv(args, &mut self.registers.v, &self.vec_engine),
            Vfnmsubvf(args) => v::vfnmsub::vf(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.f,
            ),

            Vfmaccvv(args) => v::vfmacc::vv(args, &mut self.registers.v, &self.vec_engine),
            Vfmaccvf(args) => v::vfmacc::vf(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.f,
            ),

            Vfnmaccvv(args) => v::vfnmacc::vv(args, &mut self.registers.v, &self.vec_engine),
            Vfnmaccvf(args) => v::vfnmacc::vf(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.f,
            ),

            Vfmsacvv(args) => v::vfmsac::vv(args, &mut self.registers.v, &self.vec_engine),
            Vfmsacvf(args) => v::vfmsac::vf(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.f,
            ),

            Vfnmsacvv(args) => v::vfnmsac::vv(args, &mut self.registers.v, &self.vec_engine),
            Vfnmsacvf(args) => v::vfnmsac::vf(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.f,
            ),

            Vfwaddvv(args) => v::vfwadd::vv(args, &mut self.registers.v, &self.vec_engine),
            Vfwaddvf(args) => v::vfwadd::vf(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.f,
            ),

            Vfwredusumvs(args) => v::vfwredusum::vs(args, &mut self.registers.v, &self.vec_engine),

            Vfwsubvv(args) => v::vfwsub::vv(args, &mut self.registers.v, &self.vec_engine),
            Vfwsubvf(args) => v::vfwsub::vf(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.f,
            ),

            Vfwredosumvs(args) => v::vfwredosum::vs(args, &mut self.registers.v, &self.vec_engine),

            Vfwaddwv(args) => v::vfwadd::wv(args, &mut self.registers.v, &self.vec_engine),
            Vfwaddwf(args) => v::vfwadd::wf(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.f,
            ),

            Vfwsubwv(args) => v::vfwsub::wv(args, &mut self.registers.v, &self.vec_engine),
            Vfwsubwf(args) => v::vfwsub::wf(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.f,
            ),

            Vfwmulvv(args) => v::vfmul::vv(args, &mut self.registers.v, &self.vec_engine),
            Vfwmulvf(args) => v::vfmul::vf(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.f,
            ),

            Vfwmaccvv(args) => v::vfwmacc::vv(args, &mut self.registers.v, &self.vec_engine),
            Vfwmaccvf(args) => v::vfwmacc::vf(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.f,
            ),

            Vfwnmaccvv(args) => v::vfwnmacc::vv(args, &mut self.registers.v, &self.vec_engine),
            Vfwnmaccvf(args) => v::vfwnmacc::vf(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.f,
            ),

            Vfwmsacvv(args) => v::vfwmsac::vv(args, &mut self.registers.v, &self.vec_engine),
            Vfwmsacvf(args) => v::vfwmsac::vf(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.f,
            ),

            Vfwnmsacvv(args) => v::vfwnmsac::vv(args, &mut self.registers.v, &self.vec_engine),
            Vfwnmsacvf(args) => v::vfwnmsac::vf(
                args,
                &mut self.registers.v,
                &self.vec_engine,
                &self.registers.f,
            ),

            Fusion(first, second) => {
                self.execute(*first);
                self.execute(*second);
            }
        }

        self.registers.pc += 4;
    }
}
