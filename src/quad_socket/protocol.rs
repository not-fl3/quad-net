use std::io::ErrorKind;

#[derive(Debug)]
pub enum MessageReader {
    Empty,
    Amount(usize),
}

impl MessageReader {
    pub fn new() -> MessageReader {
        MessageReader::Empty
    }

    pub fn next(&mut self, mut stream: impl std::io::Read) -> Result<Option<Vec<u8>>, ()> {
        match self {
            MessageReader::Empty => {
                let mut size = [0u8; 4];
                match stream.read_exact(&mut size) {
                    Ok(_) => {
                        *self = MessageReader::Amount(u32::from_be_bytes(size) as usize);
                        Ok(None)
                    }
                    Err(err) if err.kind() == ErrorKind::WouldBlock => Ok(None),
                    Err(_err) => Err(()),
                }
            }
            MessageReader::Amount(len) => {
                let mut buf = vec![0u8; *len];
                match stream.read_exact(&mut buf) {
                    Ok(_) => {
                        *self = MessageReader::Empty;
                        Ok(Some(buf))
                    }
                    Err(err) if err.kind() == ErrorKind::WouldBlock => Ok(None),
                    Err(_) => Err(()),
                }
            }
        }
    }
}
