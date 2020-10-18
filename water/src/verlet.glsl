/*
	Verlet (leapfrog) integration:
	update velocities and positions given accelartions.

	https://en.wikipedia.org/wiki/Leapfrog_integration
*/
#version 450 core

layout (local_size_x = 16, local_size_y = 16) in;

layout (binding = 0, r32f) uniform          image2D pos;
layout (binding = 1, r32f) uniform          image2D vel;
layout (binding = 2, r32f) uniform readonly image2D acc;

uniform float dt;

void main(){
	ivec2 xy = ivec2(gl_GlobalInvocationID.xy);

	float p = imageLoad(pos, xy).r;
	float v = imageLoad(vel, xy).r;
	float a = imageLoad(acc, xy).r;

	v = v + a * dt;
	p = p + v * dt;

	imageStore(pos, xy, vec4(p, 0.0, 0.0, 0.0));
	imageStore(vel, xy, vec4(v, 0.0, 0.0, 0.0));
}