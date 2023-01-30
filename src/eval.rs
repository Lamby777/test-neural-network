/*
* This is pretty much the "brain" of the program.
* Training will run this + modified versions and pick the best.
*/

use image::{self, imageops::*};
use crate::consts::*;

fn train_img() {
	// stuff I copied from stackoverflow kek
	// (edit later)

	let img = image::open("cat.jpeg").unwrap();
	let mut img = img.grayscale();
	let mut img = img.as_mut_luma8().unwrap();
	dither(&mut img, &BiLevel);

	resize(
		img,
		INPUT_IMAGE_SIZE.0,
		INPUT_IMAGE_SIZE.1,
		FilterType::CatmullRom
	);
}
