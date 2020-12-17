#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug)]
pub enum MessageReader {
    Empty,
    Amount(usize),
    Ready(Vec<u8>),
}

#[cfg(not(target_arch = "wasm32"))]
impl MessageReader {
    pub fn new() -> MessageReader {
        MessageReader::Empty
    }

    pub fn next(&mut self, mut stream: impl std::io::Read) -> Option<Vec<u8>> {
        let mut bytes = [0 as u8; 255];

        match self {
            MessageReader::Empty => {
                if let Ok(_) = stream.read_exact(&mut bytes[0..1]) {
                    *self = MessageReader::Amount(bytes[0] as usize);
                }
                None
            }
            MessageReader::Amount(len) => {
                if let Ok(_) = stream.read_exact(&mut bytes[0..*len]) {
                    *self = MessageReader::Ready(bytes[0..*len].to_vec());
                }
                None
            }
            MessageReader::Ready(_) => {
                let msg = std::mem::replace(self, MessageReader::Empty);
                match msg {
                    MessageReader::Ready(msg) => Some(msg),
                    _ => unreachable!(),
                }
            }
        }
    }
}

