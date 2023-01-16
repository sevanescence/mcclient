use std::{net::TcpStream};

#[allow(dead_code)]
pub const PROTOCOL_VERSION: i32 = 760;

/// Represents a connection stream to an offline Minecraft server.
/// <br>
/// The handshake packet is sent when either a status or login request
/// is made. The stream itself attempts to open upon construction of
/// the object.
/// # Examples
/// ```
/// const DOMAIN: &str = "localhost";
/// const PORT: u16 = 25565;
/// const USERNAME: &str = "Dinnerbone";
/// let connection = OfflineConnection::connect(
///     DOMAIN.to_owned(), PORT, USERNAME.to_owned()
/// ).expect("Could not connect!");
/// ```
#[allow(dead_code)]
pub struct OfflineConnection {
    stream: TcpStream,
    username: String,
    domain: String,
    port: u16
}

#[allow(dead_code)]
impl OfflineConnection {
    /// Attempts to establish a TCP connection to a server, returning an `OfflineConnection`
    /// on success.
    pub fn connect(domain: String, port: u16, username: String) -> Result<Self, std::io::Error> {
        let ip = format!("{}:{}", domain, port);
        let stream = TcpStream::connect(ip)?;

        Ok(OfflineConnection{ stream, username, domain, port })
    }

    pub fn status(&mut self) {
        
    }

    pub fn login(&mut self) {
        const LOGIN_START_PACKET_ID: i32 = 0x00;
    }
}

