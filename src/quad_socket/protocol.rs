use std::io::ErrorKind;

#[derive(Debug)]
pub(crate) struct MessageReader {
    buffer: Vec<u8>,
}

impl MessageReader {
    pub fn new() -> MessageReader {
        MessageReader {
            buffer: Vec::new()
        }
    }

    pub fn next(&mut self, mut stream: impl std::io::Read) -> Result<Option<Vec<u8>>, ()> {
        let mut bytes = [0; 16 * 1024];

        let bytes_read = match stream.read(&mut bytes) {
            Ok(bytes_read) => bytes_read,
            Err(err) if err.kind() == ErrorKind::WouldBlock => return Ok(None),
            Err(_err) => return Err(()),
        };

        if bytes_read == 0 {
            // Disconnected
            return Err(());
        }

        // Read the first 4 bytes, which encode the message's length
        self.buffer.extend_from_slice(&bytes[..bytes_read]);

        if self.buffer.len() < 4 {
            return Ok(None);
        }

        use std::convert::TryInto;
        let four_bytes: [u8; 4] = self.buffer[0..4].try_into().unwrap();
        let message_size = u32::from_be_bytes(four_bytes) as usize;

        // Keep receiving until the whole message is here
        if self.buffer.len() < 4 + message_size {
            return Ok(None);
        }

        let message = self.buffer[4..4+message_size].to_vec();
        self.buffer.drain(..4+message_size);

        Ok(Some(message))
    }
}
