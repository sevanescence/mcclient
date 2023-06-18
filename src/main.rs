use mcclient::mc::connection::{Connection, OfflineConnection};

mod mc;
mod tests;

// TODO: Implement mctype size functions (without having to make instances of the types!) and more packet builder functions
// TODO: Get rid of OutboundPacket::len()

// NOTE: Commit changes to OutboundPacket before rewriting serialize_packet

fn main() {
    const DOMAIN: &str = "localhost";
    const PORT: u16 = 25565;
    //const USERNAME: &str = "MonkeyDLuffy";

    println!("Connecting...");

    let mut connection = OfflineConnection::connect(DOMAIN, PORT).expect("Could not connect.");

    println!("Connection successful. Requesting status...");

    // TODO: Write BufferedPacketReader to read and consume packet bytes, rather than relying on side effects.
    let status_response = connection.status().expect("Could not get status.");
    println!("Response: {:?}", status_response.json_response);

    // TODO: Implement ping response and login success.
    let _ping_response = connection.ping().expect("Could not ping.");
    println!("Ping response: {:#?}", _ping_response);
}
