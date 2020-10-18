/*
	Water fragment shader.

	Considers:
	  * reflection based on the water surface normal, showing:
	      - the "sky" environment map.
		  - bright phong reflection representing the sun.
	  * refraction when looking through the water
	  * a "floor" texture under the water
	  * a pre-calculated photon map + ambient light illuminate the floor.

*/
#version 450 core

in  vec2 frag_tex_coord;
out vec4 output_color;

layout(binding = 0) uniform sampler2D  normal; // water surface normals
layout(binding = 1) uniform sampler2D  sky;    // environment map
layout(binding = 2) uniform sampler2D  floor;  // floor texture
layout(binding = 3) uniform usampler2D photon; // photon map (see photon.glsl)

// refraction
uniform float water_refraction       = 1.33; // water index of refraction
uniform float water_refraction_depth = 0.2;  // water - floor distance

// reflection
uniform float reflection_height   = 2.0;  // water - sky distance
uniform float reflection_strength = 0.3;  // scales reflection intensity
uniform float sun_strength = 0.3;         // scales sun intensity

// photon mapping
uniform float ambient = 0.8;         // ambient light added to photon map.
uniform float photon_strength = 0.2; // scales photon map intensity
uniform vec3  light_dir = normalize(vec3(0.1, 0.2, 0.7));

// photon map normalization
// photon.glsl sends ~4 photons per pixel
// udecay.glsl decay causes an additional factor 2x
// So divide by 8 to be normalized.
#define PHOTON_NORM (8.0)     

void main() {

	// initial ray
	vec2 start = frag_tex_coord;
	vec3 dir   = vec3(0.0, 0.0, -1.0);

	vec3   n = texture(normal, start).xyz;

	// sky reflection
	vec3 refl_dir  = reflect(dir, n);
	float height   = reflection_height;
	vec2     hit   = start + height * refl_dir.xy;
	vec3 reflected = texture(sky, hit).rgb;

	// phong (sun)
	float v = dot(n, light_dir);
	float p = v * v;
	p = p * p;
	p = p * p;
	p = p * p;
	p = p * p;
	p = p * p;
	p = p * p;
	float sun = p * sun_strength;

	// refraction
	vec3 refr_dir   = refract(-dir, n, water_refraction);
	float     depth = water_refraction_depth;
	            hit = start + depth * refr_dir.xy;
	vec3 ph = texture(photon, hit).rgb * (photon_strength / PHOTON_NORM);
	vec3 refracted = texture(floor, hit).rgb * (ambient + ph);

	output_color = vec4(
		refracted +
		sun +
		reflection_strength * reflected ,
		1.0);
}