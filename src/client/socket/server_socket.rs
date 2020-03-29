
pub struct ServerSocket {
    send_function: Box<dyn Fn(&str)>
}

impl ServerSocket {
    pub fn new(sender_func: impl Fn(&str) + 'static) -> ServerSocket {
        ServerSocket {
            send_function: Box::new(sender_func)
        }
    }

    pub fn send(&self, msg: &str) {
        (self.send_function)(msg);
    }
}