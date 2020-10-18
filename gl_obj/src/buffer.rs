use super::*;
use core::any::TypeId;
use gl_safe::*;
use std::mem::size_of;

#[derive(Clone)]
pub struct Buffer {
	handle: GLuint,
	len: usize,
	stride: u32,
	typeid: TypeId,
}

impl Buffer {
	/// Create a buffer object.
	/// http://docs.gl/gl4/glCreateBuffers
	pub fn create() -> Self {
		Self {
			handle: glCreateBuffer(),
			len: 0,
			stride: 0,
			typeid: TypeId::of::<()>(),
		}
	}

	/// Creates and initializes a buffer object's immutable data store.
	/// http://docs.gl/gl4/glBufferStorage
	pub fn storage<T>(self, data: &[T], flags: GLbitfield) -> Self
	where
		T: Sized + Copy + 'static,
	{
		glNamedBufferStorage(self.handle, data, flags);
		Self {
			typeid: TypeId::of::<T>(),
			stride: size_of::<T>() as u32,
			len: data.len(),
			..self
		}
	}

	pub fn stride(&self) -> i32 {
		self.stride as i32
	}

	pub fn len(&self) -> usize {
		self.len
	}

	pub fn bytes(&self) -> usize {
		self.len * (self.stride as usize)
	}

	//pub fn gl_type(&self) -> GLenum {
	//	match self.typeid {
	//		TypeId::of::<f32>() => gl::FLOAT,
	//	}
	//}
}

impl Into<GLuint> for Buffer {
	fn into(self) -> GLuint {
		self.handle
	}
}
