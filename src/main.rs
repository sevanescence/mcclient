use crate::mc::connection::OfflineConnection;

mod mc;
mod tests;

fn main() {
    // Serverbound packet structure
    // <length: VarInt> <packet ID: VarInt> <data>
    // Handshake packet structure
    // <protover: VarInt> <addr: String(255)> <port: ushort> <next_state: VarInt Enum>

    // Hardcoded constants for now
    const DOMAIN: &str = "localhost";
    const PORT: u16 = 25565;
    const USERNAME: &str = "MakotoII";

    println!("Connecting...");
    
    let _connection = match OfflineConnection::connect(
        DOMAIN.to_owned(), PORT, USERNAME.to_owned()) {
        Ok(conn) => conn,
        Err(msg) => panic!("{}", msg)
    };
    
    println!("Connection successful. Requesting status...");
}
