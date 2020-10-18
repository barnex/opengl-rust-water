use super::check;
use gl::types::*;
use std::mem;

/// Create a buffer object.
/// http://docs.gl/gl4/glCreateBuffers
#[allow(non_snake_case)]
pub fn glCreateBuffer() -> GLuint {
	let mut result = 0;
	unsafe { gl::CreateBuffers(1, &mut result) };
	check::gl_error();
	result
}

/// Creates and initializes a buffer object's immutable data store.
/// http://docs.gl/gl4/glBufferStorage
#[allow(non_snake_case)]
pub fn glNamedBufferStorage<T>(buffer: GLuint, data: &[T], flags: GLbitfield)
where
	T: Sized + Copy + 'static,
{
	let bytes = data.len() * mem::size_of::<T>();
	unsafe { gl::NamedBufferStorage(buffer, bytes as isize, mem::transmute(&data[0]), flags) }
	check::gl_error();
}

/// Installs a program object as part of current rendering state.
/// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUseProgram.xhtml
#[allow(non_snake_case)]
pub fn glUseProgram(program: GLuint) {
	unsafe { gl::UseProgram(program) };
	check::gl_error();
}
