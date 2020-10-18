extern crate image;
use gl_obj::*;

pub fn load_image(fname: &str) -> Texture {
	let (size, data) = load_image_data(fname);
	Texture::new2d(gl::SRGB8_ALPHA8, size).sub_image2d(0, 0, 0, size.0, size.1, gl::RGBA, gl::UNSIGNED_BYTE, &data)
}

pub fn load_image_data(fname: &str) -> (uvec2, Vec<[u8; 4]>) {
	let src = image::io::Reader::open(fname).expect("open image").decode().expect("decode image").into_rgba();
	let size = uvec2(src.width(), src.height());
	let mut data = Vec::with_capacity((size.0 as usize) * (size.1 as usize));
	for c in src.pixels() {
		data.push([c[0], c[1], c[2], c[3]])
	}
	(size, data)
}
