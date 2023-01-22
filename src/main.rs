mod mc;
mod tests;

use mc::connection::OfflineConnection;

fn main() {
    const DOMAIN: &str = "localhost";
    const PORT: u16 = 25565;
    const USERNAME: &str = "MonkeyDLuffy";

    println!("Connecting...");
    
    let _connection = match OfflineConnection::connect(
        DOMAIN.to_owned(), PORT, USERNAME.to_owned()) {
        Ok(conn) => conn,
        Err(msg) => panic!("{}", msg)
    };
    
    println!("Connection successful. Requesting status...");
}
