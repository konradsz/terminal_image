#[macro_use]
extern crate clap;
use clap::App;

use terminal_image;
use terminal_image::Args;

fn main() {
    let matches = App::new("terminal_image")
        .args_from_usage(
            "<input_file> 'An input image to display'
            -w, --width=[width] 'Sets width'
            -h, --height=[height] 'Sets height'
            --true-color 'Uses 24-bit color palette'",
        )
        .get_matches();

    let file_name = matches.value_of("input_file").unwrap();
    let width = value_t!(matches, "width", u32);
    let height = value_t!(matches, "height", u32);
    let true_color = matches.is_present("true-color");

    let args = Args::new(file_name, width, height, true_color);
    terminal_image::run(args);
}
