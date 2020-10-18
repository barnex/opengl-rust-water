use super::*;
use gl_safe::*;
extern crate gl;

#[derive(Copy, Clone)]
pub struct VertexArray(pub GLuint);

impl VertexArray {
	/// Create a vertex array object.
	/// http://docs.gl/gl4/glCreateVertexArrays
	pub fn create() -> Self {
		Self(glCreateVertexArray())
	}

	/// Enable a generic vertex attribute array.
	/// http://docs.gl/gl4/glEnableVertexAttribArray
	pub fn enable_attrib(self, index: u32) -> Self {
		glEnableVertexArrayAttrib(self.0, index);
		self
	}

	/// Associate a vertex attribute and a vertex buffer binding for a vertex array object
	/// http://docs.gl/gl4/glVertexAttribBinding
	pub fn attrib_binding(self, attribindex: u32, bindingindex: u32) -> Self {
		glVertexArrayAttribBinding(self.0, attribindex, bindingindex);
		self
	}

	/// Specify the organization of vertex arrays.
	/// http://docs.gl/gl4/glVertexAttribFormat.
	pub fn attrib_format(self, attribindex: u32, size: i32, typ: GLenum, normalized: bool, relativeoffset: u32) -> Self {
		glVertexArrayAttribFormat(self.0, attribindex, size, typ, normalized, relativeoffset);
		self
	}

	/// Specify the organization of vertex arrays.
	/// http://docs.gl/gl4/glVertexAttribFormat.
	pub fn attrib_iformat(self, attribindex: u32, size: i32, typ: GLenum, relativeoffset: u32) -> Self {
		glVertexArrayAttribIFormat(self.0, attribindex, size, typ, relativeoffset);
		self
	}

	/// Specify the organization of vertex arrays.
	/// http://docs.gl/gl4/glVertexAttribFormat.
	pub fn attrib_lformat(self, attribindex: u32, size: i32, typ: GLenum, relativeoffset: u32) -> Self {
		glVertexArrayAttribLFormat(self.0, attribindex, size, typ, relativeoffset);
		self
	}

	/// Bind a buffer to a vertex buffer bind point.
	/// https://khronos.org/registry/OpenGL-Refpages/gl4/html/glBindVertexBuffer.xhtml
	pub fn vertex_buffer(self, bindingindex: GLuint, buffer: Buffer, offset: GLintptr, stride: GLsizei) -> Self {
		glVertexArrayVertexBuffer(self.0, bindingindex, buffer.into(), offset, stride);
		self
	}

	/// Bind a buffer to a vertex buffer bind point.
	/// https://khronos.org/registry/OpenGL-Refpages/gl4/html/glBindVertexBuffer.xhtml
	#[allow(non_snake_case)]
	pub fn bind(self) {
		glBindVertexArray(self.0)
	}
}

impl Into<GLuint> for VertexArray {
	fn into(self) -> GLuint {
		self.0
	}
}
