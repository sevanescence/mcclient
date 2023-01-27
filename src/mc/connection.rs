use std::{net::{TcpStream, ToSocketAddrs}, io::{self, Write, Read, BufWriter, BufReader}};

use crate::mc::packet::InboundPacket;

use super::{packet::{clientbound::status_response::StatusResponse, serialize_packet, serverbound::{handshake::{Handshake, NextState}, status_request::StatusRequest}, OutboundPacket, MCPacket}, mctypes::VarInt};

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
#[deprecated]
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

#[allow(dead_code)]
pub struct MinecraftStream {
    writer: BufWriter<TcpStream>,
    reader: BufReader<TcpStream>,
}

#[allow(dead_code)]
impl MinecraftStream {
    pub fn connect<T: ToSocketAddrs>(addr: T) -> Result<Self, io::Error> {
        let stream = TcpStream::connect(addr)?;
        
        let writer = BufWriter::new(stream.try_clone().unwrap());
        let reader = BufReader::new(stream);

        Ok(MinecraftStream { writer, reader })
    }

    /// Writes to the TCP outbound buffer. This should be used in tandem with
    /// `flush()` to send the outbound data to the target server. If you want
    /// to abstract this behavior, use `send(&mut self, packet: &dyn OutboundPacket)`.
    /// # Errors
    /// An `io::Error` of any kind will be returned if the packet cannot be sent.
    pub fn write(&mut self, packet: &dyn OutboundPacket) -> Result<(), io::Error> {
        Ok(self.writer.write_all(&serialize_packet(packet))?)
    }

    /// Writes to the TCP outbound buffer, and flushes the buffer.
    /// # Errors
    /// An `io::Error` of any kind will be returned if the packet cannot be sent or the
    /// stream cannot be flushed.
    pub fn send(&mut self, packet: &dyn OutboundPacket) -> Result<(), io::Error> {
        let res = self.writer.write_all(&serialize_packet(packet));
        self.writer.flush()?;
        Ok(res?)
    }

    /// Flushes the outbound stream.
    /// # Errors
    /// An `io::Error` of any kind will be returned if the stream cannot be flushes, i.e.,
    /// the bytes cannot be sent to the target server.
    pub fn flush(&mut self) -> Result<(), io::Error> {
        self.writer.flush()
    }

    pub fn read(&mut self) -> Result<MCPacket, io::Error> {
        let mut header_buf: [u8; 10] = [0; 10];
        let bytes_read = self.reader.read(&mut header_buf)?;

        let mut received: Vec<u8> = header_buf.to_vec();
        
        let len = VarInt::from_bytes(&received)?.value() as usize;
        let mut total_bytes_read = bytes_read;
        let mut buf: [u8; 256] = [0; 256];
        while total_bytes_read < len {
            let bytes_read = self.reader.read(&mut buf)?;
            received.extend_from_slice(&buf[..bytes_read]);
            total_bytes_read += bytes_read;
        };

        MCPacket::from_bytes(&mut received)
    }
}
