use super::*;
use gl::types::*;
use std::ffi::CString;
use std::ptr;

/// Creates a program object.
/// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateProgram.xhtml
#[allow(non_snake_case)]
pub fn glCreateProgram() -> GLuint {
	let p = unsafe { gl::CreateProgram() };
	check::gl_error();
	p
}

/// Attaches a shader object to a program object.
/// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glAttachShader.xhtml
#[allow(non_snake_case)]
pub fn glAttachShader(program: GLuint, shader: GLuint) {
	unsafe { gl::AttachShader(program, shader) };
	check::gl_error()
}

/// Links a program object.
/// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLinkProgram.xhtml
#[allow(non_snake_case)]
pub fn glLinkProgram(program: GLuint) {
	unsafe { gl::LinkProgram(program) };
	check::gl_error()
}

/// Returns the information log for a program object.
/// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramInfoLog.xhtml
#[allow(non_snake_case)]
pub fn glGetProgramInfoLog(program: GLuint) -> String {
	let max_length = glGetProgramiv(program, gl::INFO_LOG_LENGTH, 1)[0];
	if max_length == 0 {
		return "".into();
	}
	let mut buf = Vec::with_capacity(max_length as usize);
	let length = ptr::null_mut();
	unsafe {
		buf.set_len((max_length as usize) - 1); // skip nul terminator
		let info_log = buf.as_mut_ptr() as *mut GLchar;
		gl::GetProgramInfoLog(program, max_length, length, info_log);
		check::gl_error();
	};
	String::from_utf8(buf).expect("invalid utf-8")
}

/// Returns the location of an attribute variable.
/// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetAttribLocation.xhtml
#[allow(non_snake_case)]
pub fn glGetAttribLocation(program: GLuint, name: &str) -> i32 {
	let result = unsafe { gl::GetAttribLocation(program, CString::new(name).unwrap().as_ptr()) };
	check::gl_error();
	result
}

/// Returns the location of a uniform variable.
/// http://docs.gl/gl4/glGetUniformLocation
#[allow(non_snake_case)]
pub fn glGetUniformLocation(program: GLuint, name: &str) -> i32 {
	let result = unsafe { gl::GetUniformLocation(program, CString::new(name).unwrap().as_ptr()) };
	check::gl_error();
	result
}

/// Specify the value of a uniform variable for a specified program object.
/// http://docs.gl/gl4/glProgramUniform
#[allow(non_snake_case)]
pub fn glProgramUniform4f(program: GLuint, location: i32, v0: f32, v1: f32, v2: f32, v3: f32) {
	unsafe { gl::ProgramUniform4f(program, location, v0, v1, v2, v3) };
	check::gl_error();
}

/// Specify the value of a uniform variable for a specified program object.
/// http://docs.gl/gl4/glProgramUniform
#[allow(non_snake_case)]
pub fn glProgramUniform3f(program: GLuint, location: i32, v0: f32, v1: f32, v2: f32) {
	unsafe { gl::ProgramUniform3f(program, location, v0, v1, v2) };
	check::gl_error();
}

/// Specify the value of a uniform variable for a specified program object.
/// http://docs.gl/gl4/glProgramUniform
#[allow(non_snake_case)]
pub fn glProgramUniform2f(program: GLuint, location: i32, v0: f32, v1: f32) {
	unsafe { gl::ProgramUniform2f(program, location, v0, v1) };
	check::gl_error();
}

/// Specify the value of a uniform variable for a specified program object.
/// http://docs.gl/gl4/glProgramUniform
#[allow(non_snake_case)]
pub fn glProgramUniform1f(program: GLuint, location: i32, v0: f32) {
	unsafe { gl::ProgramUniform1f(program, location, v0) };
	check::gl_error();
}

/// Specify the value of a uniform variable for a specified program object.
/// http://docs.gl/gl4/glProgramUniform
#[allow(non_snake_case)]
pub fn glProgramUniform4i(program: GLuint, location: i32, v0: i32, v1: i32, v2: i32, v3: i32) {
	unsafe { gl::ProgramUniform4i(program, location, v0, v1, v2, v3) };
	check::gl_error();
}

/// Specify the value of a uniform variable for a specified program object.
/// http://docs.gl/gl4/glProgramUniform
#[allow(non_snake_case)]
pub fn glProgramUniform3i(program: GLuint, location: i32, v0: i32, v1: i32, v2: i32) {
	unsafe { gl::ProgramUniform3i(program, location, v0, v1, v2) };
	check::gl_error();
}

/// Specify the value of a uniform variable for a specified program object.
/// http://docs.gl/gl4/glProgramUniform
#[allow(non_snake_case)]
pub fn glProgramUniform2i(program: GLuint, location: i32, v0: i32, v1: i32) {
	unsafe { gl::ProgramUniform2i(program, location, v0, v1) };
	check::gl_error();
}

/// Specify the value of a uniform variable for a specified program object.
/// http://docs.gl/gl4/glProgramUniform
#[allow(non_snake_case)]
pub fn glProgramUniform1i(program: GLuint, location: i32, v0: i32) {
	unsafe { gl::ProgramUniform1i(program, location, v0) };
	check::gl_error();
}
