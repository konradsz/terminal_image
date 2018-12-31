extern crate image;
extern crate termsize;
#[macro_use]
extern crate clap;

use image::GenericImageView;
use clap::App;

fn find_nearest_matching_color(pixel: image::Rgba<u8>) -> image::Rgb<u8> {
    let (r, g, b) = ANSI_COLORS
        .iter()
        .skip(16)
        .min_by(|color_a, color_b| {
            ((i32::from(pixel.data[0]) - i32::from(color_a.0)).pow(2)
                + (i32::from(pixel.data[1]) - i32::from(color_a.1)).pow(2)
                + (i32::from(pixel.data[2]) - i32::from(color_a.2)).pow(2))
            .cmp(
                &((i32::from(pixel.data[0]) - i32::from(color_b.0)).pow(2)
                    + (i32::from(pixel.data[1]) - i32::from(color_b.1)).pow(2)
                    + (i32::from(pixel.data[2]) - i32::from(color_b.2)).pow(2)),
            )
        })
        .unwrap();

    image::Rgb { data: [*r, *g, *b] }
}

fn main() {
    let matches = App::new("terminal_image")
        .args_from_usage(
            "<input_file> 'An input image to display'
            -w, --width=[width] 'Sets width'
            -h, --height=[height] 'Sets height'
            -tc, --true-colour 'Uses 24-bit colour palette'"
        ).get_matches();

    let file_name = matches.value_of("input_file").unwrap();
    let width_arg = value_t!(matches, "width", u32);
    let height_arg = value_t!(matches, "height", u32);

    if matches.is_present("true-colour") {
        println!("TRUE COLOR");
    }

    let mut input: image::DynamicImage = image::open(file_name).unwrap();
    let (input_width, input_height) = input.dimensions();

    if width_arg.is_err() && height_arg.is_err() {
        let (mut width, mut height) = termsize::get().map(|size| (size.cols as u32, ((size.rows - 1) * 2) as u32)).unwrap();
        if input_width < width && input_height < height {
            width = input_width;
            height = input_height;
        }

        input = input.resize(width, height, image::FilterType::Nearest);
    } else if width_arg.is_ok() && height_arg.is_ok() {
        let (width, height) = (width_arg.unwrap(), height_arg.unwrap());

        input = input.resize_exact(width, height, image::FilterType::Nearest);
    } else if width_arg.is_ok() && height_arg.is_err() {
        let width = width_arg.unwrap();
        let coefficient = f64::from(input_width) / f64::from(width);
        let height = (f64::from(input_height) / coefficient) as u32;

        input = input.resize_exact(width, height, image::FilterType::Nearest);
    } else if width_arg.is_err() && height_arg.is_ok() {
        let height = height_arg.unwrap();
        let coefficient = f64::from(input_height) / f64::from(height);
        let width = (f64::from(input_width) / coefficient) as u32;

        input = input.resize_exact(width, height, image::FilterType::Nearest);
    }

    let (width, height) = input.dimensions();
    println!("Output size: ({}, {})", width, height);
    let mut output = image::ImageBuffer::new(width, height);
    output
        .enumerate_pixels_mut()
        .for_each(|(x, y, pixel)| {
            *pixel = find_nearest_matching_color(input.get_pixel(x, y));
        });


    // TRUE COLOR
    /*output
        .enumerate_pixels_mut()
        .for_each(|(x, y, pixel)| *pixel = input.get_pixel(x, y));*/

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

// https://jonasjacek.github.io/colors/
const ANSI_COLORS: [(u8, u8, u8); 256] = [
    (0, 0, 0),
    (128, 0, 0),
    (0, 128, 0),
    (128, 128, 0),
    (0, 0, 128),
    (128, 0, 128),
    (0, 128, 128),
    (192, 192, 192),
    (128, 128, 128),
    (255, 0, 0),
    (0, 255, 0),
    (255, 255, 0),
    (0, 0, 255),
    (255, 0, 255),
    (0, 255, 255),
    (255, 255, 255),
    (0, 0, 0),
    (0, 0, 95),
    (0, 0, 135),
    (0, 0, 175),
    (0, 0, 215),
    (0, 0, 255),
    (0, 95, 0),
    (0, 95, 95),
    (0, 95, 135),
    (0, 95, 175),
    (0, 95, 215),
    (0, 95, 255),
    (0, 135, 0),
    (0, 135, 95),
    (0, 135, 135),
    (0, 135, 175),
    (0, 135, 215),
    (0, 135, 255),
    (0, 175, 0),
    (0, 175, 95),
    (0, 175, 135),
    (0, 175, 175),
    (0, 175, 215),
    (0, 175, 255),
    (0, 215, 0),
    (0, 215, 95),
    (0, 215, 135),
    (0, 215, 175),
    (0, 215, 215),
    (0, 215, 255),
    (0, 255, 0),
    (0, 255, 95),
    (0, 255, 135),
    (0, 255, 175),
    (0, 255, 215),
    (0, 255, 255),
    (95, 0, 0),
    (95, 0, 95),
    (95, 0, 135),
    (95, 0, 175),
    (95, 0, 215),
    (95, 0, 255),
    (95, 95, 0),
    (95, 95, 95),
    (95, 95, 135),
    (95, 95, 175),
    (95, 95, 215),
    (95, 95, 255),
    (95, 135, 0),
    (95, 135, 95),
    (95, 135, 135),
    (95, 135, 175),
    (95, 135, 215),
    (95, 135, 255),
    (95, 175, 0),
    (95, 175, 95),
    (95, 175, 135),
    (95, 175, 175),
    (95, 175, 215),
    (95, 175, 255),
    (95, 215, 0),
    (95, 215, 95),
    (95, 215, 135),
    (95, 215, 175),
    (95, 215, 215),
    (95, 215, 255),
    (95, 255, 0),
    (95, 255, 95),
    (95, 255, 135),
    (95, 255, 175),
    (95, 255, 215),
    (95, 255, 255),
    (135, 0, 0),
    (135, 0, 95),
    (135, 0, 135),
    (135, 0, 175),
    (135, 0, 215),
    (135, 0, 255),
    (135, 95, 0),
    (135, 95, 95),
    (135, 95, 135),
    (135, 95, 175),
    (135, 95, 215),
    (135, 95, 255),
    (135, 135, 0),
    (135, 135, 95),
    (135, 135, 135),
    (135, 135, 175),
    (135, 135, 215),
    (135, 135, 255),
    (135, 175, 0),
    (135, 175, 95),
    (135, 175, 135),
    (135, 175, 175),
    (135, 175, 215),
    (135, 175, 255),
    (135, 215, 0),
    (135, 215, 95),
    (135, 215, 135),
    (135, 215, 175),
    (135, 215, 215),
    (135, 215, 255),
    (135, 255, 0),
    (135, 255, 95),
    (135, 255, 135),
    (135, 255, 175),
    (135, 255, 215),
    (135, 255, 255),
    (175, 0, 0),
    (175, 0, 95),
    (175, 0, 135),
    (175, 0, 175),
    (175, 0, 215),
    (175, 0, 255),
    (175, 95, 0),
    (175, 95, 95),
    (175, 95, 135),
    (175, 95, 175),
    (175, 95, 215),
    (175, 95, 255),
    (175, 135, 0),
    (175, 135, 95),
    (175, 135, 135),
    (175, 135, 175),
    (175, 135, 215),
    (175, 135, 255),
    (175, 175, 0),
    (175, 175, 95),
    (175, 175, 135),
    (175, 175, 175),
    (175, 175, 215),
    (175, 175, 255),
    (175, 215, 0),
    (175, 215, 95),
    (175, 215, 135),
    (175, 215, 175),
    (175, 215, 215),
    (175, 215, 255),
    (175, 255, 0),
    (175, 255, 95),
    (175, 255, 135),
    (175, 255, 175),
    (175, 255, 215),
    (175, 255, 255),
    (215, 0, 0),
    (215, 0, 95),
    (215, 0, 135),
    (215, 0, 175),
    (215, 0, 215),
    (215, 0, 255),
    (215, 95, 0),
    (215, 95, 95),
    (215, 95, 135),
    (215, 95, 175),
    (215, 95, 215),
    (215, 95, 255),
    (215, 135, 0),
    (215, 135, 95),
    (215, 135, 135),
    (215, 135, 175),
    (215, 135, 215),
    (215, 135, 255),
    (215, 175, 0),
    (215, 175, 95),
    (215, 175, 135),
    (215, 175, 175),
    (215, 175, 215),
    (215, 175, 255),
    (215, 215, 0),
    (215, 215, 95),
    (215, 215, 135),
    (215, 215, 175),
    (215, 215, 215),
    (215, 215, 255),
    (215, 255, 0),
    (215, 255, 95),
    (215, 255, 135),
    (215, 255, 175),
    (215, 255, 215),
    (215, 255, 255),
    (255, 0, 0),
    (255, 0, 95),
    (255, 0, 135),
    (255, 0, 175),
    (255, 0, 215),
    (255, 0, 255),
    (255, 95, 0),
    (255, 95, 95),
    (255, 95, 135),
    (255, 95, 175),
    (255, 95, 215),
    (255, 95, 255),
    (255, 135, 0),
    (255, 135, 95),
    (255, 135, 135),
    (255, 135, 175),
    (255, 135, 215),
    (255, 135, 255),
    (255, 175, 0),
    (255, 175, 95),
    (255, 175, 135),
    (255, 175, 175),
    (255, 175, 215),
    (255, 175, 255),
    (255, 215, 0),
    (255, 215, 95),
    (255, 215, 135),
    (255, 215, 175),
    (255, 215, 215),
    (255, 215, 255),
    (255, 255, 0),
    (255, 255, 95),
    (255, 255, 135),
    (255, 255, 175),
    (255, 255, 215),
    (255, 255, 255),
    (8, 8, 8),
    (18, 18, 18),
    (28, 28, 28),
    (38, 38, 38),
    (48, 48, 48),
    (58, 58, 58),
    (68, 68, 68),
    (78, 78, 78),
    (88, 88, 88),
    (98, 98, 98),
    (108, 108, 108),
    (118, 118, 118),
    (128, 128, 128),
    (138, 138, 138),
    (148, 148, 148),
    (158, 158, 158),
    (168, 168, 168),
    (178, 178, 178),
    (188, 188, 188),
    (198, 198, 198),
    (208, 208, 208),
    (218, 218, 218),
    (228, 228, 228),
    (238, 238, 238),
];
