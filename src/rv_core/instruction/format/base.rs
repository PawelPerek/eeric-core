
#[derive(Clone, Debug)]
pub struct R {
    pub rd: usize,
    pub rs1: usize,
    pub rs2: usize,
}

#[derive(Clone, Debug)]
pub struct I {
    pub rd: usize,
    pub rs1: usize,
    pub imm12: u64,
}

#[derive(Clone, Debug)]

pub struct S {
    pub rs1: usize,
    pub rs2: usize,
    pub imm12: u64,
}

#[derive(Clone, Debug)]
pub struct U {
    pub rd: usize,
    pub imm20: u64,
}
