/*
use super::float::Float;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::ops::*;

/// Generic 2-component vector.
/// Underlies the specialized types vec2, uvec2, ivec2,
/// which should be used instead of Vec2<f32>, Vec2<u32>, ...
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Vec2<T>(pub T, pub T);

impl<T> Add for Vec2<T>
where
	T: Add<T, Output = T> + Copy,
{
	type Output = Self;

	#[inline]
	fn add(self, rhs: Vec2<T>) -> Self::Output {
		Vec2(self.0 + rhs.0, self.1 + rhs.1)
	}
}

impl<T> AddAssign for Vec2<T>
where
	T: AddAssign + Copy,
{
	#[inline]
	fn add_assign(&mut self, rhs: Self) {
		self.0 += rhs.0;
		self.1 += rhs.1;
	}
}

impl<T> Display for Vec2<T>
where
	T: Display,
{
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
		write!(f, "({}, {})", self.0, self.1)
	}
}

impl<T> Div for Vec2<T>
where
	T: Div<T, Output = T> + Copy,
{
	type Output = Self;

	#[inline]
	fn div(self, rhs: Vec2<T>) -> Self::Output {
		Vec2(self.0 / rhs.0, self.1 / rhs.1)
	}
}

impl<T> Div<T> for Vec2<T>
where
	T: Div<T, Output = T> + Copy,
{
	type Output = Self;

	#[inline]
	fn div(self, rhs: T) -> Self::Output {
		Vec2(self.0 / rhs, self.1 / rhs)
	}
}

impl<T> Mul<T> for Vec2<T>
where
	T: Mul<T, Output = T> + Copy,
{
	type Output = Self;

	#[inline]
	fn mul(self, rhs: T) -> Self::Output {
		Vec2(self.0 * rhs, self.1 * rhs)
	}
}

impl<T> MulAssign<T> for Vec2<T>
where
	T: MulAssign + Copy,
{
	#[inline]
	fn mul_assign(&mut self, rhs: T) {
		self.0 *= rhs;
		self.1 *= rhs;
	}
}

impl<T> Neg for Vec2<T>
where
	T: Neg<Output = T> + Copy,
{
	type Output = Self;
	#[inline]
	fn neg(self) -> Self::Output {
		Vec2(-self.0, -self.1)
	}
}

impl<T> Sub for Vec2<T>
where
	T: Sub<T, Output = T> + Copy,
{
	type Output = Self;

	#[inline]
	fn sub(self, rhs: Vec2<T>) -> Self::Output {
		Vec2(self.0 - rhs.0, self.1 - rhs.1)
	}
}

impl<T> SubAssign for Vec2<T>
where
	T: SubAssign + Copy,
{
	#[inline]
	fn sub_assign(&mut self, rhs: Self) {
		self.0 -= rhs.0;
		self.1 -= rhs.1;
	}
}

impl<T: Float> Vec2<T> {}
*/
