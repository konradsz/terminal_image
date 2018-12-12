extern crate image;

use image::GenericImageView;

fn calculate_average_pixel(
    image: &image::DynamicImage,
    x: u32,
    y: u32,
    size: u32,
) -> image::Rgb<u8> {
    //println!("{}, {}", x, y);
    let (mut average_r, mut average_g, mut average_b) = (0, 0, 0);

    for offset_x in 0..size {
        for offset_y in 0..size {
            let pixel = image.get_pixel(x + offset_x, y + offset_y);
            average_r += pixel.data[0] as u32;
            average_g += pixel.data[1] as u32;
            average_b += pixel.data[2] as u32;
        }
    }

    average_r = (average_r / (size * size));
    average_g = (average_g / (size * size));
    average_b = (average_b / (size * size));

    image::Rgb {
        data: [average_r as u8, average_g as u8, average_b as u8],
    }
}

fn find_nearest_matching_color(pixel: image::Rgba<u8>) -> image::Rgb<u8> {
    /*let mut best_match = (0, 0, 0);
    let mut min_distance = std::i32::MAX;
    for color in ANSI_COLORS.iter().skip(16) {
        //println!("{}, {}, {}", color.0, color.1, color.2);
        let distance = (pixel.data[0] as i32 - color.0 as i32).pow(2)
            + (pixel.data[1] as i32 - color.1 as i32).pow(2)
            + (pixel.data[2] as i32 - color.2 as i32).pow(2);
        if distance < min_distance {
            min_distance = distance;
            best_match = *color;
        }
    }*/

    let (r, g, b) = ANSI_COLORS
        .iter()
        .skip(16)
        .min_by(|color_a, color_b| {
            ((pixel.data[0] as i32 - color_a.0 as i32).pow(2)
                + (pixel.data[1] as i32 - color_a.1 as i32).pow(2)
                + (pixel.data[2] as i32 - color_a.2 as i32).pow(2))
            .cmp(
                &((pixel.data[0] as i32 - color_b.0 as i32).pow(2)
                    + (pixel.data[1] as i32 - color_b.1 as i32).pow(2)
                    + (pixel.data[2] as i32 - color_b.2 as i32).pow(2)),
            )
        })
        .unwrap();

    image::Rgb {
        //data: [best_match.0, best_match.1, best_match.2],
        data: [*r, *g, *b],
    }
}

fn main() {
    let input: image::DynamicImage = image::open("lena.jpg").unwrap();
    //input = input.to_rgb();
    let (width, height) = input.dimensions();

    //find_nearest_matching_color(image::Rgb { data: [100, 100, 100] });

    //let pixel = img.get_pixel(0, 0);
    //println!("pixel {:?}", pixel);

    //const PIXEL_SIZE: u32 = 10;

    //let width = width / PIXEL_SIZE;
    //let height = height / PIXEL_SIZE;

    /*for x in 0..width {
        for y in 0..height {
            let a = calculate_average_color(&input, x * width, y * height, PIXEL_SIZE);
        }
    }*/


    let mut output = image::ImageBuffer::new(width, height);
    for (x, y, pixel) in output.enumerate_pixels_mut() { // for_each ?
        let nearest_pixel = find_nearest_matching_color(input.get_pixel(x, y));
        *pixel = nearest_pixel;
    }

    output.save("output.jpg").unwrap();

    //output.enumerate_pixels().zip(output.enumerate_pixels().skip(1)).for_each(|(p1, p2)| {
        //print!("\x1B[48;2;0;128;0m\x1B[38;2;255;0;0m\u{2584}");
    //    println!("p1: ({}, {}), p2: ({}, {})", p1.0, p1.1, p2.0, p2.1);
    //});


    // http://jafrog.com/2013/11/23/colors-in-terminal.html
    // https://stackoverflow.com/questions/4842424/list-of-ansi-color-escape-sequences
    /*print!("\x1B[48;2;0;128;0m\x1B[38;2;255;0;0m\u{2584}"); // U+2584 Lower Half Block character
    print!("\x1B[48;2;0;128;0m\x1B[38;2;255;0;0m\u{2584}"); // U+2584 Lower Half Block character
    println!("\x1B[m\n");
    println!("\x1B[48;5;255;0;0masd");*/
}

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
