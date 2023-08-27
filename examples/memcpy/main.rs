use eeric::prelude::*;

// Example:
// loop:
//    vsetvli t0, a2, e8, m8, ta, ma   # Vectors of 8b
//    vle8.v v0, (a1)               # Load bytes
//      add a1, a1, t0              # Bump pointer
//      sub a2, a2, t0              # Decrement count
//    vse8.v v0, (a3)               # Store bytes
//      add a3, a3, t0              # Bump pointer
//      bnez a2, loop               # Any more?
//      ret

fn main() {
    let core = RvCore::new_zeroed();

    // Important note: eeric as low-level back-end abstraction layer does not support pseudo-instructions
    // Burden of decoding pseudo-instructions is on the front-end layer
    // E.G: ret == I::Jalr (F::I { rd: alias::ZERO, rs1: alias::RA, imm: 0 }),

    //TODO: vtypei! macro

    core.set_instructions(vec![
        I::Vsetvli(F::Vsetvli {
            rd: alias::T0,
            rs1: alias::A2,
            vtypei: vtypei! {e8, m8, ta, ma},
        }),
        I::Vlev {
            eew: 8,
            data: F::Vl {
                vd: 0,
                rs1: alias::A1,
            },
        },
        I::Add(F::R {
            rd: alias::A1,
            rs1: alias::A1,
            rs2: alias::T0,
        }),
        I::Sub(F::R {
            rd: alias::A2,
            rs1: alias::A2,
            rs2: alias::T0,
        }),
        I::Vsev {
            eew: 8,
            data: F::Vs {
                vs3: 0,
                rs1: alias::A3,
            },
        },
        I::Add(F::R {
            rd: alias::A3,
            rs1: alias::A3,
            rs2: alias::T0,
        }),
        I::Bne(F::B {
            rs1: alias::A2,
            rs2: alias::ZERO,
            imm: -24_i32 as usize,
        }),
        I::Jalr(F::I {
            rd: alias::ZERO,
            rs1: alias::RA,
            imm: 0,
        }),
    ]);

    for machine_state in core.executor() {
        println!("{:?}", machine_state);
    }
}
