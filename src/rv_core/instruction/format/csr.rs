#[derive(Clone, Debug)]
pub struct Csrr {
    pub rd: usize,
    pub rs1: usize,
    pub csr: usize,
}

#[derive(Clone, Debug)]
pub struct Csri {
    pub rd: usize,
    pub uimm: usize,
    pub csr: usize,
}
