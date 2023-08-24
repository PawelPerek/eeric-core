use core::{
    cmp::Ordering,
    num::FpCategory,
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};

use num_traits::{Float, Num, NumCast, One, ToPrimitive, Zero};

#[derive(Clone, Copy)]
pub struct ArbitraryFloat<F: Float>(F);

impl<F: Float> ArbitraryFloat<F> {
    pub fn new(value: F) -> Self {
        Self(value)
    }

    pub fn value(&self) -> F {
        self.0
    }
}

impl<F: Float> Neg for ArbitraryFloat<F> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl<F: Float> PartialEq for ArbitraryFloat<F> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<F: Float> PartialOrd for ArbitraryFloat<F> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<F: Float> ToPrimitive for ArbitraryFloat<F> {
    fn to_i64(&self) -> Option<i64> {
        self.0.to_i64()
    }

    fn to_u64(&self) -> Option<u64> {
        self.0.to_u64()
    }

    fn to_isize(&self) -> Option<isize> {
        self.0.to_isize()
    }

    fn to_i8(&self) -> Option<i8> {
        self.0.to_i8()
    }

    fn to_i16(&self) -> Option<i16> {
        self.0.to_i16()
    }

    fn to_i32(&self) -> Option<i32> {
        self.0.to_i32()
    }

    fn to_usize(&self) -> Option<usize> {
        self.0.to_usize()
    }

    fn to_u8(&self) -> Option<u8> {
        self.0.to_u8()
    }

    fn to_u16(&self) -> Option<u16> {
        self.0.to_u16()
    }

    fn to_u32(&self) -> Option<u32> {
        self.0.to_u32()
    }

    fn to_f32(&self) -> Option<f32> {
        self.0.to_f32()
    }

    fn to_f64(&self) -> Option<f64> {
        self.0.to_f64()
    }
}

impl<F: Float> NumCast for ArbitraryFloat<F> {
    fn from<T: ToPrimitive>(n: T) -> Option<Self> {
        F::from(n).map(Self)
    }
}

impl<F: Float> Add for ArbitraryFloat<F> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

impl<F: Float> Sub for ArbitraryFloat<F> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0)
    }
}

impl<F: Float> Mul for ArbitraryFloat<F> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self(self.0 * other.0)
    }
}

impl<F: Float> Div for ArbitraryFloat<F> {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self(self.0 / other.0)
    }
}

impl<F: Float> Rem for ArbitraryFloat<F> {
    type Output = Self;

    fn rem(self, other: Self) -> Self::Output {
        Self(self.0 % other.0)
    }
}

impl<F: Float> Zero for ArbitraryFloat<F> {
    fn zero() -> Self {
        Self(F::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl<F: Float> One for ArbitraryFloat<F> {
    fn one() -> Self {
        Self(F::one())
    }
}

impl<F: Float> Num for ArbitraryFloat<F> {
    type FromStrRadixErr = F::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        F::from_str_radix(str, radix).map(Self)
    }
}

impl<F: Float> Float for ArbitraryFloat<F> {
    fn nan() -> Self {
        Self(F::nan())
    }

    fn infinity() -> Self {
        Self(F::infinity())
    }

    fn neg_infinity() -> Self {
        Self(F::neg_infinity())
    }

    fn neg_zero() -> Self {
        Self(F::neg_zero())
    }

    fn min_value() -> Self {
        Self(F::min_value())
    }

    fn min_positive_value() -> Self {
        Self(F::min_positive_value())
    }

    fn max_value() -> Self {
        Self(F::max_value())
    }

    fn is_nan(self) -> bool {
        self.0.is_nan()
    }

    fn is_infinite(self) -> bool {
        self.0.is_infinite()
    }

    fn is_finite(self) -> bool {
        self.0.is_finite()
    }

    fn is_normal(self) -> bool {
        self.0.is_normal()
    }

    fn classify(self) -> FpCategory {
        self.0.classify()
    }

    fn floor(self) -> Self {
        Self(self.0.floor())
    }

    fn ceil(self) -> Self {
        Self(self.0.ceil())
    }

    fn round(self) -> Self {
        Self(self.0.round())
    }

    fn trunc(self) -> Self {
        Self(self.0.trunc())
    }

    fn fract(self) -> Self {
        Self(self.0.fract())
    }

    fn abs(self) -> Self {
        Self(self.0.abs())
    }

    fn signum(self) -> Self {
        Self(self.0.signum())
    }

    fn is_sign_positive(self) -> bool {
        self.0.is_sign_positive()
    }

    fn is_sign_negative(self) -> bool {
        self.0.is_sign_negative()
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        Self(self.0.mul_add(a.0, b.0))
    }

    fn recip(self) -> Self {
        Self(self.0.recip())
    }

    fn powi(self, n: i32) -> Self {
        Self(self.0.powi(n))
    }

    fn powf(self, n: Self) -> Self {
        Self(self.0.powf(n.0))
    }

    fn sqrt(self) -> Self {
        Self(self.0.sqrt())
    }

    fn exp(self) -> Self {
        Self(self.0.exp())
    }

    fn exp2(self) -> Self {
        Self(self.0.exp2())
    }

    fn ln(self) -> Self {
        Self(self.0.ln())
    }

    fn log(self, base: Self) -> Self {
        Self(self.0.log(base.0))
    }

    fn log2(self) -> Self {
        Self(self.0.log2())
    }

    fn log10(self) -> Self {
        Self(self.0.log10())
    }

    fn max(self, other: Self) -> Self {
        Self(self.0.max(other.0))
    }

    fn min(self, other: Self) -> Self {
        Self(self.0.min(other.0))
    }

    fn abs_sub(self, other: Self) -> Self {
        Self(self.0.abs_sub(other.0))
    }

    fn cbrt(self) -> Self {
        Self(self.0.cbrt())
    }

    fn hypot(self, other: Self) -> Self {
        Self(self.0.hypot(other.0))
    }

    fn sin(self) -> Self {
        Self(self.0.sin())
    }

    fn cos(self) -> Self {
        Self(self.0.cos())
    }

    fn tan(self) -> Self {
        Self(self.0.tan())
    }

    fn asin(self) -> Self {
        Self(self.0.asin())
    }

    fn acos(self) -> Self {
        Self(self.0.acos())
    }

    fn atan(self) -> Self {
        Self(self.0.atan())
    }

    fn atan2(self, other: Self) -> Self {
        Self(self.0.atan2(other.0))
    }

    fn sin_cos(self) -> (Self, Self) {
        let (sin, cos) = self.0.sin_cos();
        (Self(sin), Self(cos))
    }

    fn exp_m1(self) -> Self {
        Self(self.0.exp_m1())
    }

    fn ln_1p(self) -> Self {
        Self(self.0.ln_1p())
    }

    fn sinh(self) -> Self {
        Self(self.0.sinh())
    }

    fn cosh(self) -> Self {
        Self(self.0.cosh())
    }

    fn tanh(self) -> Self {
        Self(self.0.tanh())
    }

    fn asinh(self) -> Self {
        Self(self.0.asinh())
    }

    fn acosh(self) -> Self {
        Self(self.0.acosh())
    }

    fn atanh(self) -> Self {
        Self(self.0.atanh())
    }

    fn integer_decode(self) -> (u64, i16, i8) {
        self.0.integer_decode()
    }

    fn epsilon() -> Self {
        Self(F::epsilon())
    }

    fn is_subnormal(self) -> bool {
        self.0.is_subnormal()
    }
}
