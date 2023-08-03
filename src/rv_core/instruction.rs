pub mod format;

pub mod executor;

use format::{
    base::*,
    vector::*
};

pub enum Instruction {
    // Arithmetic Operations
    Add(R),
    Sub(R),
    Addi(I),
    Slt(R),
    Slti(I),
    Sltu(R),
    Sltiu(I),
    Lui(U),
    Auipc(U),

    // Logic operations
    And(R),
    Or(R),
    Xor(R),
    Andi(I),
    Ori(I),
    Xori(I),
    Sll(R),
    Srl(R),
    Sra(R),
    Slli(I),
    Srli(I),
    Srai(I),

    // Load/store operations
    Ld(I),
    Lw(I),
    Lh(I),
    Lb(I),
    Lwu(I),
    Lhu(I),
    Lbu(I),
    Sd(S),
    Sw(S),
    Sh(S),
    Sb(S),

    // Branching

    Beq(S),
    Bne(S),
    Bge(S),
    Bgeu(S),
    Blt(S),
    Bltu(S),
    Jal(U),
    Jalr(I),

    // Vector config

    Vaddvv(Opivv),
    Vaddvx(Opivx),
    Vaddvi(Opivi),
    Vmvvv(Opivv),
    Vmvvx(Opivx),
    Vmvvi(Opivi)

}