extern crate mcclient;

use std::io;

use mcclient::mc::connection::{OfflineConnection, MinecraftStream};

fn main() -> Result<(), io::Error> {
    const DOMAIN: &str = "localhost";
    const PORT: u16 = 25565;
    const USERNAME: &str = "MonkeyDLuffy";

    println!("Connecting...");
    
    let connection = MinecraftStream::connect(format!("{}:{}", DOMAIN, PORT))?;
    
    println!("Connection successful. Requesting status...");

    Ok(())
}
