use crate::mc::mctypes::VarInt;

mod mc;
mod tests;


fn main() {
    println!("{:?}", VarInt::from(2147483647).as_bytes());

    // Serverbound packet structure
    // <length: VarInt> <packet ID: VarInt> <data>
    // Handshake packet structure
    // <protover: VarInt> <addr: String(255)> <port: ushort> <next_state: VarInt Enum>

    // Hardcoded constants for now
    // const DOMAIN: &str = "okbuddyholotard.serveminecraft.net";
    // const PORT: u16 = 8081;
    // const USERNAME: &str = "MakotoII";

    // println!("Connecting to server {}:{}", DOMAIN, PORT);

    // let mut connection = OfflineConnection::new(
    //     DOMAIN.to_string(), PORT, USERNAME.to_string()
    // );

    // println!("Connection established. Requesting status...");
    // let status = connection.status();

    // println!("Status: {}", status.json);

    // println!("Logging in...");+
    // connection.login();
}
