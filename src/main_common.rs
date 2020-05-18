
extern crate log;

use gaia_client_socket::{ClientSocket, ClientSocketImpl, SocketEvent, MessageSender};

#[cfg(not(target_arch = "wasm32"))]
use gaia_socket_shared::{find_my_ip_address};

pub struct Client {
    socket: ClientSocketImpl,
    sender: MessageSender,
}

#[cfg(target_arch = "wasm32")]
const SERVER_IP_ADDRESS: &str = "192.168.1.6";

const SERVER_PORT: &str = "3179";
const PING_MSG: &str = "ping";
const PONG_MSG: &str = "pong";

pub fn main_common() {

    // Uncomment the line below to enable logging. You don't need it if something else (e.g. quicksilver) is logging for you
    set_logger(log::Level::Info);

    info!("Client Example Started");

    #[cfg(target_arch = "wasm32")]
    let server_socket_address = SERVER_IP_ADDRESS.to_owned() + ":" + SERVER_PORT;

    #[cfg(not(target_arch = "wasm32"))]
    let server_socket_address = find_my_ip_address::get() + ":" + SERVER_PORT;

    let mut client_socket = ClientSocketImpl::bind(&server_socket_address);

    let mut message_sender = client_socket.get_sender();

    #[cfg(not(target_arch = "wasm32"))]
    loop {
        match client_socket.receive() {
            SocketEvent::Connection() => {
                info!("Client connected to: {}", client_socket.server_address());
                message_sender.send(PING_MSG.to_string())
                    .expect("send error");
            }
            SocketEvent::Disconnection() => {
                info!("Client disconnected from: {}", client_socket.server_address());
            }
            SocketEvent::Message(message) => {
                info!("Client recv: {}", message);

                if message.eq(&PONG_MSG.to_string()) {
//                    thread::sleep(time::Duration::from_millis(1000));
                    let to_server_message: String = PING_MSG.to_string();
                    info!("Client send: {}", to_server_message);
                    message_sender.send(to_server_message)
                        .expect("send error");
                }
            }
            SocketEvent::Error(error) => {
                info!("Client error: {}", error);
            }
            SocketEvent::None => {
                //info!("Client no event");
            }
        }
    }
}

///TODO: example should have a method, loop(func: &Closure<FnMut()>)
/// in Linux, this will create a blocking loop that repeatedly calls func
/// in Wasm, this will call func every request_animation_frame, from code below
///                 we need to do this because we can't just block the main thread of the browser
///                 since it's gotta process messages from the data channel
///
/// do client_socket.receive() stuff in a closure passed to this method
/*
fn start_loop(self) {
	fn request_animation_frame(f: &Closure<FnMut()>) {
		window().unwrap()
			.request_animation_frame(f.as_ref().unchecked_ref())
			.expect("should register `requestAnimationFrame` OK");
	}

	log(format!("Starting loop...").as_ref());

	let mut rc = Rc::new(self);
	let f = Rc::new(RefCell::new(None));
	let g = f.clone();

	let c = move || {
		if let Some(the_self) = Rc::get_mut(&mut rc) {
			the_self.frame_callback();
		};
		request_animation_frame(f.borrow().as_ref().unwrap());
	};

	*g.borrow_mut() = Some(Closure::wrap(Box::new(c) as Box<FnMut()>));

	request_animation_frame(g.borrow().as_ref().unwrap());
}
*/

fn set_logger(level: log::Level) {
    #[cfg(target_arch = "wasm32")]
    web_logger::custom_init(web_logger::Config { level });

    #[cfg(not(target_arch = "wasm32"))]
    simple_logger::init_with_level(level).expect("A logger was already initialized");
}