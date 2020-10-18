use super::*;
use gl_safe::*;

#[derive(Copy, Clone)]
pub struct Program(pub GLuint);

impl Program {
	/// Creates a program object.
	/// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateProgram.xhtml
	pub fn create() -> Self {
		Self(glCreateProgram())
	}

	pub fn new(shaders: &[Shader]) -> Self {
		let p = Self::create();
		for s in shaders {
			p.attach_shader(*s);
		}
		p.link().expect("link program")
	}

	/// Attaches a shader object to a program object.
	/// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glAttachShader.xhtml
	pub fn attach_shader(self, shader: Shader) -> Self {
		glAttachShader(self.0, shader.into());
		self
	}

	/// Links a program object.
	/// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLinkProgram.xhtml
	#[must_use]
	pub fn link(self) -> Result<Self, String> {
		glLinkProgram(self.0);
		let status = glGetProgramiv(self.0, gl::LINK_STATUS, 1)[0];
		if status != (gl::TRUE as GLint) {
			Err(self.info_log())
		} else {
			Ok(self)
		}
	}

	/// Returns a parameter from a program object.
	/// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgram.xhtml.
	pub fn get_iv(self, pname: GLenum, n: usize) -> Vec<i32> {
		glGetProgramiv(self.0, pname, n)
	}

	/// Returns the information log for a program object.
	/// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramInfoLog.xhtml
	pub fn info_log(self) -> String {
		glGetProgramInfoLog(self.0)
	}

	/// Returns the location of an attribute variable.
	/// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetAttribLocation.xhtml
	pub fn attrib_location(self, name: &str) -> Option<u32> {
		let result = glGetAttribLocation(self.0, name);
		if result < 0 {
			None
		} else {
			Some(result as u32)
		}
	}

	/// Returns the location of a uniform variable.
	/// http://docs.gl/gl4/glGetUniformLocation
	pub fn uniform_location(self, name: &str) -> u32 {
		let result = glGetUniformLocation(self.0, name);
		if result < 0 {
			panic!("uniform location `{}` not found", name);
		} else {
			result as u32
		}
	}

	// TODO: ivec3
	pub fn compute_work_group_size(self) -> (u32, u32, u32) {
		let s = self.get_iv(gl::COMPUTE_WORK_GROUP_SIZE, 3);
		(s[0] as u32, s[1] as u32, s[2] as u32)
	}

	pub fn compute_and_sync(self, global_size: uvec3) {
		// TODO: glGetIntegerv(gl:CURRENT_PROGRAM) + restore
		glUseProgram(self.into());
		let wgs = self.compute_work_group_size();
		glDispatchCompute(global_size.0 / wgs.0, global_size.1 / wgs.1, global_size.2 / wgs.2);
		glMemoryBarrier(gl::ALL_BARRIER_BITS);
	}

	/// Specify the value of a uniform variable for a specified program object.
	/// http://docs.gl/gl4/glProgramUniform
	pub fn uniform4f(self, location: u32, v0: f32, v1: f32, v2: f32, v3: f32) -> Self {
		glUseProgram(self.0); // Weird that this is needed!
		glProgramUniform4f(self.0, location as i32, v0, v1, v2, v3);
		self
	}

	/// Specify the value of a uniform variable for a specified program object.
	/// http://docs.gl/gl4/glProgramUniform
	pub fn uniform3f(self, location: u32, v0: f32, v1: f32, v2: f32) -> Self {
		glUseProgram(self.0); // Weird that this is needed!
		glProgramUniform3f(self.0, location as i32, v0, v1, v2);
		self
	}

	/// Specify the value of a uniform variable for a specified program object.
	/// http://docs.gl/gl4/glProgramUniform
	pub fn uniform2f(self, location: u32, v0: f32, v1: f32) -> Self {
		glUseProgram(self.0); // Weird that this is needed!
		glProgramUniform2f(self.0, location as i32, v0, v1);
		self
	}

	/// Specify the value of a uniform variable for a specified program object.
	/// http://docs.gl/gl4/glProgramUniform
	pub fn uniform1f(self, location: u32, v0: f32) -> Self {
		glUseProgram(self.0); // Weird that this is needed!
		glProgramUniform1f(self.0, location as i32, v0);
		self
	}

	/// Specify the value of a uniform variable for a specified program object.
	/// http://docs.gl/gl4/glProgramUniform
	pub fn uniform4i(self, location: u32, v0: i32, v1: i32, v2: i32, v3: i32) -> Self {
		glUseProgram(self.0); // Weird that this is needed!
		glProgramUniform4i(self.0, location as i32, v0, v1, v2, v3);
		self
	}

	/// Specify the value of a uniform variable for a specified program object.
	/// http://docs.gl/gl4/glProgramUniform
	pub fn uniform3i(self, location: u32, v0: i32, v1: i32, v2: i32) -> Self {
		glUseProgram(self.0); // Weird that this is needed!
		glProgramUniform3i(self.0, location as i32, v0, v1, v2);
		self
	}

	/// Specify the value of a uniform variable for a specified program object.
	/// http://docs.gl/gl4/glProgramUniform
	pub fn uniform2i(self, location: u32, v0: i32, v1: i32) -> Self {
		glUseProgram(self.0); // Weird that this is needed!
		glProgramUniform2i(self.0, location as i32, v0, v1);
		self
	}

	/// Specify the value of a uniform variable for a specified program object.
	/// http://docs.gl/gl4/glProgramUniform
	pub fn uniform1i(self, location: u32, v0: i32) -> Self {
		glUseProgram(self.0); // Weird that this is needed!
		glProgramUniform1i(self.0, location as i32, v0);
		self
	}

	pub fn set1f(self, attrib: &str, v: f32) -> Self {
		let loc = self.uniform_location(attrib);
		self.uniform1f(loc, v);
		self
	}

	pub fn set1i(self, attrib: &str, v: i32) -> Self {
		let loc = self.uniform_location(attrib);
		self.uniform1i(loc, v);
		self
	}

	pub fn set2i(self, attrib: &str, v0: i32, v1: i32) -> Self {
		let loc = self.uniform_location(attrib);
		self.uniform2i(loc, v0, v1);
		self
	}

	/// Installs a program object as part of current rendering state.
	/// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUseProgram.xhtml
	pub fn use_program(self) {
		glUseProgram(self.0)
	}
}

impl Into<GLuint> for Program {
	fn into(self) -> GLuint {
		self.0
	}
}
