use command_args::parse_complex;
use helpers::pixel_to_point;

pub mod command_args;
pub mod helpers;

fn main() {
    let pair = parse_complex("10,20").unwrap();
    println!("pair: {:#?}", pair);
}
