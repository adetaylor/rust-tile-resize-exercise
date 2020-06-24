extern crate image;
extern crate clap;

use clap::{App, Arg}; 
use image::{Rgba, ImageOutputFormat, ImageError, GenericImageView, DynamicImage};
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

const SEARCH_OFFSET: u32 = 60u32;
const BLACK: image::Rgba<u8> = Rgba([0, 0, 0, 255]);
const WHITE: image::Rgba<u8> = Rgba([255, 255, 255, 255]);

fn create_tile(image: &DynamicImage, tile_counter: &mut u32, x: u32, y: u32, w: u32, h: u32, outdir: &Path) {
	let s = format!("{}.png", tile_counter);
	*tile_counter = *tile_counter + 1;
	let outpath = outdir.join(s);
	println!("Tile at {},{},{},{} -> {:?}", x, y, w, h, &outpath);
	let mut outfile = File::create(outpath).unwrap();
	let newimage = image.crop_imm(x, y, w, h);
	newimage.write_to(&mut outfile, ImageOutputFormat::Png).unwrap();
}

fn extract_tile(image: &DynamicImage, tile_counter: &mut u32, startx: u32, starty: u32, outdir: &Path) -> (u32, u32) {
	let (width, height) = image.dimensions();
	let mut tilewidth = None;
	for x in startx..width {
		let p = image.get_pixel(x, starty);
		if p == WHITE {
			tilewidth = Some(x - startx);
			break;
		}
	}

	let mut tileheight = None;
	for y in starty..height {
		let p = image.get_pixel(startx, y);
		if p == WHITE {
			tileheight = Some(y - starty);
			break;
		}
	}

	create_tile(image, tile_counter, startx, starty, tilewidth.unwrap(), tileheight.unwrap(), outdir);
	(tilewidth.unwrap(), tileheight.unwrap())
}

fn extract_tiles_from_row(image: &DynamicImage, tile_counter: &mut u32, starty: u32, outdir: &Path) -> u32 {
    let (width, height) = image.dimensions();
	let search_row = starty + SEARCH_OFFSET;
	let mut tileheight = None;
	let mut x = 0;
	while x < width {
		let p = image.get_pixel(x, starty);
		if p == BLACK {
			let (tw, th) = extract_tile(&image, tile_counter, x, starty, outdir);
			tileheight = Some(th);
			x = x + tw;
		} else {
			x = x + 1;
		}
	}
	tileheight.unwrap()
}

fn process_image<R>(input: R, outdir: &Path) -> Result<(), ConvertError> 
	where R: Read + Seek + BufRead
{
	let r = Reader::new(input).with_guessed_format().map_err(|e| ConvertError::InputReadError(e))?;
	let image = r.decode().map_err(|e| ConvertError::CannotDecode(e))?;
    let (_, height) = image.dimensions();

	let mut tile_counter = 1u32;

	let mut y = 0;
	while y < height {
		let p = image.get_pixel(SEARCH_OFFSET, y);
		if p == BLACK {
			let tileheight = extract_tiles_from_row(&image, &mut tile_counter, y, outdir);
			y = y + tileheight;
		} else {
			y = y + 1;
		}
	}

	// How to get the size of an image...
	// How to inspect a pixel:
	//  let p = image.get_pixel(x, y)
	// How to loop through some pixels...
	//  for d in 0..width { ... do stuff ... } (also, 'break' can break out of a loop)
	// How to see if a pixel is a particular colour:
	//  (put the next line above 'fn process_image')
	//  const BLACK: image::Rgba<u8> = Rgba([0, 0, 0, 255]);  (those four numbers are red, green, blue, and opacity)
	//  if p == BLACK { ... }
	// How to create a new cropped image:
	//  
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
	let inpath = matches.value_of("in").unwrap();
	let inpath = Path::new(inpath);
	let outdir = inpath.parent().unwrap().join("output");
	let infile = BufReader::new(File::open(inpath).unwrap());
	match process_image(infile, outdir.as_path()) {
		Err(x) => println!("{:?}", x),
		Ok(_) => {}
	};
}
