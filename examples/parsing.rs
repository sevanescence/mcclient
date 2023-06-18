use std::io;

use mcclient::mc::mctypes::{MCString, MCType};

fn main() -> Result<(), io::Error> {
    let s: MCString = "Makoto".into();
    let bytes = s.to_bytes();

    println!("{:?}", bytes);

    Ok(())
}
