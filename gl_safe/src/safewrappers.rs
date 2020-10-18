extern crate gl;

use super::check;
use gl::types::*;
use std::ffi::CString;
use std::mem;
use std::ptr;

/// Bind a named texture to a texturing target.
/// https://khronos.org/registry/OpenGL-Refpages/gl4/html/glBindTexture.xhtml
#[allow(non_snake_case)]
pub fn glBindTexture(target: GLenum, texture: GLuint) {
	unsafe { gl::BindTexture(target, texture) };
	check::gl_error()
}

/// Select active texture unit.
/// https://khronos.org/registry/OpenGL-Refpages/gl4/html/glActiveTexture.xhtml
#[allow(non_snake_case)]
pub fn glActiveTexture(texture: GLenum) {
	assert!(texture >= gl::TEXTURE0); // argument must be TEXTURE0 + i;
	unsafe { gl::ActiveTexture(texture) };
	check::gl_error()
}

/// Generate (a single) texture name.
/// https://khronos.org/registry/OpenGL-Refpages/gl4/html/glGenTextures.xhtml
#[allow(non_snake_case)]
pub fn glGenTexture() -> GLuint {
	let n = 1;
	let mut textures = 0;
	unsafe { gl::GenTextures(n, &mut textures) };
	check::gl_error();
	textures
}

/// Specify a two-dimensional texture image.
/// https://khronos.org/registry/OpenGL-Refpages/gl4/html/glTexImage2D.xhtml
#[allow(non_snake_case)]
pub fn glTexImage2D<T>(target: GLenum, level: i32, internalformat: GLint, width: u32, height: u32, border: GLint, format: GLenum, type_: GLenum, data: Option<&[T]>)
where
	T: Sized + Copy + 'static,
{
	let data = match data {
		Some(slice) => unsafe { mem::transmute(&slice[0]) },
		None => ptr::null(),
	};
	// TODO: check width * height == data.len()
	// check sizeof T == size of GL type.
	let width = width as GLint;
	let height = height as GLint;
	unsafe { gl::TexImage2D(target, level, internalformat, width, height, border, format, type_, data) };
	check::gl_error()
}

/// Bind a level of a texture to an image unit.
/// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindImageTexture.xhtml
#[allow(non_snake_case)]
pub fn glBindImageTexture(unit: u32, texture: GLuint, level: i32, layered: bool, layer: i32, access: GLenum, format: GLenum) {
	unsafe { gl::BindImageTexture(unit, texture, level, layered as GLboolean, layer, access, format) };
	check::gl_error()
}

/// Defines a barrier ordering memory transactions.
/// https://khronos.org/registry/OpenGL-Refpages/gl4/html/glMemoryBarrier.xhtml
#[allow(non_snake_case)]
pub fn glMemoryBarrier(barriers: GLbitfield) {
	unsafe { gl::MemoryBarrier(barriers) };
	check::gl_error();
}
/// Launch one or more compute work groups.
/// https://khronos.org/registry/OpenGL-Refpages/gl4/html/glDispatchCompute.xhtml
#[allow(non_snake_case)]
pub fn glDispatchCompute(num_groups_x: u32, num_groups_y: u32, num_groups_z: u32) {
	unsafe { gl::DispatchCompute(num_groups_x, num_groups_y, num_groups_z) };
	check::gl_error()
}

/// Render primitives from array data.
/// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawArrays.xhtml
#[allow(non_snake_case)]
pub fn glDrawArrays(mode: GLenum, first: i32, count: i32) {
	unsafe { gl::DrawArrays(mode, first, count) };
	check::gl_error()
}

/// Specify clear values for the color buffers.
/// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearColor.xhtml
#[allow(non_snake_case)]
pub fn glClearColor(red: f32, green: f32, blue: f32, alpha: f32) {
	unsafe { gl::ClearColor(red, green, blue, alpha) };
	check::gl_error()
}

/// Clear buffers to preset values.
/// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClear.xhtml
#[allow(non_snake_case)]
pub fn glClear(mask: GLbitfield) {
	unsafe { gl::Clear(mask) };
	check::gl_error()
}

/// Returns a parameter from a program object.
/// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgram.xhtml.
/// TODO: could be more than 1 !!!
#[allow(non_snake_case)]
pub fn glGetProgramiv(program: GLuint, pname: GLenum, n: usize) -> Vec<i32> {
	let mut safezone: [i32; 128] = [0xdeadb3f; 128];
	unsafe { gl::GetProgramiv(program, pname, &mut safezone[0]) }
	let mut result = Vec::with_capacity(n);
	for i in 0..n {
		result.push(safezone[i]);
	}
	for i in n..128 {
		if safezone[i] != 0xdeadb3f {
			panic!("glGetProgramiv: argument n ({}) too small.", n);
		}
	}
	check::gl_error();
	result
}

/// Generate a (single) vertex array object name.
/// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenVertexArrays.xhtml
#[allow(non_snake_case)]
pub fn glGenVertexArray() -> GLuint {
	let mut arrays = 0;
	unsafe { gl::GenVertexArrays(1, &mut arrays) }
	check::gl_error();
	arrays
}

/// Generate a (single) buffer object name.
/// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenBuffers.xhtml
#[allow(non_snake_case)]
pub fn glGenBuffer() -> GLuint {
	let mut buffers = 0;
	unsafe { gl::GenBuffers(1, &mut buffers) }
	check::gl_error();
	buffers
}

/// Bind a vertex array object.
/// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindVertexArray.xhtml
#[allow(non_snake_case)]
pub fn glBindVertexArray(array: GLuint) {
	unsafe { gl::BindVertexArray(array) }
	check::gl_error();
}

/// Bind a named buffer object.
/// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindBuffer.xhtml
#[allow(non_snake_case)]
pub fn glBindBuffer(target: GLenum, buffer: GLuint) {
	unsafe { gl::BindBuffer(target, buffer) }
	check::gl_error()
}

/// bind a user-defined varying out variable to a fragment shader color number.
/// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindFragDataLocation.xhtml
#[allow(non_snake_case)]
pub fn glBindFragDataLocation(program: GLuint, colorNumber: GLuint, name: &str) {
	unsafe { gl::BindFragDataLocation(program, colorNumber, CString::new(name).unwrap().as_ptr()) };
	check::gl_error()
}

// /// Enable or disable a generic vertex attribute array.
// /// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnableVertexAttribArray.xhtml
// #[allow(non_snake_case)]
// pub fn glEnableVertexAttribArray(index: GLuint) {
// 	unsafe { gl::EnableVertexAttribArray(index) }
// 	check::gl_error()
// }

/// Creates and initializes a buffer object's data store.
/// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBufferData.xhtml
#[allow(non_snake_case)]
pub fn glBufferData<T>(target: GLenum, data: &[T], usage: GLenum)
where
	T: Sized + 'static,
{
	let size = (data.len() * mem::size_of::<T>()) as GLsizeiptr;
	let data = unsafe { mem::transmute(&data[0]) };
	unsafe { gl::BufferData(target, size, data, usage) };
	check::gl_error();
}

/// Define an array of generic vertex attribute data.
/// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribPointer.xhtml
#[allow(non_snake_case)]
pub fn glVertexAttribPointer(index: GLuint, size: i32, typ: GLenum, normalized: bool, stride: i32) {
	let ptr = ptr::null(); // TODO
	unsafe { gl::VertexAttribPointer(index, size, typ, normalized as GLboolean, stride, ptr) };
	check::gl_error()
}

/// return error information.
/// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetError.xhtml
#[allow(non_snake_case)]
pub fn glGetError() -> GLenum {
	unsafe { gl::GetError() }
}

/// Force execution of GL commands in finite time.
/// http://docs.gl/gl4/glFlush
#[allow(non_snake_case)]
pub fn glFlush() {
	unsafe { gl::Flush() };
}

/// Block until all GL execution is complete.
/// http://docs.gl/gl4/glFinish
#[allow(non_snake_case)]
pub fn glFinish() {
	unsafe { gl::Finish() };
}
