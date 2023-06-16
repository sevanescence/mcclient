use std::{net::{TcpStream, ToSocketAddrs}, io::{self, Write, Read, BufWriter, BufReader}};

use super::{packet::{clientbound::{status_response::StatusResponse, login_success::LoginSuccess, ping_response::PingResponse}, serialize_packet, serverbound::{handshake::{Handshake, NextState}, status_request::StatusRequest}, OutboundPacket, MCPacket, InboundPacket}, mctypes::VarInt, PROTOCOL_VERSION};

/// Describes a two-way TCP connection to a Minecraft server. The internal
/// buffer bytes are handled by a high-level serdes which encapsulates the
/// Minecraft packets. No byte manipulation is necessary to send packets
/// using a MinecraftStream.
pub struct MinecraftStream {
    writer: BufWriter<TcpStream>,
    reader: BufReader<TcpStream>,
}


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
        self.write(packet)?;
        self.flush()?;
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

/// Describes a connection to a Minecraft server. The stream is a `MinecraftStream` which handles
/// packet serdes, and the domain, port, and username are inferred when they are relevant. For instance,
/// the domain and port are known when the initial connection attempt is made, and the username will
/// be inferred once a user attempts to login.
/// # Example
/// ```
/// let mut connection = OfflineConnection::connect("localhost", 25565).expect("Could not connect");
/// connection.username(); // -> Returns `None`
/// let login_success = connection.login("Makoto").expect("Could not log in");
/// connection.username(); // -> Returns `Some` of String "Makoto"
/// ```
pub trait Connection: Sized {
    /// Attempts to connect to a Minecraft server. On success, the `Connection` is returned.
    /// # Errors
    /// This function will return an error if the connection cannot be established.
    fn connect<T: Into<String> + Clone>(domain: T, port: u16) -> Result<Self, io::Error>;
    /// Attempts to fetch a status report of the server.
    /// # Errors
    /// This function will return an error if the connection cannot be established. It can be
    /// inferred that failure to receive this packet means the connection cannot continue.
    fn status(&mut self) -> Result<StatusResponse, io::Error>;
    /// Attempts to ping the recipient server.
    /// # Errors
    /// This function will return an error if the ping fails. It can be inferred that failure
    /// to receive this packet means the connection cannot continue.
    fn ping(&mut self) -> Result<PingResponse, io::Error>;
    /// Attempts to log into the recipient server. The steps for this varies by connection type.
    /// For offline connections, a Login Request packet is followed immediately by a Login Success,
    /// while an online connection may require Mojang server authentication, and in newer versions,
    /// encryption authentication for Microsoft clients.
    /// # Errors
    /// This function will return an error if the login attempt fails. It can be inferred that
    /// failure to receive this packet means the connection cannot continue.
    fn login<T: Into<String> + Clone>(&mut self, username: T) -> Result<LoginSuccess, io::Error>;

    /// Gets the stream managed by this connection, which is used to send and receive packets.
    fn sock(&mut self) -> &mut MinecraftStream; 

    /// Gets the domain of the connection. This retrieves the domain passed to the initial connection
    /// attempt, not the endpoint IP resolved by the underlying TCP stream object.
    fn domain(&self) -> &str;
    /// Gets the port of the connection.
    fn port(&self) -> u16;
    /// Gets the username of the connection if it is set. This is set by a `login` invocation.
    fn username(&self) -> &Option<String>;
}

/// Represents a connection stream to an offline Minecraft server.
/// <br>
/// The handshake packet is sent when either a status or login request
/// is made. The stream itself attempts to open upon construction of
/// the object.
#[allow(unused)]
pub struct OfflineConnection {
    stream: MinecraftStream,
    domain: String,
    port: u16,
    username: Option<String>
}

#[allow(unused)]
impl Connection for OfflineConnection {
    fn connect<T: Into<String> + Clone>(domain: T, port: u16) -> Result<Self, io::Error> {
        let mut stream = MinecraftStream::connect(format!("{}:{}", domain.clone().into(), port))?;
        
        Ok(OfflineConnection { stream, domain: domain.into(), port, username: None })
    }

    fn status(&mut self) -> Result<StatusResponse, io::Error> {
        let handshake = Handshake {
            protocol_version: PROTOCOL_VERSION.into(),
            server_addr: self.domain.clone().into(), // TODO change to String type after MCType refactor
            port: self.port,
            next_state: NextState::STATUS
        };
        
        self.stream.send(&handshake)?;
        self.stream.send(&StatusRequest)?;

        let inbound = self.stream.read()?;
        if inbound.header.id.value() != 0x00 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Bad packet ID."));
        }
        let response = StatusResponse::from_data(&inbound)?;

        Ok(response)
    }

    fn ping(&mut self) -> Result<PingResponse, io::Error> {
        Ok(PingResponse {  })
    }

    fn login<T: Into<String> + Clone>(&mut self, username: T) -> Result<LoginSuccess, io::Error> {
        self.username = Some(username.into());
        Ok(LoginSuccess {  })
    }

    fn sock(&mut self) -> &mut MinecraftStream {
        &mut self.stream
    }

    fn domain(&self) -> &str {
        &self.domain
    }

    fn port(&self) -> u16 {
        self.port
    }

    fn username(&self) -> &Option<String> {
        &self.username
    }
}
