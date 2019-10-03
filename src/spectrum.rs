use vek::Clamp;

use crate::math::Vec3;
use std::fmt::Debug;
use std::iter::*;
use std::ops::*;

pub trait IsSpectrum:
    Add<Self, Output = Self>
    + AddAssign<Self>
    + Sub<Self, Output = Self>
    + SubAssign<Self>
    + Mul<Self, Output = Self>
    + MulAssign<Self>
    + Mul<f32, Output = Self>
    + Div<f32, Output = Self>
    + DivAssign<f32>
    + Sum
    + PartialEq
    + Sized
    + Clone
    + Copy
    + Debug
    + Into<Rgb>
    + From<Rgb>
    + Send
    + Sync
{
    fn zero() -> Self;
    fn one() -> Self;
    fn is_black(&self) -> bool;
    fn is_nan(&self) -> bool;
    fn max_channel(&self) -> f32;
}

type VekRgb = vek::vec::Rgb<f32>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgb(pub VekRgb);

impl From<Vec3> for Rgb {
    fn from(v: Vec3) -> Self {
        Rgb(VekRgb::from(v))
    }
}

impl Rgb {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Rgb(VekRgb::new(r, g, b))
    }

    #[allow(dead_code)]
    pub fn gamma_corrected(&self, gamma: f32) -> Self {
        Rgb(self.0.map(|x| x.powf(1.0 / gamma)))
    }

    #[allow(dead_code)]
    pub fn saturated(&self) -> Rgb {
        Rgb(self.0.map(|x| Clamp::clamped01(x)))
    }
}

impl Deref for Rgb {
    type Target = VekRgb;
    fn deref(&self) -> &VekRgb {
        &self.0
    }
}

impl IsSpectrum for Rgb {
    fn zero() -> Self {
        Rgb(VekRgb::zero())
    }

    fn one() -> Self {
        Rgb(VekRgb::one())
    }

    fn is_black(&self) -> bool {
        self.max_channel() < 0.0001
    }

    fn is_nan(&self) -> bool {
        self.r.is_nan() || self.g.is_nan() || self.b.is_nan()
    }

    fn max_channel(&self) -> f32 {
        self.0.reduce_partial_max()
    }
}

impl Sum for Rgb {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Rgb::zero(), |a, b| a + b)
    }
}

macro_rules! impl_wrapper_ops {
    ($wrapper_t:ident) => {
        impl ::std::ops::Add for $wrapper_t {
            type Output = $wrapper_t;

            fn add(self, other: $wrapper_t) -> $wrapper_t {
                $wrapper_t(self.0 + other.0)
            }
        }

        impl std::ops::AddAssign for $wrapper_t {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs
            }
        }

        impl ::std::ops::Sub for $wrapper_t {
            type Output = $wrapper_t;

            fn sub(self, other: $wrapper_t) -> $wrapper_t {
                $wrapper_t(self.0 - other.0)
            }
        }

        impl std::ops::SubAssign for $wrapper_t {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs
            }
        }

        impl ::std::ops::Div<f32> for $wrapper_t {
            type Output = $wrapper_t;

            fn div(self, other: f32) -> $wrapper_t {
                $wrapper_t(self.0 / other)
            }
        }

        impl std::ops::DivAssign<f32> for $wrapper_t {
            fn div_assign(&mut self, rhs: f32) {
                *self = *self / rhs
            }
        }

        impl ::std::ops::Mul<f32> for $wrapper_t {
            type Output = $wrapper_t;

            fn mul(self, other: f32) -> $wrapper_t {
                $wrapper_t(self.0 * other)
            }
        }

        impl std::ops::MulAssign<f32> for $wrapper_t {
            fn mul_assign(&mut self, rhs: f32) {
                *self = *self * rhs
            }
        }

        impl ::std::ops::Mul<$wrapper_t> for $wrapper_t {
            type Output = $wrapper_t;

            fn mul(self, other: $wrapper_t) -> $wrapper_t {
                $wrapper_t(self.0 * other.0)
            }
        }

        impl std::ops::MulAssign for $wrapper_t {
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs
            }
        }
    };
}

impl_wrapper_ops!(Rgb);
