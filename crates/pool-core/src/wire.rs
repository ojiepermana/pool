//! Reading whole messages from a stream with exact reads.

use pool_protocol::message::{FrameError, Message, MessageReader};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

/// Why a message could not be read or written.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum WireError {
    /// The peer closed the stream, or reading or writing failed.
    Closed,
    /// The message broke the framing or was above the limit.
    Frame,
}

/// Reads one whole message whose body is at most `limit` bytes.
///
/// It reads exactly what the message needs, so no byte of the next message is consumed.
pub(crate) async fn read_message<S: AsyncRead + Unpin>(
    stream: &mut S,
    limit: u32,
) -> Result<Message, WireError> {
    let mut reader = MessageReader::new(limit);
    let mut buffer = [0u8; 4096];
    loop {
        let want = reader.wanted().clamp(1, buffer.len());
        stream
            .read_exact(&mut buffer[..want])
            .await
            .map_err(|_| WireError::Closed)?;
        match reader.feed(&buffer[..want]) {
            (_, None) => {}
            (_, Some(Ok(message))) => return Ok(message),
            (_, Some(Err(FrameError::Malformed | FrameError::TooLarge))) => {
                return Err(WireError::Frame);
            }
        }
    }
}

/// Writes all bytes and flushes.
pub(crate) async fn write_all<S: AsyncWrite + Unpin>(
    stream: &mut S,
    bytes: &[u8],
) -> Result<(), WireError> {
    stream
        .write_all(bytes)
        .await
        .map_err(|_| WireError::Closed)?;
    stream.flush().await.map_err(|_| WireError::Closed)
}
