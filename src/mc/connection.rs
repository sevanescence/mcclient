use std::io;

use super::{
    packet::{
        clientbound::{
            login_success::LoginSuccess, ping_response::PingResponse,
            status_response::StatusResponse,
        },
        packet_ids::STATUS_RES_PACKET_ID,
        serverbound::{
            handshake::{Handshake, NextState},
            status_request::StatusRequest,
        },
        InboundPacket,
    },
    stream::MinecraftStream,
    PROTOCOL_VERSION,
};

// type AnyStringType = dyn AsRef<str>;

/// Describes a connection to a Minecraft server. The stream is a `MinecraftStream` which handles
/// packet serdes, and the domain, port, and username are inferred when they are relevant. For instance,
/// the domain and port are known when the initial connection attempt is made, and the username will
/// be inferred once a user attempts to login.
/// # Example
/// ```
/// use mcclient::mc::connection::{Connection, OfflineConnection};
/// 
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

    /// Gets the domain of the connection. 
    /// # Note
    /// This retrieves the domain passed to the initial connection
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
    username: Option<String>,
}

#[allow(unused)]
impl Connection for OfflineConnection {
    fn connect<T: Into<String> + Clone>(domain: T, port: u16) -> Result<Self, io::Error> {
        let mut stream = MinecraftStream::connect(format!("{}:{}", domain.clone().into(), port))?;

        Ok(OfflineConnection {
            stream,
            domain: domain.into(),
            port,
            username: None,
        })
    }

    fn status(&mut self) -> Result<StatusResponse, io::Error> {
        let handshake = Handshake {
            protocol_version: PROTOCOL_VERSION.into(),
            server_addr: self.domain.clone().into(), // TODO change to String type after MCType refactor
            port: self.port,
            next_state: NextState::STATUS,
        };

        self.stream.send(&handshake)?;
        self.stream.send(&StatusRequest)?;

        let inbound = self.stream.read()?;
        if inbound.header.id != STATUS_RES_PACKET_ID {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Bad packet ID."));
        }
        let response = StatusResponse::from_data(&inbound)?;

        Ok(response)
    }

    fn ping(&mut self) -> Result<PingResponse, io::Error> {
        Ok(PingResponse {})
    }

    fn login<T: Into<String> + Clone>(&mut self, username: T) -> Result<LoginSuccess, io::Error> {
        self.username = Some(username.into());
        Ok(LoginSuccess {})
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
