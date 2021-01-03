#![warn(rust_2018_idioms, single_use_lifetimes)]

use std::{
    env, fs,
    // io::{self, BufWriter, Write},
    path::PathBuf,
};
mod xml;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn try_main() -> Result<()> {
    let mut args = env::args_os();
    let _ = args.next(); // executable name

    let filepath = match (args.next(), args.next()) {
        (Some(arg), None) => PathBuf::from(arg),
        _ => {
            println!("Usage: rust2xml path/to/filename.rs");
            return Ok(());
        }
    };

    let code = fs::read_to_string(&filepath)?;
    let syntax = syn::parse_file(&code)?;

    println!("{}", xml::to_string(&syntax));
    Ok(())
}
