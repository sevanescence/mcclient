use std::{net::{TcpStream, ToSocketAddrs}, io::{self, Write, Read, BufWriter, BufReader}};

use crate::mc::{packet::InboundPacket, PROTOCOL_VERSION};

use super::{packet::{clientbound::{status_response::StatusResponse, login_success::LoginSuccess, ping_response::PingResponse}, serialize_packet, serverbound::{handshake::{Handshake, NextState}, status_request::StatusRequest}, OutboundPacket, MCPacket}, mctypes::VarInt};

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

        Ok(MinecraftStream{ writer, reader })
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
        self.writer.write_all(&serialize_packet(packet))?;
        self.writer.flush()?;
        Ok(())
    }

    /// Flushes the outbound stream.
    /// # Errors
    /// An `io::Error` of any kind will be returned if the stream cannot be flushed, i.e.,
    /// the bytes cannot be sent to the target server.
    pub fn flush(&mut self) -> Result<(), io::Error> {
        self.writer.flush()
    }

    pub fn read(&mut self) -> Result<MCPacket, io::Error> {
        const MAX_HEADER_SIZE: usize = 6;
        let mut header_buf: [u8; MAX_HEADER_SIZE] = [0; MAX_HEADER_SIZE];
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

// type AnyStringType = dyn AsRef<str>;

pub trait Connection: Sized {
    fn connect<T: AsRef<str>>(domain: &T, port: u16, username: &T) -> Result<Self, io::Error>;
    fn status(&mut self) -> Result<StatusResponse, io::Error>;
    fn ping(&mut self) -> Result<PingResponse, io::Error>;
    fn login(&mut self) -> Result<LoginSuccess, io::Error>;
}

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
#[allow(unused)]
pub struct OfflineConnection {
    stream: TcpStream,
    username: String,
    domain: String,
    port: u16
}

#[allow(unused)]
impl Connection for OfflineConnection {
    fn connect<T: AsRef<str>>(domain: &T, port: u16, username: &T) -> Result<Self, io::Error> {
        Err(io::Error::new(io::ErrorKind::InvalidData, "Unimplemented."))
    }

    fn status(&mut self) -> Result<StatusResponse, io::Error> {
        Ok(StatusResponse{ json_response: "{\"message\":\"Unimplemented\"}".into() })
    }

    fn ping(&mut self) -> Result<PingResponse, io::Error> {
        Ok(PingResponse {  })
    }

    fn login(&mut self) -> Result<LoginSuccess, io::Error> {
        Ok(LoginSuccess {  })
    }
}
