pub mod fadd;
pub mod fclass;
pub mod fcvt;
pub mod fdiv;
pub mod feq;
pub mod fle;
pub mod flt;
mod flw;
pub mod fmadd;
pub mod fmax;
pub mod fmin;
pub mod fmsub;
pub mod fmul;
pub mod fmv;
pub mod fnmadd;
pub mod fnmsub;
pub mod fsgnj;
pub mod fsgnjn;
pub mod fsgnjx;
pub mod fsqrt;
pub mod fsub;
mod fsw;

pub use flw::flw;
pub use fsw::fsw;

pub mod utils;