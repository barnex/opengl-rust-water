/*
	Calculate the normal vector of a height map.
*/

#version 450 core

layout (local_size_x = 16, local_size_y = 16) in;

layout (binding = 0, r32f)    uniform readonly  image2D height;
layout (binding = 1, rgba32f) uniform writeonly image2D normal;


void main(){
	ivec2 xy = ivec2(gl_GlobalInvocationID.xy);

	float hpx = imageLoad(height, xy + ivec2( 0, 1)).r;
	float hmx = imageLoad(height, xy + ivec2( 0,-1)).r;
	float hpy = imageLoad(height, xy + ivec2( 1, 0)).r;
	float hmy = imageLoad(height, xy + ivec2(-1, 0)).r;

	float partialx = 0.5*(hpx - hmx);
	float partialy = 0.5*(hpy - hmy);

	vec3 n = normalize(vec3(-partialx, -partialy, 1.0));

	imageStore(normal, xy, vec4(n, 0.0));
}