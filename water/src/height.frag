/*
	This fragment shader colors according to the height of the water surface.

	 0 => black
	>0 => gold
	<0 => silver

	Used for debugging only.
*/
#version 450 core

uniform sampler2D tex_z;
in  vec2 frag_tex_coord;
out vec4 output_color;

void main() {
	float z = texture(tex_z, f_texc).r;
	if (z > 0.0){
		output_color = vec4(vec3(0.9, 0.7, 0.5) * z, 1.0);
	}else{
		output_color = vec4(vec3(0.5, 0.7, 0.9) * (-z), 1.0);
	}
}