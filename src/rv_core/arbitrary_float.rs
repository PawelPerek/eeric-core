use core::{
    cmp::Ordering,
    num::FpCategory,
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};

use num_traits::{Float, Num, NumCast, One, ToPrimitive, Zero};

// Note: RVV 1.1 spec defines a f16 type, maybe in the future :)

// Single precision and rest decomposition
pub fn decompose(f: f64) -> (f32, u32) {
    let bytes = f.to_le_bytes();
    let (low_bytes, high_bytes) = (
        bytes[0 .. 3].try_into().unwrap(), 
        bytes[4 .. 7].try_into().unwrap()
    );
    
    (
        f32::from_le_bytes(low_bytes),
        u32::from_le_bytes(high_bytes)
    )
}

// Double precision recombination
pub fn compose(float: f32, int: u32) -> f64 {
    let low_bytes = float.to_le_bytes();
    let high_bytes = int.to_le_bytes();

    let mut bytes = [0; 8];

    bytes[0 .. 7].copy_from_slice([&low_bytes[..], &high_bytes[..]].concat().as_slice());

    f64::from_le_bytes(bytes)
}

#[derive(Clone, Copy)]
pub enum ArbitraryFloat {
    F32(f32),
    F64(f64),
}

impl ArbitraryFloat {
    pub fn copy_type(other: &Self, value: f64) -> Self {
        match other {
            Self::F32(_) => Self::F32(decompose(value).0),
            Self::F64(_) => Self::F64(value),
        }
    }
}

impl Neg for ArbitraryFloat {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::F32(fp) => Self::F32(-fp),
            Self::F64(fp) => Self::F64(-fp),
        }
    }
}

impl PartialEq for ArbitraryFloat {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::F32(fp1), Self::F32(fp2)) => fp1.eq(fp2),
            (Self::F32(fp1), Self::F64(fp2)) => fp1.eq(&(*fp2 as f32)),
            (Self::F64(fp1), Self::F32(fp2)) => fp1.eq(&(*fp2 as f64)),
            (Self::F64(fp1), Self::F64(fp2)) => fp1.eq(fp2),
        }
    }
}

impl PartialOrd for ArbitraryFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::F32(fp1), Self::F32(fp2)) => fp1.partial_cmp(fp2),
            (Self::F32(fp1), Self::F64(fp2)) => fp1.partial_cmp(&(*fp2 as f32)),
            (Self::F64(fp1), Self::F32(fp2)) => fp1.partial_cmp(&(*fp2 as f64)),
            (Self::F64(fp1), Self::F64(fp2)) => fp1.partial_cmp(fp2),
        }
    }
}

impl ToPrimitive for ArbitraryFloat {
    fn to_i64(&self) -> Option<i64> {
        match self {
            Self::F32(fp) => fp.to_i64(),
            Self::F64(fp) => fp.to_i64(),
        }
    }

    fn to_u64(&self) -> Option<u64> {
        match self {
            Self::F32(fp) => fp.to_u64(),
            Self::F64(fp) => fp.to_u64(),
        }
    }

    fn to_isize(&self) -> Option<isize> {
        match self {
            Self::F32(fp) => fp.to_isize(),
            Self::F64(fp) => fp.to_isize(),
        }
    }

    fn to_i8(&self) -> Option<i8> {
        match self {
            Self::F32(fp) => fp.to_i8(),
            Self::F64(fp) => fp.to_i8(),
        }
    }

    fn to_i16(&self) -> Option<i16> {
        match self {
            Self::F32(fp) => fp.to_i16(),
            Self::F64(fp) => fp.to_i16(),
        }
    }

    fn to_i32(&self) -> Option<i32> {
        match self {
            Self::F32(fp) => fp.to_i32(),
            Self::F64(fp) => fp.to_i32(),
        }
    }

    fn to_usize(&self) -> Option<usize> {
        match self {
            Self::F32(fp) => fp.to_usize(),
            Self::F64(fp) => fp.to_usize(),
        }
    }

    fn to_u8(&self) -> Option<u8> {
        match self {
            Self::F32(fp) => fp.to_u8(),
            Self::F64(fp) => fp.to_u8(),
        }
    }

    fn to_u16(&self) -> Option<u16> {
        match self {
            Self::F32(fp) => fp.to_u16(),
            Self::F64(fp) => fp.to_u16(),
        }
    }

    fn to_u32(&self) -> Option<u32> {
        match self {
            Self::F32(fp) => fp.to_u32(),
            Self::F64(fp) => fp.to_u32(),
        }
    }

    fn to_f32(&self) -> Option<f32> {
        match self {
            Self::F32(fp) => Some(*fp),
            Self::F64(fp) => fp.to_f32(),
        }
    }

    fn to_f64(&self) -> Option<f64> {
        match self {
            Self::F32(fp) => fp.to_f64(),
            Self::F64(fp) => Some(*fp),
        }
    }
}

impl NumCast for ArbitraryFloat {
    fn from<T: ToPrimitive>(n: T) -> Option<Self> {
        match n.to_f64() {
            Some(fp) => Some(Self::F64(fp)),
            None => None,
        }
    }
}

impl Add for ArbitraryFloat {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::F32(fp1), Self::F32(fp2)) => Self::F32(fp1 + fp2),
            (Self::F32(fp1), Self::F64(fp2)) => Self::F32(fp1 + fp2 as f32),
            (Self::F64(fp1), Self::F32(fp2)) => Self::F64(fp1 + fp2 as f64),
            (Self::F64(fp1), Self::F64(fp2)) => Self::F64(fp1 + fp2),
        }
    }
}

impl Sub for ArbitraryFloat {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::F32(fp1), Self::F32(fp2)) => Self::F32(fp1 - fp2),
            (Self::F32(fp1), Self::F64(fp2)) => Self::F32(fp1 - fp2 as f32),
            (Self::F64(fp1), Self::F32(fp2)) => Self::F64(fp1 - fp2 as f64),
            (Self::F64(fp1), Self::F64(fp2)) => Self::F64(fp1 - fp2),
        }
    }
}

impl Mul for ArbitraryFloat {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::F32(fp1), Self::F32(fp2)) => Self::F32(fp1 * fp2),
            (Self::F32(fp1), Self::F64(fp2)) => Self::F32(fp1 * fp2 as f32),
            (Self::F64(fp1), Self::F32(fp2)) => Self::F64(fp1 * fp2 as f64),
            (Self::F64(fp1), Self::F64(fp2)) => Self::F64(fp1 * fp2),
        }
    }
}

impl Div for ArbitraryFloat {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::F32(fp1), Self::F32(fp2)) => Self::F32(fp1 / fp2),
            (Self::F32(fp1), Self::F64(fp2)) => Self::F32(fp1 / fp2 as f32),
            (Self::F64(fp1), Self::F32(fp2)) => Self::F64(fp1 / fp2 as f64),
            (Self::F64(fp1), Self::F64(fp2)) => Self::F64(fp1 / fp2),
        }
    }
}

impl Rem for ArbitraryFloat {
    type Output = Self;

    fn rem(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::F32(fp1), Self::F32(fp2)) => Self::F32(fp1 % fp2),
            (Self::F32(fp1), Self::F64(fp2)) => Self::F32(fp1 % fp2 as f32),
            (Self::F64(fp1), Self::F32(fp2)) => Self::F64(fp1 % fp2 as f64),
            (Self::F64(fp1), Self::F64(fp2)) => Self::F64(fp1 % fp2),
        }
    }
}

impl Zero for ArbitraryFloat {
    fn zero() -> Self {
        Self::F64(0.0)
    }

    fn is_zero(&self) -> bool {
        match self {
            Self::F32(fp) => fp.is_zero(),
            Self::F64(fp) => fp.is_zero(),
        }
    }
}

impl One for ArbitraryFloat {
    fn one() -> Self {
        Self::F64(1.0)
    }
}

impl Num for ArbitraryFloat {
    type FromStrRadixErr = <f64 as Num>::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        match f64::from_str_radix(str, radix) {
            Ok(fp) => Ok(Self::F64(fp)),
            Err(err) => Err(err),
        }
    }
}

impl Float for ArbitraryFloat {
    fn nan() -> Self {
        Self::F64(f64::nan())
    }

    fn infinity() -> Self {
        Self::F64(f64::infinity())
    }

    fn neg_infinity() -> Self {
        Self::F64(f64::neg_infinity())
    }

    fn neg_zero() -> Self {
        Self::F64(f64::neg_zero())
    }

    fn min_value() -> Self {
        Self::F64(f64::min_value())
    }

    fn min_positive_value() -> Self {
        Self::F64(f64::min_positive_value())
    }

    fn max_value() -> Self {
        Self::F64(f64::max_value())
    }

    fn is_nan(self) -> bool {
        match self {
            Self::F32(fp) => fp.is_nan(),
            Self::F64(fp) => fp.is_nan(),
        }
    }

    fn is_infinite(self) -> bool {
        match self {
            Self::F32(fp) => fp.is_infinite(),
            Self::F64(fp) => fp.is_infinite(),
        }
    }

    fn is_finite(self) -> bool {
        match self {
            Self::F32(fp) => fp.is_finite(),
            Self::F64(fp) => fp.is_finite(),
        }
    }

    fn is_normal(self) -> bool {
        match self {
            Self::F32(fp) => fp.is_normal(),
            Self::F64(fp) => fp.is_normal(),
        }
    }

    fn classify(self) -> FpCategory {
        match self {
            Self::F32(fp) => fp.classify(),
            Self::F64(fp) => fp.classify(),
        }
    }

    fn floor(self) -> Self {
        match self {
            Self::F32(fp) => Self::F32(fp.floor()),
            Self::F64(fp) => Self::F64(fp.floor()),
        }
    }

    fn ceil(self) -> Self {
        match self {
            Self::F32(fp) => Self::F32(fp.ceil()),
            Self::F64(fp) => Self::F64(fp.ceil()),
        }
    }

    fn round(self) -> Self {
        match self {
            Self::F32(fp) => Self::F32(fp.round()),
            Self::F64(fp) => Self::F64(fp.round()),
        }
    }

    fn trunc(self) -> Self {
        match self {
            Self::F32(fp) => Self::F32(fp.trunc()),
            Self::F64(fp) => Self::F64(fp.trunc()),
        }
    }

    fn fract(self) -> Self {
        match self {
            Self::F32(fp) => Self::F32(fp.fract()),
            Self::F64(fp) => Self::F64(fp.fract()),
        }
    }

    fn abs(self) -> Self {
        match self {
            Self::F32(fp) => Self::F32(fp.abs()),
            Self::F64(fp) => Self::F64(fp.abs()),
        }
    }

    fn signum(self) -> Self {
        match self {
            Self::F32(fp) => Self::F32(fp.signum()),
            Self::F64(fp) => Self::F64(fp.signum()),
        }
    }

    fn is_sign_positive(self) -> bool {
        match self {
            Self::F32(fp) => fp.is_sign_positive(),
            Self::F64(fp) => fp.is_sign_positive(),
        }
    }

    fn is_sign_negative(self) -> bool {
        match self {
            Self::F32(fp) => fp.is_sign_negative(),
            Self::F64(fp) => fp.is_sign_negative(),
        }
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        match self {
            Self::F32(fp) => Self::F32(fp.mul_add(a.to_f32().unwrap(), b.to_f32().unwrap())),
            Self::F64(fp) => Self::F64(fp.mul_add(a.to_f64().unwrap(), b.to_f64().unwrap())),
        }
    }

    fn recip(self) -> Self {
        match self {
            Self::F32(fp) => Self::F32(fp.recip()),
            Self::F64(fp) => Self::F64(fp.recip()),
        }
    }

    fn powi(self, n: i32) -> Self {
        match self {
            Self::F32(fp) => Self::F32(fp.powi(n)),
            Self::F64(fp) => Self::F64(fp.powi(n)),
        }
    }

    fn powf(self, n: Self) -> Self {
        match self {
            Self::F32(fp) => Self::F32(fp.powf(n.to_f32().unwrap())),
            Self::F64(fp) => Self::F64(fp.powf(n.to_f64().unwrap())),
        }
    }

    fn sqrt(self) -> Self {
        match self {
            Self::F32(fp) => Self::F32(fp.sqrt()),
            Self::F64(fp) => Self::F64(fp.sqrt()),
        }
    }

    fn exp(self) -> Self {
        match self {
            Self::F32(fp) => Self::F32(fp.exp()),
            Self::F64(fp) => Self::F64(fp.exp()),
        }
    }

    fn exp2(self) -> Self {
        match self {
            Self::F32(fp) => Self::F32(fp.exp2()),
            Self::F64(fp) => Self::F64(fp.exp2()),
        }
    }

    fn ln(self) -> Self {
        match self {
            Self::F32(fp) => Self::F32(fp.ln()),
            Self::F64(fp) => Self::F64(fp.ln()),
        }
    }

    fn log(self, base: Self) -> Self {
        match self {
            Self::F32(fp) => Self::F32(fp.log(base.to_f32().unwrap())),
            Self::F64(fp) => Self::F64(fp.log(base.to_f64().unwrap())),
        }
    }

    fn log2(self) -> Self {
        match self {
            Self::F32(fp) => Self::F32(fp.log2()),
            Self::F64(fp) => Self::F64(fp.log2()),
        }
    }

    fn log10(self) -> Self {
        match self {
            Self::F32(fp) => Self::F32(fp.log10()),
            Self::F64(fp) => Self::F64(fp.log10()),
        }
    }

    fn max(self, other: Self) -> Self {
        match self {
            Self::F32(fp) => Self::F32(fp.max(other.to_f32().unwrap())),
            Self::F64(fp) => Self::F64(fp.max(other.to_f64().unwrap())),
        }
    }

    fn min(self, other: Self) -> Self {
        match self {
            Self::F32(fp) => Self::F32(fp.min(other.to_f32().unwrap())),
            Self::F64(fp) => Self::F64(fp.min(other.to_f64().unwrap())),
        }
    }

    fn abs_sub(self, other: Self) -> Self {
        (self - other).abs()
    }

    fn cbrt(self) -> Self {
        match self {
            Self::F32(fp) => Self::F32(fp.cbrt()),
            Self::F64(fp) => Self::F64(fp.cbrt()),
        }
    }

    fn hypot(self, other: Self) -> Self {
        match self {
            Self::F32(fp) => Self::F32(fp.hypot(other.to_f32().unwrap())),
            Self::F64(fp) => Self::F64(fp.hypot(other.to_f64().unwrap())),
        }
    }

    fn sin(self) -> Self {
        match self {
            Self::F32(fp) => Self::F32(fp.sin()),
            Self::F64(fp) => Self::F64(fp.sin()),
        }
    }

    fn cos(self) -> Self {
        match self {
            Self::F32(fp) => Self::F32(fp.cos()),
            Self::F64(fp) => Self::F64(fp.cos()),
        }
    }

    fn tan(self) -> Self {
        match self {
            Self::F32(fp) => Self::F32(fp.tan()),
            Self::F64(fp) => Self::F64(fp.tan()),
        }
    }

    fn asin(self) -> Self {
        match self {
            Self::F32(fp) => Self::F32(fp.asin()),
            Self::F64(fp) => Self::F64(fp.asin()),
        }
    }

    fn acos(self) -> Self {
        match self {
            Self::F32(fp) => Self::F32(fp.acos()),
            Self::F64(fp) => Self::F64(fp.acos()),
        }
    }

    fn atan(self) -> Self {
        match self {
            Self::F32(fp) => Self::F32(fp.atan()),
            Self::F64(fp) => Self::F64(fp.atan()),
        }
    }

    fn atan2(self, other: Self) -> Self {
        match self {
            Self::F32(fp) => Self::F32(fp.atan2(other.to_f32().unwrap())),
            Self::F64(fp) => Self::F64(fp.atan2(other.to_f64().unwrap())),
        }
    }

    fn sin_cos(self) -> (Self, Self) {
        match self {
            Self::F32(fp) => {
                let (sin, cos) = fp.sin_cos();
                (Self::F32(sin), Self::F32(cos))
            }
            Self::F64(fp) => {
                let (sin, cos) = fp.sin_cos();
                (Self::F64(sin), Self::F64(cos))
            }
        }
    }

    fn exp_m1(self) -> Self {
        match self {
            Self::F32(fp) => Self::F32(fp.exp_m1()),
            Self::F64(fp) => Self::F64(fp.exp_m1()),
        }
    }

    fn ln_1p(self) -> Self {
        match self {
            Self::F32(fp) => Self::F32(fp.ln_1p()),
            Self::F64(fp) => Self::F64(fp.ln_1p()),
        }
    }

    fn sinh(self) -> Self {
        match self {
            Self::F32(fp) => Self::F32(fp.sinh()),
            Self::F64(fp) => Self::F64(fp.sinh()),
        }
    }

    fn cosh(self) -> Self {
        match self {
            Self::F32(fp) => Self::F32(fp.cosh()),
            Self::F64(fp) => Self::F64(fp.cosh()),
        }
    }

    fn tanh(self) -> Self {
        match self {
            Self::F32(fp) => Self::F32(fp.tanh()),
            Self::F64(fp) => Self::F64(fp.tanh()),
        }
    }

    fn asinh(self) -> Self {
        match self {
            Self::F32(fp) => Self::F32(fp.asinh()),
            Self::F64(fp) => Self::F64(fp.asinh()),
        }
    }

    fn acosh(self) -> Self {
        match self {
            Self::F32(fp) => Self::F32(fp.acosh()),
            Self::F64(fp) => Self::F64(fp.acosh()),
        }
    }

    fn atanh(self) -> Self {
        match self {
            Self::F32(fp) => Self::F32(fp.atanh()),
            Self::F64(fp) => Self::F64(fp.atanh()),
        }
    }

    fn integer_decode(self) -> (u64, i16, i8) {
        match self {
            Self::F32(fp) => fp.integer_decode(),
            Self::F64(fp) => fp.integer_decode(),
        }
    }

    fn epsilon() -> Self {
        Self::F64(f64::epsilon())
    }

    fn is_subnormal(self) -> bool {
        match self {
            Self::F32(fp) => fp.is_subnormal(),
            Self::F64(fp) => fp.is_subnormal(),
        }
    }
}