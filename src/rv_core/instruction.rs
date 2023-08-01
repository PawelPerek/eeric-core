pub mod format;

use format::{
    base::*,
    vector::*
};

pub enum Instruction {
    // Arithmetic instructions

    Add(R),
    Sub(R),
    Addi(I),
    Slt(R),
    Slti(I),
    Sltu(R),
    Sltiu(I),
    Lui(U),
    Auip(U),

    // Logic instructions

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

    // Vector config

}