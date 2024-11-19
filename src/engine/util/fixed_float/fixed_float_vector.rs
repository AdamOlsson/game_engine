use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

use super::fixed_float::FixedFloat;

#[derive(Copy, Debug, Clone)]
pub struct FixedFloatVector {
    pub x: FixedFloat,
    pub y: FixedFloat,
    pub z: FixedFloat,
}

impl FixedFloatVector {
    pub fn new<T: Into<FixedFloat>>(x: T, y: T, z: T) -> Self {
        FixedFloatVector {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    pub fn rotate_z(&self, theta: &FixedFloat) -> Self {
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();
        Self::new(
            self.x * cos_theta - self.y * sin_theta,
            self.x * sin_theta + self.y * cos_theta,
            self.z,
        )
    }

    pub fn dot(&self, _rhs: &FixedFloatVector) -> FixedFloat {
        todo!();
    }

    pub fn magnitude2(&self) -> FixedFloat {
        todo!();
    }

    pub fn magnitude(&self) -> FixedFloat {
        self.magnitude2().sqrt()
    }

    pub fn normalize(&self) -> FixedFloatVector {
        todo!();
    }

    pub fn distance2(&self, _other: &Self) -> FixedFloat {
        todo!();
    }

    pub fn distance(&self, other: &Self) -> FixedFloat {
        self.distance2(other).sqrt()
    }
}

impl From<[f32; 3]> for FixedFloatVector {
    fn from(value: [f32; 3]) -> FixedFloatVector {
        FixedFloatVector::new(
            FixedFloat::from(value[0]),
            FixedFloat::from(value[1]),
            FixedFloat::from(value[2]),
        )
    }
}

impl From<cgmath::Vector3<f32>> for FixedFloatVector {
    fn from(vec: cgmath::Vector3<f32>) -> FixedFloatVector {
        FixedFloatVector::new(
            FixedFloat::from(vec.x),
            FixedFloat::from(vec.y),
            FixedFloat::from(vec.z),
        )
    }
}

impl Into<[f32; 3]> for FixedFloatVector {
    fn into(self) -> [f32; 3] {
        [self.x.into(), self.y.into(), self.z.into()]
    }
}

impl Into<cgmath::Vector3<f32>> for FixedFloatVector {
    fn into(self) -> cgmath::Vector3<f32> {
        cgmath::Vector3::new(self.x.into(), self.y.into(), self.z.into())
    }
}

impl Sub<Self> for FixedFloatVector {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl<'a, 'b> Sub<&'b FixedFloatVector> for &'a FixedFloatVector {
    type Output = FixedFloatVector;
    fn sub(self, other: &'b FixedFloatVector) -> FixedFloatVector {
        FixedFloatVector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for FixedFloatVector {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<'a, 'b> Add<&'b FixedFloatVector> for &'a FixedFloatVector {
    type Output = FixedFloatVector;
    fn add(self, other: &'b FixedFloatVector) -> FixedFloatVector {
        FixedFloatVector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<Self> for FixedFloatVector {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl AddAssign for FixedFloatVector {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<T> Div<T> for FixedFloatVector
where
    T: Into<FixedFloat>,
{
    type Output = FixedFloatVector;

    fn div(self, rhs: T) -> FixedFloatVector {
        let rhs = rhs.into();
        FixedFloatVector {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Mul<FixedFloat> for FixedFloatVector {
    type Output = FixedFloatVector;

    fn mul(self, rhs: FixedFloat) -> FixedFloatVector {
        FixedFloatVector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<FixedFloatVector> for f32 {
    type Output = FixedFloatVector;
    fn mul(self, rhs: FixedFloatVector) -> FixedFloatVector {
        FixedFloatVector {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.y,
        }
    }
}

impl Neg for FixedFloatVector {
    type Output = FixedFloatVector;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl PartialEq for FixedFloatVector {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
