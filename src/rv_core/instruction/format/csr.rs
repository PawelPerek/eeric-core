pub struct Csrr {
    pub rd: usize,
    pub rs1: usize,
    pub csr: usize,
}

pub struct Csri {
    pub rd: usize,
    pub uimm: usize,
    pub csr: usize,
}
