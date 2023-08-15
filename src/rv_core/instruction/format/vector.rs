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
    pub vd: usize,
    pub vs1: usize,
    pub vs2: usize,
    pub vm: bool
}

// Vector - Scalar (Register)

pub struct Opivx {
    pub vd: usize,
    pub rs1: usize,
    pub vs2: usize,
    pub vm: bool
}

// Vector - Immediate
pub struct Opivi {
    pub vd: usize,
    pub imm5: u64,
    pub vs2: usize,
    pub vm: bool
}

/// OPM - Mask instructions

// Vector - Vector
pub struct Opmvv {
    pub dest: usize, // Note: can be either vd/rd
    pub vs1: usize,
    pub vs2: usize,
    pub vm: bool
}

// Vector - Scalar (Register)
pub struct Opmvx {
    pub dest: usize, // Note: can be either vd/rd
    pub rs1: usize,
    pub vs2: usize,
    pub vm: bool
}

/// OPF - Floating point instructions

// Vector - Vector
pub struct Opfvv {
    pub dest: usize, // Note: can be either vd/rd
    pub vs1: usize,
    pub vs2: usize,
    pub vm: bool
}

// Vector - Scalar (FP Register)
pub struct Opfvf {
    pub vd: usize,
    pub rs1: usize,
    pub vs2: usize,
    pub vm: bool
}

/// Other encoding spaces

// VRXUNARY0 - OPMVV with vs2 as function opcode
pub type Vrxunary0 = Opmvv;

// VWXUNARY0 - OPMVX with rs1 as function opcode
pub type Vwxunary0 = Opmvx;

// VXUNARY0 - OPMVV with vs1 as function opcode
pub type Vxunary0 = Opmvv;

// VMUNARY0 - OPMVV with vs1 as function opcode
pub type Vmunary0 = Opmvv;

// VWFUNARY0 - OPFVV with vs1 as function opcode
pub type Vwfunary0 = Opfvv;

// VRFUNARY0 - OPFVF with vs2 as function opcode
pub type Vrfunary0 = Opfvf;

// VFUNARY0 - OPFVV with vs1 as function opcode
pub type Vfunary0 = Opfvv;

// VFUNARY1 - OPFVV with vs1 as function opcode
pub type Vfunary1 = Opfvv;