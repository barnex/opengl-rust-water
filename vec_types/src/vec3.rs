/*
use super::vector::*;

#[allow(non_camel_case_types)]
pub type vec3 = Vector3<f32>;

pub fn vec3(x: f32, y: f32) -> vec3 {
	Vector3(x, y)
}

/*
impl vec2 {
	/// Length (norm).
	///
	///     use brilliance::math::*;
	///     let v = Vectorf(0.0, 3.0, 4.0);
	///     assert_eq!(v.len(), 5.0);
	///
	#[inline]
	pub fn len(self) -> f32 {
		self.dot(self).sqrt()
	}

	/// Component-wise reciprocal.
	///
	///     use brilliance::math::*;
	///     let v = Vectorf(1.0, 2.0, 4.0);
	///     assert_eq!(v.inv(), Vectorf(1.0, 0.5, 0.25));
	///
	#[inline]
	pub fn inv(self) -> Self {
		Self::new(1.0 / self[0], 1.0 / self[1], 1.0 / self[2])
	}

	/// Component-wise minimum.
	///
	///     use brilliance::math::*;
	///     let a = Vectorf(1.0, 2.0, 3.0);
	///     let b = Vectorf(2.0, 1.0, 0.0);
	///     assert_eq!(Vectorf::min(a, b), Vectorf(1.0, 1.0, 0.0));
	///
	#[inline]
	pub fn min(a: Self, b: Self) -> Self {
		Self::new(f32::min(a[0], b[0]), f32::min(a[1], b[1]), f32::min(a[2], b[2]))
	}

	/// Component-wise maximum.
	///
	///     use brilliance::math::*;
	///     let a = Vectorf(1.0, 2.0, 3.0);
	///     let b = Vectorf(2.0, 1.0, 0.0);
	///     assert_eq!(Vectorf::max(a, b), Vectorf(2.0, 2.0, 3.0));
	///
	#[inline]
	pub fn max(a: Self, b: Self) -> Self {
		Self::new(f32::max(a[0], b[0]), f32::max(a[1], b[1]), f32::max(a[2], b[2]))
	}

	/// Returns a vector with the same direction but unit length.
	///
	///     use brilliance::math::*;
	///     let v = Vectorf(99.9, 0.0, 0.0);
	///     assert_eq!(v.normalized(), Vectorf(1.0, 0.0, 0.0));
	///
	#[inline]
	#[must_use]
	pub fn normalized(self) -> Self {
		self * (1. / self.len())
	}

	/// Re-scale the vector to unit length.
	///
	///     use brilliance::math::*;
	///     let mut v = Vectorf(99.9, 0.0, 0.0);
	///     v.normalize();
	///     assert_eq!(v, Vectorf(1.0, 0.0, 0.0));
	///
	#[inline]
	pub fn normalize(&mut self) {
		*self = self.normalized()
	}

	/// Test if the vector has approximately unit length.
	/// Intended for use with debug_assert! where a unit vector is expected.
	///
	///     use brilliance::math::*;
	///     assert!(!Vectorf(9.9, 0.0, 0.0).is_normalized());
	///     assert!( Vectorf(1.0, 0.0, 0.0).is_normalized());
	///
	pub fn is_normalized(&self) -> bool {
		(self.len() - 1.0).abs() < 1e-6
	}

	/// Test that all components are neither NaN nor infinite.
	/// Intended for use with debug_assert!
	///
	///     use brilliance::math::*;
	///     assert!(!Vectorf(0.0/0.0, 0.0, 0.0).is_finite());
	///     assert!(!Vectorf(1.0/0.0, 0.0, 0.0).is_finite());
	///     assert!( Vectorf(1.0, 2.0, 3.0).is_finite());
	///
	pub fn is_finite(&self) -> bool {
		self[0].is_finite() && self[1].is_finite() && self[2].is_finite()
	}

	/// The zero vector.
	pub const ZERO: Vectorf = Self { el: [0.0, 0.0, 0.0] };
	/// Unit vector along X.
	pub const EX: Vectorf = Self { el: [1.0, 0.0, 0.0] };
	/// Unit vector along Y.
	pub const EY: Vectorf = Self { el: [0.0, 1.0, 0.0] };
	/// Unit vector along Z.
	pub const EZ: Vectorf = Self { el: [0.0, 0.0, 1.0] };

	/// Minimum of all components.
	///
	///     use brilliance::math::*;
	///     let v = Vectorf(1.0, 2.0, 3.0);
	///     assert_eq!(v.min3(), 1.0);
	///
	#[inline]
	pub fn min3(self) -> f32 {
		f32::min(f32::min(self[0], self[1]), self[2])
	}

	/// Maximum of all components.
	///
	///     use brilliance::math::*;
	///     let v = Vectorf(1.0, 2.0, 3.0);
	///     assert_eq!(v.max3(), 3.0);
	///
	#[inline]
	pub fn max3(self) -> f32 {
		f32::max(f32::max(self[0], self[1]), self[2])
	}
}
*/
*/
