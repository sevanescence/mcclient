MCClient is a network client compliant with the Minecraft protocol.

**The currently supported version is 1.19.3.**
**I have no plans to change this until I decide the best course of action.**
**For the time being, this is primarily just a client program, but I would like**
**for it, at some point, to be a fully-fledged MC Protocol library.**

In early development. Do not contribute, my ego is fragile.

NOTE: I have TODO's placed all around the code. I should really factor stuff.

# Changelog
### 0.1.1
- Refactored internal code.
- Slight changes to API:
  - MCType now implements from_bytes to explicitly error if invalid bytes are provided for a type.
  - Clientbound packets no longer use from_bytes, and instead use from_data.
### 0.1.2
- Refactored internal code.
- Added not-yet-entirely-implemented packet builder to make serialization simpler.
  - Migrating packet member types to standard data types, abstracting away from their
    protocol serialization.
- **NOTE: The OutboundPacket function len() is weirdly implemented, and should honestly be abstracted to not be so confusing.**
  **This needs to be amended before any further development.**
### 0.1.3
- Removed OutboundPacket::len(), this is all taken care of in the `serialize_packet`.