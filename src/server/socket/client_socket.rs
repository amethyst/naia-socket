use std::net::IpAddr;

pub struct ClientSocket {
    pub ip: IpAddr
}

impl ClientSocket {
    fn new(ip: IpAddr) -> ClientSocket {
        ClientSocket {
            ip
        }
    }
}