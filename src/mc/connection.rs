use std::{net::TcpStream, io::{self, Write, Read}};

use crate::mc::packet::InboundPacket;

use super::packet::{clientbound::status_response::StatusResponse, serialize_packet, serverbound::{handshake::{Handshake, NextState}, status_request::StatusRequest}};

#[allow(dead_code)]
pub const PROTOCOL_VERSION: i32 = 760;

/// Represents a connection stream to an offline Minecraft server.
/// <br>
/// The handshake packet is sent when either a status or login request
/// is made. The stream itself attempts to open upon construction of
/// the object.
/// # Examples
/// ```
/// use mcclient::mc::connection::OfflineConnection;
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
    pub fn connect(domain: String, port: u16, username: String) -> Result<Self, io::Error> {
        let ip = format!("{}:{}", domain, port);
        let stream = TcpStream::connect(ip)?;

        Ok(OfflineConnection{ stream, username, domain, port })
    }

    pub fn status(&mut self) -> Result<StatusResponse, io::Error> {
        self.stream.write_all(&serialize_packet(&Handshake {
            protocol_version: PROTOCOL_VERSION.into(),
            server_addr: self.domain.clone().into(),
            port: self.port,
            next_state: NextState::STATUS
        }))?;
        self.stream.write_all(&serialize_packet(&StatusRequest))?;
        self.stream.flush()?;

        let mut received: Vec<u8> = vec![];

        const RX_BUFSIZE: usize = 256;
        let mut rx_bytes: [u8; RX_BUFSIZE] = [0; RX_BUFSIZE];
        loop {
            let bytes_read = self.stream.read(&mut rx_bytes)?;

            received.extend_from_slice(&rx_bytes[..bytes_read]);

            if bytes_read < RX_BUFSIZE {
                break;
            }
        }

        Ok(StatusResponse::from_bytes(&received)?)
    }

    pub fn login(&mut self) {
        const LOGIN_START_PACKET_ID: i32 = 0x00;
    }

    /// Retrieves a copy of the username used to connect to an offline server.
    pub fn username(&self) -> String {
        self.username.clone()
    }
}
