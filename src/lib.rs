/*
* IDK WHAT I'M DOING 2: ELECTRIC BOOGALOO uwu
* ( Another crappy project by @Dex :D )
*/

use image::{self, imageops::*};

mod classes;
use classes::*;

const INTERMEDIATE_LAYERS: u64 = 255;

pub fn main() -> IDFC<()> {
	// do stuff
	// find out if the picture is a hot dog

	Ok(())
}

fn amogus() {
	// stuff I copied from stackoverflow kek
	// (edit later)

	let img = image::open("cat.jpeg").unwrap();
	let mut img = img.grayscale();
	let mut img = img.as_mut_luma8().unwrap();
	dither(&mut img, &BiLevel);
	img.save("cat.png").unwrap(); // this step is optional but convenient for testing
}
