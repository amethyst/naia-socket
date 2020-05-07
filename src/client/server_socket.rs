use std::net::SocketAddr;
use std::collections::VecDeque;

pub struct ServerSocket {
    pub address: SocketAddr,
    pub connected: bool,
    send_function: Box<dyn Fn(&str)>,
    send_queue: VecDeque<String>
}

impl ServerSocket {
    pub fn new(address: SocketAddr, sender_func: impl Fn(&str) + 'static) -> ServerSocket {
        ServerSocket {
            address,
            connected: false,
            send_function: Box::new(sender_func),
            send_queue: VecDeque::new()
        }
    }

    pub fn send(&self, msg: &str) {
        (self.send_function)(msg);
    }

    pub fn add_to_send_queue(&mut self, msg: &str) {
        self.send_queue.push_back(String::from(msg));
    }

    pub fn process_send_queue(&mut self) {
        if !self.connected {
            return;
        }
        while !self.send_queue.is_empty() {
            (self.send_function)(self.send_queue.pop_front().unwrap().as_ref());
        }
    }
}