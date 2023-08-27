# eeric

An Easily Embeddable RIsc-v Core

# Design

eeric is a RV64I core with support for Zicsr, M, F, D and V extensions. I designed it with following design goals in mind:

- It's designed with WASM compilation in mind (although any cdylib target should work as well)
- It's doesn't support interrupts
- It's single threaded, hence no A extension support
- It's not designed to be most performant emulator, but it should be reasonably fast
- It's meant to be an abstract back-end machine, so it needs a front-end compiler or interpreter to work (see https://github.com/PawelPerek/rv-interpreter)

# Example

Let's consider following RISC-V Vector Algorithm from [RISCV Vector Spec examples](https://github.com/riscv/riscv-v-spec):

```
loop:
   vsetvli t0, a2, e8, m8, ta, ma   # Vectors of 8b
   vle8.v v0, (a1)               # Load bytes
     add a1, a1, t0              # Bump pointer
     sub a2, a2, t0              # Decrement count
   vse8.v v0, (a3)               # Store bytes
     add a3, a3, t0              # Bump pointer
     bnez a2, loop               # Any more?
     ret
```

It can be expressed as following eeric core:
```rust
use eeric::prelude::*;

fn main() {
    let core = RvCore::new_zeroed();

    // Important note: eeric as low-level back-end abstraction layer does not support pseudo-instructions
    // Burden of decoding pseudo-instructions is on the front-end layer
    // E.G: ret == I::Jalr (F::I { rd: alias::ZERO, rs1: alias::RA, imm: 0 }),

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
}
```

# Roadmap

As for version 0.0.1, `eeric` doesn't support a few vector instructions and it not yet usable as RISC-V Virtual Machine. They need to be implemented before bump to version 0.1.0.
Besides that, I keep an eye on following features:
 
 - Document code with comment docs
 - Support for bitmanip extension
 - Support for signalling NaN
 - Overhaul of interrupt handling
