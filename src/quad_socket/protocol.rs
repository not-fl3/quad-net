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
        let mut bytes = [0 as u8; 255];

        match self {
            MessageReader::Empty => match stream.read_exact(&mut bytes[0..1]) {
                Ok(_) => {
                    *self = MessageReader::Amount(bytes[0] as usize);
                    Ok(None)
                }
                Err(err) if err.kind() == ErrorKind::WouldBlock => Ok(None),
                Err(_err) => Err(()),
            },
            MessageReader::Amount(len) => match stream.read_exact(&mut bytes[0..*len]) {
                Ok(_) => {
                    let msg = bytes[0..*len].to_vec();
                    *self = MessageReader::Empty;
                    Ok(Some(msg))
                }
                Err(err) if err.kind() == ErrorKind::WouldBlock => Ok(None),
                Err(_) => Err(()),
            },
        }
    }
}
