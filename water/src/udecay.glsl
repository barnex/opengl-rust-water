/*
	Divide the colors in an unsigned RGB image by 2.

	Used to smooth the photon map over time:
	instead of resetting it to zero every time, we
	divide it by 2. So the final photon map
	holds the current light + 1/2 the light of the
	previous time step, + 1/4 of the step before that + ...

	This kind filtering is cheap and yields a modest
	improvement in visual quality (reduces noise).
*/
#version 450 core

layout(binding = 0, rgba8ui) uniform uimage2D dst;

layout (local_size_x = 16, local_size_y = 16) in;

void main() {
	ivec2 xy = ivec2(gl_GlobalInvocationID.xy);
	uvec3 v = imageLoad(dst, xy).rgb;
	v = v / 2;
	imageStore(dst, xy, uvec4(v, 0));
}