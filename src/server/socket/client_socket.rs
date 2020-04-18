use std::net::IpAddr;

pub struct ClientSocket {
    pub ip: IpAddr,
    send_function: Box<dyn Fn(&str)>
}

impl ClientSocket {
    pub fn new(ip: IpAddr, sender_func: impl Fn(&str) + 'static) -> ClientSocket {
        ClientSocket {
            ip,
            send_function: Box::new(sender_func)
        }
    }

    pub fn send(&self, msg: &str) {
        (self.send_function)(msg);
    }
}