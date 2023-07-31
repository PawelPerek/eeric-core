#[derive(Clone, Default)]
pub struct Registers {
    reg_file: [u32; 31],
    pc: u32,
}

impl Registers {
    fn new_zeros() -> Registers {
        Registers {
            reg_file: [0; 31],
            pc: 0,
        }
    }

    fn get(&self, nth: usize) -> u32 {
        assert!(nth < 32);

        if nth == 0 {
            0
        } else {
            self.reg_file[nth - 1]
        }
    }

    fn set(&mut self, nth: usize, new_value: u32) {
        assert!(nth < 32);

        if nth != 0 {
            self.reg_file[nth - 1] = new_value;
        }
    }

    fn get_pc(&self) -> u32 {
        self.pc
    }

    fn set_pc(&mut self, new_value: u32) {
        self.pc = new_value;
    }
}
