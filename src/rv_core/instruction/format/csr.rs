#[derive(Clone)]
pub struct Csrr {
    pub rd: usize,
    pub rs1: usize,
    pub csr: usize,
}

#[derive(Clone)]
pub struct Csri {
    pub rd: usize,
    pub uimm: usize,
    pub csr: usize,
}
