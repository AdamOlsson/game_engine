use std::ops::{Add, Sub};

use super::fixed_float::FixedFloat;


pub struct FixedFloatVector {
    pub x: FixedFloat,
    pub y: FixedFloat,
    pub z: FixedFloat,
}

impl FixedFloatVector {
    pub fn new<T: Into<FixedFloat>>(x: T, y: T, z: T) -> Self {
        FixedFloatVector { x: x.into(), y: y.into(), z: z.into(),}
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

}

impl From<[f32;3]> for FixedFloatVector {
    fn from(value: [f32;3]) -> FixedFloatVector {
        FixedFloatVector::new(
            FixedFloat::from(value[0]), 
            FixedFloat::from(value[1]), 
            FixedFloat::from(value[2]),)
    } 
}

impl From<cgmath::Vector3<f32>> for FixedFloatVector {
    fn from(vec: cgmath::Vector3<f32>) -> FixedFloatVector {
        FixedFloatVector::new(
            FixedFloat::from(vec.x), 
            FixedFloat::from(vec.y), 
            FixedFloat::from(vec.z),)
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

impl Add<Self> for FixedFloatVector {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
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
