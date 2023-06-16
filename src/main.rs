use mcclient::mc::connection::{OfflineConnection, Connection};

mod mc;
mod tests;

fn main() {
    const DOMAIN: &str = "localhost";
    const PORT: u16 = 25565;
    //const USERNAME: &str = "MonkeyDLuffy";

    println!("Connecting...");

    let mut connection = OfflineConnection::connect(DOMAIN, PORT)
        .expect("Could not connect.");
    
    println!("Connection successful. Requesting status...");

    let status_response = connection.status().expect("Could not get status.");
    // TODO: Refactor MCTypes as per mctypes repo.
    // TODO also: fucking refactor MCString as Into<String> or something
    println!("Response: {:?}", status_response.json_response);

    let _ping_response = connection.ping().expect("Could not ping.");
    println!("Ping response: {:#?}", _ping_response);
}
