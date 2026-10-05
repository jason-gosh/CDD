mod search;
mod shared;

use std::env;
use std::process;

use crate::shared::hex_console_colors;

fn main() {
    let mut args = env::args();

    args.next();

    let Some(prefix) = args.next() else {
        eprintln!("use: cdf <prefijo>");
        process::exit(1);
    };

    match search::find_directory(&prefix) {
        Some(directories) => {
            for dir in directories{
                println!("{} {}/ {}", hex_console_colors::BLUE, dir, hex_console_colors::RESET);
            }
        }

        None => {
            process::exit(1);
        }

    }
}