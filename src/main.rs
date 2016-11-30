mod hexdump;

use std::env;
use std::path::Path;
use std::error::Error;

const HINTS: &'static str = "Usage: hexdump <file>";

fn main() {
    // parse args
    match env::args().len() {
        2 => {
            let filepath = match env::args().nth(1) {
                Some(val) => val,
                None => unreachable!(),
            };
            let filepath = Path::new(&filepath);
            if !filepath.is_file() {
                println!("Error: Not File");
                return;
            }
            if let Err(why) = hexdump::dump(filepath) {
                println!("Error: {}", why.description());
                return;
            };
        },
        _ => println!("{}", HINTS),
    }
}
