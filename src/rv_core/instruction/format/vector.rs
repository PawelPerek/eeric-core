/// Configuration formats

pub struct Vsetvli {
    rd: usize,
    rs1: usize,
    vtypei: u32
}

pub struct Vsetivli {
    rd: usize,
    uimm: u32,
    vtypei: u32
}

pub struct Vsetvl {
    rd: usize,
    rs1: usize,
    rs2: usize
}

/// OPI - Integer instructions

// Vector - Vector
pub struct Opivv {
    vd: usize,
    vs1: usize,
    vs2: usize
}

// Vector - Scalar (Register)

pub struct Opivx {
    vd: usize,
    rs1: usize,
    vs2: usize
}

// Vector - Immediate
pub struct Opivi {
    vd: usize,
    imm5: u32,
    vs2: usize
}

/// OPM - Mask instructions

// Vector - Vector
pub struct Opmvv {
    dest: usize, // Note: can be either vd/rd
    vs1: usize,
    vs2: usize
}

// Vector - Scalar (Register)
pub struct Opmvx {
    dest: usize, // Note: can be either vd/rd
    rs1: usize,
    vs2: usize
}

/// OPF - Floating point instructions

// Vector - Vector
pub struct Opfvv {
    dest: usize, // Note: can be either vd/rd
    vs1: usize,
    vs2: usize
}

// Vector - Scalar (FP Register)
pub struct Opfvf {
    vd: usize,
    rs1: usize,
    vs2: usize
}

