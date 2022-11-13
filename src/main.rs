use std::env;
use command_args::{parse_pair, parse_complex};
use helpers::{render, write_image};

pub mod command_args;
pub mod helpers;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]);
        eprintln!("Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.2", args[0]);
        std::process::exit(1);
    }

    let bounds= parse_pair(&args[2], 'x')
        .expect("error parsing image dimentions");
    let upper_left = parse_complex(&args[3])
        .expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[4])
        .expect("error parsing lower right corner point");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    render(&mut pixels, bounds, upper_left, lower_right);
    write_image(&args[1], &pixels, bounds)
        .expect("error writting PNG file");
}
