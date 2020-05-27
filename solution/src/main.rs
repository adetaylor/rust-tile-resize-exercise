extern crate image;
extern crate clap;

use clap::{App, Arg}; 
use image::{ImageOutputFormat, ImageError, GenericImageView};
use image::io::Reader;
use std::fs::File;
use std::ffi::OsString;
use std::path::Path;
use std::io::{Read, Write, Seek, BufRead, BufReader};

#[derive(Debug)]
enum ConvertError {
	InputReadError(std::io::Error),
	CannotDecode(ImageError),
	CannotWrite(ImageError),
}

fn process_image<R, W>(input: R, mut output: W) -> Result<(), ConvertError> 
	where R: Read + Seek + BufRead, W: Write
{
	let r = Reader::new(input).with_guessed_format().map_err(|e| ConvertError::InputReadError(e))?;
	let mut image = r.decode().map_err(|e| ConvertError::CannotDecode(e))?;
	// Process image
	let topleft = image.get_pixel(0, 0);
	image.write_to(&mut output, ImageOutputFormat::Png).map_err(|e| ConvertError::CannotWrite(e))?;
	Ok(())
}

fn main() {
  let matches = App::new("tile-trimmer")
		.version("1.0")
		.about("Trims tiles")
		.author("Ade T.")
		.arg(Arg::with_name("in")
			.takes_value(true))
		.get_matches();  
	let infile = matches.value_of("in").unwrap();
	let infile = Path::new(infile);
	let stem = infile.file_stem().unwrap();
	let mut outname = String::new();
	outname.push_str(stem.to_str().unwrap());
	outname.push_str("-converted.png");
	let outfile = infile.with_file_name(outname);
	let infile = BufReader::new(File::open(infile).unwrap());
	let outfile = File::create(outfile).unwrap();
	match process_image(infile, outfile) {
		Err(x) => println!("{:?}", x),
		Ok(_) => {}
	};
}
