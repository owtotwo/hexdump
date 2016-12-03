pub mod hexdump;

use std::env;
use std::path::Path;
use std::error::Error;

const HINTS: &'static str = "Usage: hexdump <file>";

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // parse args
    match args.len() {
        2 => {
            let filepath = Path::new(&args[1]);
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
