use std::{
    io::{self, BufReader, BufWriter, Read, Write},
    net::{TcpStream, ToSocketAddrs},
};

use crate::mc::mctypes::VarInt;

use super::packet::{ClientboundRawPacket, OutboundPacket, OutboundPacketBuffer};

/// Describes a two-way TCP connection to a Minecraft server. The internal
/// buffer bytes are handled by a high-level serdes which encapsulates the
/// Minecraft packets. No byte manipulation is necessary to send packets
/// using a MinecraftStream.
pub struct MinecraftStream {
    writer: BufWriter<TcpStream>,
    reader: BufReader<TcpStream>,
}

impl MinecraftStream {
    /// Connect to a remote Minecraft server. This function is not able to determine whether a
    /// server endpoint is MCProto-compliant without attempting to establish a status response.
    /// # Returns
    /// A stream to a Minecraft server is returned if the connection is successfully established.
    /// # Errors
    /// Any `io::Error` is returned if the connection cannot be established.
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
        let packet_buffer = OutboundPacketBuffer::from(packet);

        Ok(self.writer.write_all(packet_buffer.data())?)
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

    /// Attempts to consume a packet from the pending inbound byte stream.
    /// # Returns
    /// The corresponding packet data upon read success.
    /// # Errors
    /// This function will return an error if the packet could not be properly consumed.
    pub fn read(&mut self) -> Result<ClientboundRawPacket, io::Error> {
        const MAX_HEADER_BYTE_SIZE: usize = 6;
        let mut header_buf: [u8; MAX_HEADER_BYTE_SIZE] = [0; MAX_HEADER_BYTE_SIZE];
        let bytes_read = self.reader.read(&mut header_buf)?;

        let mut received: Vec<u8> = header_buf.to_vec();

        let len = TryInto::<i32>::try_into(VarInt::from_bytes(&received)?).unwrap() as usize;
        let mut total_bytes_read = bytes_read;
        let mut buf: [u8; 256] = [0; 256];
        while total_bytes_read < len {
            let bytes_read = self.reader.read(&mut buf)?;
            received.extend_from_slice(&buf[..bytes_read]);
            total_bytes_read += bytes_read;
        }

        ClientboundRawPacket::from_bytes(&mut received)
    }
}
