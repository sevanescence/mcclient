MCClient is a network client compliant with the Minecraft protocol.

In early development. Do not contribute, my ego is fragile.

NOTE: I have TODO's placed all around the code. I should really factor stuff.

# Changelog
### 0.1.1
- Refactored internal code.
- Slight changes to API:
  - MCType now implements from_bytes to explicitly error if invalid bytes are provided for a type.
  - Clientbound packets no longer use from_bytes, and instead use from_data.