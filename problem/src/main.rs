extern crate image;
extern crate clap;

use clap::{App, Arg}; 
use image::{Rgba, ImageOutputFormat, ImageError, GenericImageView};
use image::io::Reader;
use std::fs::File;
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
	let image = r.decode().map_err(|e| ConvertError::CannotDecode(e))?;

	// How to get the size of an image...
	//  let (width, height) = image.dimensions();
	// How to inspect a pixel:
	//  let p = image.get_pixel(x, y)
	// How to loop through some pixels...
	//  for d in 0..width { ... do stuff ... } (also, 'break' can break out of a loop)
	// How to see if a pixel is a particular colour:
	//  (put the next line above 'fn process_image')
	//  const BLACK: image::Rgba<u8> = Rgba([0, 0, 0, 255]);  (those four numbers are red, green, blue, and opacity)
	//  if p == BLACK { ... }
	// How to create a new cropped image:
	//  let newimage = image.crop_imm(lower_x, lower_y, upper_x-lower_x, upper_y-lower_y);
	//  newimage.write_to(&mut output, ImageOutputFormat::Png).map_err(|e| ConvertError::CannotWrite(e))?;
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

#[cfg(test)]
mod tests {
	use process_image;

	#[test]
	fn test_tile1() {
		let infile: &[u8] = include_bytes!("../../tiles/tile.png");
		let outfile: &[u8] = include_bytes!("../../tiles/tile-converted.png");
		let mut processedout = std::io::Cursor::new(Vec::new());
		process_image(std::io::Cursor::new(infile), &mut processedout).unwrap();
		assert_eq!(outfile.to_vec(), processedout.into_inner());
	}

	#[test]
	fn test_tile2() {
		let infile: &[u8] = include_bytes!("../../tiles/tile2.png");
		let outfile: &[u8] = include_bytes!("../../tiles/tile2-converted.png");
		let mut processedout = std::io::Cursor::new(Vec::new());
		process_image(std::io::Cursor::new(infile), &mut processedout).unwrap();
		assert_eq!(outfile.to_vec(), processedout.into_inner());
	}

	#[test]
	fn test_tile3() {
		let infile: &[u8] = include_bytes!("../../tiles/tile3.png");
		let outfile: &[u8] = include_bytes!("../../tiles/tile3-converted.png");
		let mut processedout = std::io::Cursor::new(Vec::new());
		process_image(std::io::Cursor::new(infile), &mut processedout).unwrap();
		assert_eq!(outfile.to_vec(), processedout.into_inner());
	}
}
