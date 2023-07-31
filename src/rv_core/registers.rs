pub mod integer;

use integer::IntegerRegisters;

#[derive(Clone, Default)]
pub struct Registers {
    x: IntegerRegisters
}

impl Registers {
    fn new_zeros() -> Registers {
        Registers {
            x: Default::default()
        }
    }
}
