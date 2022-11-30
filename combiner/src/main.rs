mod args;
use args::Args;
use image::GenericImageView;
use image::{imageops::FilterType::Triangle, io::Reader, DynamicImage, ImageFormat};
use std::fs::File;
use std::io::BufReader;

fn main() -> Result<(), ImageDataErrors> {
    let args = Args::new();
    println!("{:?}", args);

    // Open the image from path
    let (image_1, image_1_format) = find_image_from_path(args.image_1);
    let (image_2, image_2_format) = find_image_from_path(args.image_2);

    // If images have different format, return an error
    if image_1_format != image_2_format {
        return Err(ImageDataErrors::DifferentImageFormats);
    }

    // Get the smaller of the two and resize images two the smaller dimensions
    let (image_1, image_2) = standardise_size(image_1, image_2);

    // Create a FloatingImage object as placeholder for the output
    let mut output = FloatingImage::new(image_1.width(), image_1.height(), args.output);

    // Get RGBA values of both images and alternate the pixels
    let combined_data = combine_images(image_1, image_2);

    // Attach combined_data RGBA values to FloatingImage
    output.set_data(combined_data)?; // ? is shorthand way of handling the result of a function call. If it's an error, it returns the error, otherwise, the result

    // Write image to file
    image::save_buffer_with_format(
        output.name,
        &output.data,
        output.width,
        output.height,
        image::ColorType::Rgba8,
        image_1_format,
    )
    .unwrap();
    Ok(())
}

#[derive(Debug)]
enum ImageDataErrors {
    DifferentImageFormats,
    BufferTooSmall,
}

struct FloatingImage {
    width: u32,
    height: u32,
    data: Vec<u8>, // RGBA values 0 - 255
    name: String,
}

impl FloatingImage {
    fn new(width: u32, height: u32, name: String) -> Self {
        let buffer_capacity = 3_655_744; // 956 x 956 x 4
        let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
        FloatingImage {
            width,
            height,
            data: buffer,
            name,
        }
    }

    fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors> {
        // If previously assigned buffer is too small to hold the new data
        if data.len() > self.data.capacity() {
            return Err(ImageDataErrors::BufferTooSmall);
        }
        self.data = data;
        Ok(())
    }
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
    let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap();
    let image_format: ImageFormat = image_reader.format().unwrap();
    let image: DynamicImage = image_reader.decode().unwrap();
    (image, image_format)
}

fn get_smaller_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
    let pix_1 = dim_1.0 * dim_1.1;
    let pix_2 = dim_2.0 * dim_2.1;
    if pix_1 < pix_2 {
        dim_1
    } else {
        dim_2
    }
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
    let (width, height) = get_smaller_dimensions(image_1.dimensions(), image_2.dimensions());
    println!("width: {width}, heigh: {height}");

    if image_2.dimensions() == (width, height) {
        (image_1.resize_exact(width, height, Triangle), image_2)
    } else {
        (image_1, image_2.resize_exact(width, height, Triangle))
    }
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
    // A Vec<u8> is created with the same length as vec_1
    let mut combined_data = vec![0u8; vec_1.len()];

    let mut i = 0;
    while i < vec_1.len() {
        if i % 8 == 0 {
            combined_data.splice(i..=i + 3, set_rgba(&vec_1, i, i + 3));
        } else {
            combined_data.splice(i..=i + 3, set_rgba(&vec_2, i, i + 3));
        }
        i += 4;
    }

    combined_data
}

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
    let vec_1 = image_1.to_rgba8().into_vec();
    let vec_2 = image_2.to_rgba8().into_vec();

    alternate_pixels(vec_1, vec_2)
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
    let mut rgba = Vec::new();
    for i in start..=end {
        // ..= is Rust's range syntax with inclusive end value
        let val = match vec.get(i) {
            Some(d) => *d, // Dereferencing operator, access the value of the variable
            None => panic!("Index out of bounds"),
        };
        rgba.push(val);
    }
    rgba
}
