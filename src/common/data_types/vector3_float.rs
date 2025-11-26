//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::common::SerializedLength;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
/// Custom vector type containing 3 single precision fields
pub struct Vector3Float {
    /// The first value within the vector
    pub x: f32,
    /// The second value within the vector
    pub y: f32,
    /// The third value within the vector
    pub z: f32,
}

impl Vector3Float {
    /// Creates a new vector with the given components.
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Creates a new vector with all components set to zero.
    #[must_use]
    pub const fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// Creates a new vector with all components set to one.
    #[must_use]
    pub const fn one() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    /// Creates a unit vector along the X axis.
    #[must_use]
    pub const fn unit_x() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }

    /// Creates a unit vector along the Y axis.
    #[must_use]
    pub const fn unit_y() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }

    /// Creates a unit vector along the Z axis.
    #[must_use]
    pub const fn unit_z() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }

    /// Computes the dot product with another vector.
    #[must_use]
    pub fn dot(&self, other: &Self) -> f32 {
        self.z
            .mul_add(other.z, self.x.mul_add(other.x, self.y * other.y))
    }

    /// Computes the cross product with another vector.
    #[must_use]
    pub fn cross(&self, other: &Self) -> Self {
        Self::new(
            self.y.mul_add(other.z, -(self.z * other.y)),
            self.z.mul_add(other.x, -(self.x * other.z)),
            self.x.mul_add(other.y, -(self.y * other.x)),
        )
    }

    /// Computes the magnitude (length) of the vector.
    #[must_use]
    pub fn magnitude(&self) -> f32 {
        self.z
            .mul_add(self.z, self.x.mul_add(self.x, self.y * self.y))
            .sqrt()
    }

    /// Returns a normalized version of this vector (unit length).
    #[must_use]
    pub fn normalize(&self) -> Option<Self> {
        let mag = self.magnitude();
        if mag == 0.0 {
            None
        } else {
            Some(Self::new(self.x / mag, self.y / mag, self.z / mag))
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_f32(self.x);
        buf.put_f32(self.y);
        buf.put_f32(self.z);
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        Self {
            x: buf.get_f32(),
            y: buf.get_f32(),
            z: buf.get_f32(),
        }
    }
}

impl std::ops::Add for Vector3Float {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl std::ops::Sub for Vector3Float {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl std::ops::Mul<f32> for Vector3Float {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl std::ops::Div<f32> for Vector3Float {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        Self::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

impl SerializedLength for Vector3Float {
    const LENGTH: usize = 12;
}

#[cfg(test)]
mod tests {
    use std::f32;

    use approx::relative_eq;

    use super::*;

    #[test]
    fn test_vector_operations() {
        let v1 = Vector3Float::new(1.0, 2.0, 3.0);
        let v2 = Vector3Float::new(4.0, 5.0, 6.0);

        // Test addition
        let sum = v1 + v2;
        assert_eq!(sum, Vector3Float::new(5.0, 7.0, 9.0));

        // Test dot product
        let dot = v1.dot(&v2);
        let _ = relative_eq!(dot, 32.0, epsilon = f32::EPSILON);

        // Test cross product
        let cross = v1.cross(&v2);
        assert_eq!(cross, Vector3Float::new(-3.0, 6.0, -3.0));

        // Test scalar multiplication
        let scaled = v1 * 2.0;
        assert_eq!(scaled, Vector3Float::new(2.0, 4.0, 6.0));

        // Test normalization
        let normalized = v1.normalize().unwrap_or_default();
        let mag = normalized.magnitude();
        assert!((mag - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_const_constructors() {
        let zero = Vector3Float::zero();
        assert_eq!(zero, Vector3Float::new(0.0, 0.0, 0.0));

        let unit_x = Vector3Float::unit_x();
        assert_eq!(unit_x, Vector3Float::new(1.0, 0.0, 0.0));
    }
}
