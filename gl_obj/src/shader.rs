use super::*;
use gl_safe::*;

#[derive(Copy, Clone)]
pub struct Shader(pub GLuint);

impl Shader {
	/// Creates a shader object.
	/// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateShader.xhtml
	pub fn create(shader_type: GLenum) -> Self {
		Self(glCreateShader(shader_type))
	}

	pub fn new_vert(src: &str) -> Self {
		Self::create(gl::VERTEX_SHADER).source(src).compile().expect("compile vertex sharder")
	}

	pub fn new_frag(src: &str) -> Self {
		Self::create(gl::FRAGMENT_SHADER).source(src).compile().expect("compile fragment sharder")
	}

	pub fn new_comp(src: &str) -> Self {
		Self::create(gl::COMPUTE_SHADER).source(src).compile().expect("compile compute sharder")
	}

	/// Replaces the source code in a shader object.
	/// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glShaderSource.xhtml
	pub fn source(self, src: &str) -> Self {
		glShaderSource(self.0, src);
		self
	}

	/// Compiles a shader object.
	/// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompileShader.xhtml
	#[must_use]
	pub fn compile(self) -> Result<Self, String> {
		glCompileShader(self.0);
		let status = self.get_iv(gl::COMPILE_STATUS);
		if status != (gl::TRUE as GLint) {
			Err(self.info_log())
		} else {
			Ok(self)
		}
	}

	/// Returns a parameter from a shader object.
	/// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetShader.xhtml
	/// TODO: iv is vector!
	pub fn get_iv(self, pname: GLenum) -> i32 {
		glGetShaderiv(self.0, pname)
	}

	/// Returns the information log for a shader object.
	/// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetShaderInfoLog.xhtml
	#[allow(non_snake_case)]
	pub fn info_log(self) -> String {
		glGetShaderInfoLog(self.0)
	}
}

impl Into<GLuint> for Shader {
	fn into(self) -> GLuint {
		self.0
	}
}
