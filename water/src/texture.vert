/*
	Passthrough vertex sharder
	for 2D vertex positions and texture coordinates.
*/
#version 450 core

in  vec2 vertex_pos;
in  vec2 vertex_tex_coord;
out vec2 frag_tex_coord;

void main() {
    gl_Position        = vec4(vertex_pos, 0.0, 1.0);
	frag_tex_coord = vertex_tex_coord;
}