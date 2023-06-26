use mcclient::mc::connection::{Connection, OfflineConnection};

mod mc;
mod tests;

// TODO: Implement mctype size functions (without having to make instances of the types!) and more packet builder functions
// TODO: Get rid of OutboundPacket::len()

// NOTE: Commit changes to OutboundPacket before rewriting serialize_packet

// TODO: Continue using (and reimplement) MC Types, but only use them internally
fn main() {
    const DOMAIN: &'static str = "localhost";
    const PORT: u16 = 25565;
    //const USERNAME: &str = "MonkeyDLuffy";

    println!("Connecting...");

    let mut connection = OfflineConnection::connect(DOMAIN, PORT).expect("Could not connect.");

    println!("Connection successful. Requesting status...");

    // TODO: Write BufferedPacketReader to read and consume packet bytes, rather than relying on side effects.
    //       The BufferedPackedReader is meant to replace the weird global functions that consume an array to
    //       read VarInt data and other stuff. The first calls to it are in the constructor of the MCPacketHeader.
    let status_response = connection.status().expect("Could not get status.");
    println!("Response: {:#?}", status_response.json_response);

    connection.reset();
    let _login_success = connection.login("Makoto").expect("Could not log in.");
    println!("{:#?}", "Login Success?");

    // TODO: Implement ping response and login success.
    // let _ping_response = connection.ping().expect("Could not ping.");
    // println!("Ping response: {:#?}", _ping_response);
}
