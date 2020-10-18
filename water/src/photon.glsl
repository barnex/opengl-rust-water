/*
	Photon mapping with dispersion.

	Inputs: 
	  * water's surface normals
	  * light direction
	  * water optical  parameters
	  * geometric parameters
	  * random seed

	Output is added to photon map (assumed to be cleared beforehand).
	    Unsigned RGB with ~4 "photons" per pixel on average.
	    (I.e.: divide the photon map by 4 afterwards to end up roughly normalized.)

	The random seed is supposed to change on each invocation,
	to avoid static noise patterns in the output.

*/
#version 450 core

layout (local_size_x = 16, local_size_y = 16) in;

layout(binding = 0)          uniform sampler2D normals; // water surface normals
layout(binding = 1, rgba8ui) uniform uimage2D  photons; // output added here

uniform vec3  light_dir = normalize(vec3(0.03, 0.01, -1.0));  // sign??
uniform float eta = 1.33;          // refractive index @ green
uniform float dispersion = 0.01;   // delta refractive index @ cyan
uniform float depth = 2.0;         // water - floor distance
uniform int   rand_seed = 0;       // to be changed at every invocation

// Colors represented as int,
// because atomicAdd only takes ints.
#define RGB(r, g, b) (((r)<<0) | ((g)<<8) | ((b)<<16))
#define RED    (RGB(2, 0, 0))
#define YELLOW (RGB(1, 1, 0))
#define GREEN  (RGB(0, 2, 0))
#define CYAN   (RGB(0, 1, 1))
#define BLUE   (RGB(0, 0, 2))
#define PURPLE (RGB(1, 0, 1))

// Add a "photon" to the photons texture,
// given a start position (on the water surface),
// and the water surface normal.
//
// Photons with varying colors (ranging from RED to PURPLE)
// can be mapped with slightly differnt refractive indices
// to achive dispersion (rainbow effect).
void map_photon(vec2 start, vec3 n, float eta, uint col){
	ivec2 size = imageSize(photons);
	vec3 refr = refract(light_dir, n, eta);
	vec2 hit = start + depth * refr.xy;
	ivec2 xy = ivec2(hit * size);
	imageAtomicAdd(photons, xy, col);
}

// Pseudo random number used to jitter the photon
// start position within it's pixel.
// Avoids patterns like dotted lines in the output.
float random (vec2 st) {
    return fract(sin(float(rand_seed)+dot(st.xy, vec2(12.9898,78.233)))* 43758.5453123);
}

void main() {
	ivec2 xy = ivec2(gl_GlobalInvocationID.xy);
	ivec2 size = imageSize(photons);

	vec2 pos = vec2(xy)/size;
	float r = random(pos);
	float s = random(pos+vec2(0.123, 0.456));

	vec2 start = pos + vec2(r, s) / size;
	vec3 n = texture(normals, start).xyz;

	map_photon(start, n, eta - 2.0*dispersion, RED);
	map_photon(start, n, eta - 1.0*dispersion, YELLOW);
	map_photon(start, n, eta - 0.0*dispersion, GREEN);
	map_photon(start, n, eta + 1.0*dispersion, CYAN);
	map_photon(start, n, eta + 2.0*dispersion, BLUE);
	map_photon(start, n, eta + 3.0*dispersion, PURPLE);
}
