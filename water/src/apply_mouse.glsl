/*
	Add a gaussian disturbance around the mouse position.

	Used to trigger waves as the mouse moves over the water surface.
*/
#version 450 core

layout (local_size_x = 16, local_size_y = 16) in;

layout (binding = 0, r32f) uniform image2D dst;

uniform ivec2 mouse_pos;
uniform float mouse_pow;
uniform float mouse_rad;

#define PI 3.1415926535

void main(){
	ivec2 pix = ivec2(gl_GlobalInvocationID.xy);

	float d = imageLoad(dst, pix).r;
	vec2 pos = vec2(pix);
	vec2 mouse = vec2(mouse_pos);

	float mdist = distance(pos, mouse);
	if (mdist < mouse_rad){
		float x = mdist / (0.3*mouse_rad);
		float e = exp(-x*x);
		d -= e * mouse_pow;
	}

	imageStore(dst, pix, vec4(d, 0.0, 0.0, 0.0));
}