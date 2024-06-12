use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use num_traits::{Float, Num, NumCast, One, ToPrimitive, Zero};
use serde::{Deserialize, Serialize};

use crate::algorithm::GeoNum;

#[derive(Serialize, Deserialize, Eq, PartialEq, PartialOrd, Clone, Copy, Debug, Hash, Default)]
pub struct NoValue;

impl Add for NoValue {
    type Output = Self;

    #[inline]
    fn add(self, _: Self) -> Self::Output {
        NoValue
    }
}

impl<T> Div<T> for NoValue {
    type Output = Self;

    #[inline]
    fn div(self, _: T) -> Self::Output {
        NoValue
    }
}

impl<T> Mul<T> for NoValue {
    type Output = Self;

    #[inline]
    fn mul(self, _: T) -> Self::Output {
        NoValue
    }
}

impl Neg for NoValue {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        NoValue
    }
}

impl<T> Rem<T> for NoValue {
    type Output = Self;

    #[inline]
    fn rem(self, _: T) -> Self::Output {
        NoValue
    }
}

impl Sub for NoValue {
    type Output = Self;

    #[inline]
    fn sub(self, _: Self) -> Self::Output {
        NoValue
    }
}

/// This hack allows mathematical operations that result in noop due to above ops
impl Zero for NoValue {
    #[inline]
    fn zero() -> Self {
        NoValue
    }

    #[inline]
    fn is_zero(&self) -> bool {
        true
    }
}

/// These hacks allows mathematical operations that result in noop due to above ops
impl One for NoValue {
    #[inline]
    fn one() -> Self {
        NoValue
    }
}

impl ToPrimitive for NoValue {
    #[inline]
    fn to_i64(&self) -> Option<i64> {
        None
    }

    #[inline]
    fn to_u64(&self) -> Option<u64> {
        None
    }

    #[inline]
    fn to_f32(&self) -> Option<f32> {
        None
    }

    #[inline]
    fn to_f64(&self) -> Option<f64> {
        None
    }
}

impl NumCast for NoValue {
    fn from<T: ToPrimitive>(_: T) -> Option<Self> {
        None
    }
}

impl Num for NoValue {
    type FromStrRadixErr = ();

    fn from_str_radix(_str: &str, _radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Err(())
    }
}

impl Float for NoValue {
    fn nan() -> Self {
        NoValue
    }

    fn infinity() -> Self {
        NoValue
    }

    fn neg_infinity() -> Self {
        NoValue
    }

    fn neg_zero() -> Self {
        NoValue
    }

    fn min_value() -> Self {
        NoValue
    }

    fn min_positive_value() -> Self {
        NoValue
    }

    fn max_value() -> Self {
        NoValue
    }

    fn is_nan(self) -> bool {
        true
    }

    fn is_infinite(self) -> bool {
        true
    }

    fn is_finite(self) -> bool {
        false
    }

    fn is_normal(self) -> bool {
        false
    }

    fn classify(self) -> std::num::FpCategory {
        std::num::FpCategory::Nan
    }

    fn floor(self) -> Self {
        NoValue
    }

    fn ceil(self) -> Self {
        NoValue
    }

    fn round(self) -> Self {
        NoValue
    }

    fn trunc(self) -> Self {
        NoValue
    }

    fn fract(self) -> Self {
        NoValue
    }

    fn abs(self) -> Self {
        NoValue
    }

    fn signum(self) -> Self {
        NoValue
    }

    fn is_sign_positive(self) -> bool {
        false
    }

    fn is_sign_negative(self) -> bool {
        false
    }

    fn mul_add(self, _a: Self, _b: Self) -> Self {
        NoValue
    }

    fn recip(self) -> Self {
        NoValue
    }

    fn powi(self, _: i32) -> Self {
        NoValue
    }

    fn powf(self, _: Self) -> Self {
        NoValue
    }

    fn sqrt(self) -> Self {
        NoValue
    }

    fn exp(self) -> Self {
        NoValue
    }

    fn exp2(self) -> Self {
        NoValue
    }

    fn ln(self) -> Self {
        NoValue
    }

    fn log(self, _: Self) -> Self {
        NoValue
    }

    fn log2(self) -> Self {
        NoValue
    }

    fn log10(self) -> Self {
        NoValue
    }

    fn max(self, _other: Self) -> Self {
        NoValue
    }

    fn min(self, _other: Self) -> Self {
        NoValue
    }

    fn abs_sub(self, _other: Self) -> Self {
        NoValue
    }

    fn cbrt(self) -> Self {
        NoValue
    }

    fn hypot(self, _other: Self) -> Self {
        NoValue
    }

    fn sin(self) -> Self {
        NoValue
    }

    fn cos(self) -> Self {
        NoValue
    }

    fn tan(self) -> Self {
        NoValue
    }

    fn asin(self) -> Self {
        NoValue
    }

    fn acos(self) -> Self {
        NoValue
    }

    fn atan(self) -> Self {
        NoValue
    }

    fn atan2(self, _other: Self) -> Self {
        NoValue
    }

    fn sin_cos(self) -> (Self, Self) {
        (NoValue, NoValue)
    }

    fn exp_m1(self) -> Self {
        NoValue
    }

    fn ln_1p(self) -> Self {
        NoValue
    }

    fn sinh(self) -> Self {
        NoValue
    }

    fn cosh(self) -> Self {
        NoValue
    }

    fn tanh(self) -> Self {
        NoValue
    }

    fn asinh(self) -> Self {
        NoValue
    }

    fn acosh(self) -> Self {
        NoValue
    }

    fn atanh(self) -> Self {
        NoValue
    }

    fn integer_decode(self) -> (u64, i16, i8) {
        todo!()
    }
}

impl GeoNum for NoValue {
    fn total_cmp(&self, _other: &Self) -> Ordering {
        Ordering::Equal
    }
}
