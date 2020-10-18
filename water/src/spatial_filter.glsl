/*
	Spatial blur filter.
	(unused).
*/
#version 450 core

layout (local_size_x = 16, local_size_y = 16) in;

layout (binding = 0, r32f) uniform readonly image2D src;
layout (binding = 1, r32f) uniform writeonly image2D dst;

uniform float strength = 1.0;

void main(){
	ivec2 pix = ivec2(gl_GlobalInvocationID.xy);

	float v0 = imageLoad(src, pix).r;
	float v1 = imageLoad(src, pix + ivec2(-1, 0)).r;
	float v2 = imageLoad(src, pix + ivec2(1, 0)).r;
	float v3 = imageLoad(src, pix + ivec2(0, 1)).r;
	float v4 = imageLoad(src, pix + ivec2(0, -1)).r;

	float filtered = v0 * 0.5 + (v1+v2+v3+v4) * 0.125;
	float result = (1.0-strength)*v0 + strength*filtered;

	imageStore(dst, pix, vec4(result));
}