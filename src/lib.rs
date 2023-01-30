/*
* IDK WHAT I'M DOING 2: ELECTRIC BOOGALOO uwu
* ( Another crappy project by @Dex :D )
*/

mod consts;

mod classes;
use classes::*;

mod eval;

pub fn main(args: Vec<String>) -> IDFC<()> {
	if args.len() < 2 {
		return Ok(())
	}

	let mode = &args[1].to_lowercase();

	match mode.as_str() {
		"train"	=> {
			// do stuff
			// find out if the picture is a hot dog

			Ok(())
		},

		_	=> Ok(())
	}
}
