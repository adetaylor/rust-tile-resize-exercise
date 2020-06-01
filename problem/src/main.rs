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

const BLACK: image::Rgba<u8> = Rgba([0, 0, 0, 255]);

fn process_image<R, W>(input: R, mut output: W) -> Result<(), ConvertError> 
	where R: Read + Seek + BufRead, W: Write
{
	let r = Reader::new(input).with_guessed_format().map_err(|e| ConvertError::InputReadError(e))?;
	let image = r.decode().map_err(|e| ConvertError::CannotDecode(e))?;
	let (width, height) = image.dimensions();
	let mut lower_y = 0;
	let mut lower_x = 0;
	let mut lower_d = 0;
	for d in 0..width {
		if image.get_pixel(d, d) == BLACK {
			lower_d = d;
			break;
		}
	}
	for x in 0..=lower_d {
		if image.get_pixel(x, lower_d) == BLACK {
			lower_x = x;
			break;
		}
	}
	for y in 0..=lower_d {
		if image.get_pixel(lower_d, y) == BLACK {
			lower_y = y;
			break;
		}
	}
	let mindimension = std::cmp::min(width, height);
	let mut upper_d = 0;
	let mut upper_x = 0;
	let mut upper_y = 0;
	for d in 1..mindimension {
		if image.get_pixel(width-d, height-d) == BLACK {
			upper_d = d;
			break;
		}
	}
	for x in 1..=width {
		if image.get_pixel(width-x, height-upper_d) == BLACK {
			upper_x = width-x+1;
			break;
		}
	}
	for y in 1..=height {
		if image.get_pixel(width-upper_d, height-y) == BLACK {
			upper_y = height-y+1;
			break;
		}
	}
	let newimage = image.crop_imm(lower_x, lower_y, upper_x-lower_x, upper_y-lower_y);
	newimage.write_to(&mut output, ImageOutputFormat::Png).map_err(|e| ConvertError::CannotWrite(e))?;
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
