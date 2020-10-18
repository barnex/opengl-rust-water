/*
	Acceleration function for 2D linear waves + damping.

	Acceleration = -gradient(height) - damping * velocity

	This is in "natural" units, leading to unit wave speed.
	Actual wave speed can be controlled by the time step (verlet.glsl).

	https://en.wikipedia.org/wiki/Wave_equation
*/
#version 450 core

layout (local_size_x = 16, local_size_y = 16) in;

layout (binding = 0, r32f) uniform readonly  image2D height;
layout (binding = 1, r32f) uniform readonly  image2D velocity;
layout (binding = 2, r32f) uniform writeonly image2D acceleration;

uniform float damping;

void main(){
	ivec2 xy = ivec2(gl_GlobalInvocationID.xy);

	float z0 = imageLoad(height, xy + ivec2( 0, 0)).r;
	float z1 = imageLoad(height, xy + ivec2(-1, 0)).r;
	float z2 = imageLoad(height, xy + ivec2( 1, 0)).r;
	float z3 = imageLoad(height, xy + ivec2( 0, 1)).r;
	float z4 = imageLoad(height, xy + ivec2( 0,-1)).r;
	float v  = imageLoad(velocity, xy).r;

	float a = 0.25 * (z1+z2+z3+z4) - z0;
	a -= damping * v;

	imageStore(acceleration, xy, vec4(a, 0.0, 0.0, 0.0));
}