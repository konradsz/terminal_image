extern crate image;
extern crate termsize;

mod ansi_colors;

use image::GenericImageView;

pub struct Args {
    file_name: String,
    width: Result<u32, clap::Error>,
    height: Result<u32, clap::Error>,
    true_colour: bool,
}

impl Args {
    pub fn new(
        file_name: &str,
        width: Result<u32, clap::Error>,
        height: Result<u32, clap::Error>,
        true_colour: bool,
    ) -> Args {
        Args {
            file_name: String::from(file_name),
            width,
            height,
            true_colour,
        }
    }
}

fn resize_image(
    mut image: image::DynamicImage,
    width: Result<u32, clap::Error>,
    height: Result<u32, clap::Error>,
) -> image::DynamicImage {
    let (original_width, original_height) = image.dimensions();

    if width.is_err() && height.is_err() {
        let (mut width, mut height) = termsize::get()
            .map(|size| (u32::from(size.cols), u32::from((size.rows - 1) * 2)))
            .unwrap();
        if original_width < width && original_height < height {
            width = original_width;
            height = original_height;
        }

        image = image.resize(width, height, image::FilterType::Nearest);
    } else if width.is_ok() && height.is_ok() {
        let (width, height) = (width.unwrap(), height.unwrap());

        image = image.resize_exact(width, height, image::FilterType::Nearest);
    } else if width.is_ok() && height.is_err() {
        let width = width.unwrap();
        let coefficient = f64::from(original_width) / f64::from(width);
        let height = (f64::from(original_height) / coefficient) as u32;

        image = image.resize_exact(width, height, image::FilterType::Nearest);
    } else if width.is_err() && height.is_ok() {
        let height = height.unwrap();
        let coefficient = f64::from(original_height) / f64::from(height);
        let width = (f64::from(original_width) / coefficient) as u32;

        image = image.resize_exact(width, height, image::FilterType::Nearest);
    }
    image
}

fn create_output_image(
    input: &image::DynamicImage,
    true_colour: bool,
) -> image::ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>> {
    let (width, height) = input.dimensions();
    println!("Output size: ({}, {})", width, height);

    let mut output = image::ImageBuffer::new(width, height);
    if true_colour {
        output
            .enumerate_pixels_mut()
            .for_each(|(x, y, pixel)| *pixel = input.get_pixel(x, y));
    } else {
        output.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
            *pixel = find_nearest_matching_color(input.get_pixel(x, y));
        });
    }

    output
}

fn find_nearest_matching_color(pixel: image::Rgba<u8>) -> image::Rgba<u8> {
    let distance = |pixel: image::Rgba<u8>, color: (u8, u8, u8)| {
        (i32::from(pixel.data[0]) - i32::from(color.0)).pow(2)
            + (i32::from(pixel.data[1]) - i32::from(color.1)).pow(2)
            + (i32::from(pixel.data[2]) - i32::from(color.2)).pow(2)
    };

    let (r, g, b) = ansi_colors::COLORS
        .iter()
        .skip(16)
        .min_by(|&color_a, &color_b| distance(pixel, *color_a).cmp(&distance(pixel, *color_b)))
        .unwrap();

    image::Rgba {
        data: [*r, *g, *b, u8::max_value()],
    }
}

fn display_image(output: &image::ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>>) {
    let width = output.dimensions().0;
    // U+2584 Lower Half Block with background gives 2 pixels per one character in terminal
    output
        .enumerate_pixels()
        .filter(|(_, y, _)| y % 2 == 0)
        .zip(output.enumerate_pixels().filter(|(_, y, _)| y % 2 == 1))
        .for_each(|((x, _, top_pixel), (_, _, bottom_pixel))| {
            print!(
                "\x1B[48;2;{};{};{}m\x1B[38;2;{};{};{}m\u{2584}",
                top_pixel.data[0],
                top_pixel.data[1],
                top_pixel.data[2],
                bottom_pixel.data[0],
                bottom_pixel.data[1],
                bottom_pixel.data[2]
            );
            if x == width - 1 {
                println!("\x1B[m");
            }
        });
}

pub fn run(config: Args) {
    let input = resize_image(
        image::open(&config.file_name).unwrap(),
        config.width,
        config.height,
    );
    let output = create_output_image(&input, config.true_colour);

    display_image(&output);
}
