use crate::prelude::Snapshotable;

#[derive(Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
enum CsrPrivilege {
    ReadOnly,
    ReadWrite
}

#[derive(Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct CsrRegister {
    value: u64,
    privilege: CsrPrivilege
}

impl CsrRegister {
    pub fn read(&self) -> u64 {
        self.value
    }

    pub fn write(&mut self, value: u64) -> Result<(), String> {
        if self.privilege == CsrPrivilege::ReadOnly {
            return Err("Cannot write to read-only register".to_owned());
        }

        self.set(value);

        Ok(())
    }

    pub fn set(&mut self, value: u64) {
        self.value = value;
    }

}

#[derive(Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct CsrRegisters([CsrRegister; 4096]);

impl Snapshotable for CsrRegisters {
    type Snapshot = [CsrRegister; 4096];

    fn snapshot(&self) -> Self::Snapshot {
        self.0.clone()
    }
}

impl Default for CsrRegisters {
    fn default() -> Self {
        let mut index = 0;
        Self([0; 4096].map(|_| {
            let privilege = if ((index >> 10) & 0b11) == 0b11 { CsrPrivilege::ReadOnly } else { CsrPrivilege::ReadWrite };
            index += 1;
            CsrRegister { 
                value: 0, 
                privilege 
            }
        }))
    }
}

impl std::ops::Index<usize> for CsrRegisters {
    type Output = CsrRegister;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for CsrRegisters {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
