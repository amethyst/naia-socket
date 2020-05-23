
#[repr(u8)]
pub enum MessageHeader {
    ServerHandshake = 0,
    ClientHandshake = 1,
    Heartbeat = 2,
    Data = 3,
    Unknown = 255
}

impl From<u8> for MessageHeader {
    fn from(orig: u8) -> Self {
        match orig {
            0 => return MessageHeader::ServerHandshake,
            1 => return MessageHeader::ClientHandshake,
            2 => return MessageHeader::Heartbeat,
            3 => return MessageHeader::Data,
            _ => return MessageHeader::Unknown,
        };
    }
}

pub const DEFAULT_MTU: u16 = 1452;