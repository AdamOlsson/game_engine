use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};


const PRECISION: i32 = 3;

#[derive(Debug, Copy, Clone)]
pub struct FixedFloat {
    n: f32,
}

impl FixedFloat {
    pub fn from_array(arr: &[f32; 3]) -> [FixedFloat; 3] {
        [
            FixedFloat::from(arr[0]),
            FixedFloat::from(arr[1]),
            FixedFloat::from(arr[2]),
        ] 
    }

    pub fn from_cgmath_vector3(arr: &cgmath::Vector3<f32>) -> [FixedFloat; 3] {
        [
            FixedFloat::from(arr.x),
            FixedFloat::from(arr.y),
            FixedFloat::from(arr.z),
        ] 
    }

    pub fn cos(&self) -> Self {
        Self::from(self.n.cos())
    }

    pub fn sin(&self) -> Self {
        Self::from(self.n.sin())
    }

    pub fn min(&self, other: &FixedFloat) -> Self {
        FixedFloat::from(self.n.min(other.n))
    }

    pub fn max(&self, other: &FixedFloat) -> Self {
        FixedFloat::from(self.n.max(other.n))
    }
}

impl From<f32> for FixedFloat {
    fn from(value: f32) -> Self {
        Self { n : value } 
    }
}

impl Into<f32> for FixedFloat {
    fn into(self) -> f32 {
        (self.n * 10.0_f32.powi(PRECISION)).round()  / 10.0_f32.powi(PRECISION)
    }
}

impl<T> Mul<T> for FixedFloat
where
    T: Into<FixedFloat>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        let rhs_fixed = rhs.into();
        Self::from(self.n * rhs_fixed.n)
    }
}

impl Sub<Self> for FixedFloat {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::from(self.n - other.n)
    }
}

impl SubAssign for FixedFloat {
    fn sub_assign(&mut self, other: Self) {
        *self = Self { n : self.n - other.n };
    }
}

impl Add<Self> for FixedFloat {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::from(self.n + other.n)
    }
}

impl AddAssign for FixedFloat {
    fn add_assign(&mut self, other: Self) {
        *self = Self { n : self.n + other.n };
    }
}

impl Div<Self> for FixedFloat {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Self::from(self.n / rhs.n)
    }
}

impl Div<f32> for FixedFloat {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        Self::from(self.n / rhs)
    }
}

impl Neg for FixedFloat {
    type Output = Self;
    fn neg(self) -> Self {
        FixedFloat::from(-self.n)
    }
}

impl std::fmt::Display for FixedFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let float: f32 = self.n.into();
        write!(f, "{float}")
    }
}

#[cfg(test)]
mod tests {
    mod rounding {
        use super::super::FixedFloat;
        macro_rules! rounding_tests {
            ($($name:ident: $input: expr, $expected: expr)*) => {
                $(
                    #[test]
                    fn $name() {
                        let f: f32 = FixedFloat::from($input).into();
                        let expected = $expected;
                        assert_eq!($expected, f, "Expected {expected} found {f}");
                    }
                )*
            }
        }
        
        rounding_tests! {
            given_1_decimals_expect_1_decimals: 1.2, 1.2
            given_3_decimals_expect_3_decimals: 0.250, 0.250
            given_4_decimals_expect_3_decimals: 0.2501, 0.250
        }
    }

    mod cos {
        use super::super::FixedFloat;
        macro_rules! cos_tests {
            ($($name:ident: $input: expr, $expected: expr)*) => {
                $(
                    #[test]
                    fn $name() {
                        let input = $input;
                        let expected = $expected;
                        let f: f32 = FixedFloat::from(input).cos().into();
                        assert_eq!($expected, f, "Expected {expected} found {f}");
                    }
                )*
            }
        }

        cos_tests! {
            given_0_expect_1: 0.0, 1.0
            given_90_degrees_expect_0: std::f32::consts::PI/2.0, 0.0
            given_180_degrees_expect_neg_1: std::f32::consts::PI, -1.0
            given_270_degrees_expect_0: std::f32::consts::PI*3.0/2.0, 0.0
            given_45_degrees_expect_0_707: std::f32::consts::PI/4.0, 0.707
            given_neg_45_degrees_expect_0_707: std::f32::consts::PI/4.0, 0.707
            given_30_degrees_expect_0_5: std::f32::consts::PI/3.0, 0.5
        }
    }

    mod sin {
        use super::super::FixedFloat;
        macro_rules! sin_tests {
            ($($name:ident: $input: expr, $expected: expr)*) => {
                $(
                    #[test]
                    fn $name() {
                        let input = $input;
                        let expected = $expected;
                        let f: f32 = FixedFloat::from(input).sin().into();
                        assert_eq!($expected, f, "Expected {expected} found {f}");
                    }
                )*
            }
        }

        sin_tests! {
            given_0_expect_0: 0.0, 0.0
            given_90_degrees_expect_1: std::f32::consts::PI/2.0, 1.0
            given_180_degrees_expect_0: std::f32::consts::PI, 0.0
            given_270_degrees_expect_neg_1: std::f32::consts::PI*3.0/2.0, -1.0
            given_45_degrees_expect_0_707: std::f32::consts::PI/4.0, 0.707
            given_neg_45_degrees_expect_0_707: std::f32::consts::PI/4.0, 0.707
            given_60_degrees_expect_0_5: std::f32::consts::PI/6.0, 0.5
        }
    }

}
