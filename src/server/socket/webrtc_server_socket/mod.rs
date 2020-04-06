use crate::server::socket::ServerSocket;
use super::client_socket::ClientSocket;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};
use std::str;

use futures::{
    future::{self, Either, IntoFuture},
    Async, Future,
};
use hyper::{
    header::{self, HeaderValue},
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
    Body, Method, Response, Server, StatusCode,
};
use log::{info, warn};
use tokio::runtime::Runtime;

mod webrtc_unreliable;
use webrtc_unreliable::{
    MessageResult as RtcMessageResult, RecvError as RtcRecvError, SendError as RtcSendError,
    Server as RtcServer,
    MessageType
};

use std::net::TcpListener;
use crate::server::socket::webrtc_server_socket::webrtc_unreliable::MessageResult;
use std::borrow::Borrow;
use std::sync::mpsc::{channel, Sender, Receiver};

pub struct WebrtcServerSocket {
    connect_function: Option<Box<dyn Fn(&ClientSocket)>>,
    receive_function: Option<Box<dyn Fn(&ClientSocket, &str)>>,
    disconnect_function: Option<Box<dyn Fn(IpAddr)>>,
}

struct ClientSocketMessage {
    ip_address: SocketAddr,
    message: String
}

impl ServerSocket for WebrtcServerSocket {
    fn new() -> WebrtcServerSocket {
        println!("Hello WebrtcServerSocket!");

        let new_server_socket = WebrtcServerSocket {
            connect_function: None,
            receive_function: None,
            disconnect_function: None
        };

        new_server_socket
    }

    fn listen(&self, address: &str) {

        env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

        let mut runtime = Runtime::new().expect("could not build tokio runtime");

        let session_listen_addr: SocketAddr = address
            .parse()
            .expect("could not parse HTTP address/port");

        let webrtc_listen_ip: IpAddr = session_listen_addr.ip();
        let webrtc_listen_port = get_available_port(webrtc_listen_ip.to_string().as_str())
            .expect("no available port");
        let webrtc_listen_addr = SocketAddr::new(webrtc_listen_ip, webrtc_listen_port);

        let (rtc_connect_sender, rtc_connect_receiver): (Sender<SocketAddr>, Receiver<SocketAddr>) = channel();
        let mut rtc_server =  RtcServer::new(webrtc_listen_addr, webrtc_listen_addr, rtc_connect_sender)
            .expect("could not start RTC server");

        /// Start of HTTP Listener ///
        let session_endpoint = rtc_server.session_endpoint();

        runtime.spawn(Box::new(
            Server::bind(&session_listen_addr)
                .serve(make_service_fn(move |addr_stream: &AddrStream| {
                    let session_endpoint = session_endpoint.clone();
                    let remote_addr = addr_stream.remote_addr();
                    service_fn(move |req| {
                        if req.uri().path() == "/"
                            || req.uri().path() == "/index.html" && req.method() == Method::GET
                        {
                            info!("serving example index HTML to {}", remote_addr);
                            Either::A(
                                Response::builder()
                                    .body(Body::from(include_str!("./echo_server.html")))
                                    .into_future(),
                            )
                        } else if req.uri().path() == "/new_rtc_session" && req.method() == Method::POST
                        {
                            info!("WebRTC session request from {}", remote_addr);
                            Either::B(
                                session_endpoint
                                    .http_session_request(req.into_body())
                                    .map(|mut resp| {
                                        resp.headers_mut().insert(
                                            header::ACCESS_CONTROL_ALLOW_ORIGIN,
                                            HeaderValue::from_static("*"),
                                        );
                                        resp.map(Body::from)
                                    })
                                    .then(|resp| match resp {
                                        Ok(resp) => Either::A(future::ok(resp)),
                                        Err(err) => Either::B(
                                            Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("error: {}", err)))
                                                .into_future(),
                                        ),
                                    }),
                            )
                        } else {
                            Either::A(
                                Response::builder()
                                    .status(StatusCode::NOT_FOUND)
                                    .body(Body::from("not found"))
                                    .into_future(),
                            )
                        }
                    })
                }))
                .map_err(|e| panic!("HTTP session server has died! {}", e)),
        ));

        /// End of HTTP Listener ///

        /// Start of WebRtc Listener ///

        let mut message_buf = vec![0; 0x10000];

        let (rtc_receive_sender, rtc_receive_receiver) = channel();
        let (rtc_send_sender, rtc_send_receiver): (Sender<ClientSocketMessage>, Receiver<ClientSocketMessage>) = channel();

        runtime.spawn(Box::new(future::poll_fn(move || loop {

            match rtc_server.poll_recv(&mut message_buf) {
                Ok(Async::Ready(incoming_message)) => {
                    let msg_str: &str = str::from_utf8(&message_buf[0..incoming_message.message_len])
                        .expect("cannot convert incoming message to string");

                    let total_package = ClientSocketMessage {
                        ip_address: incoming_message.remote_addr,
                        message: String::from(msg_str)
                    };

                    rtc_receive_sender.send(total_package);
                }
                Ok(Async::NotReady) => return Ok(Async::NotReady),
                Err(RtcRecvError::Internal(err)) => panic!("internal WebRTC server error: {}", err),
                Err(err) => warn!("could not receive RTC message: {}", err),
            }

            match rtc_send_receiver.recv() {
                Ok(incoming_message) => {

                    let outgoing_message: &[u8] = incoming_message.message.as_bytes();

                    match rtc_server.poll_send(
                        outgoing_message,
                        MessageType::Text,
                        &incoming_message.ip_address) {
                            Ok(Async::Ready(())) => {}
                            Ok(Async::NotReady) => {
                                return Ok(Async::NotReady);
                            }
                            Err(RtcSendError::Internal(err)) => {
                                panic!("internal WebRTC server error: {}", err)
                            }
                            Err(err) => warn!(
                                "could not send message to {}: {}",
                                incoming_message.ip_address, err
                            ),
                    }
                }
                Err(_) => {
                    warn!("main send_receive loop error")
                }
            }
        })));

        /// End of WebRtc Listener ///

        /// Start of Blocking Loop to send/receive with WebRtc thread ///

        loop {
            match rtc_receive_receiver.recv() {
                Ok(incoming_message) => {
                    let ip_address = incoming_message.ip_address;
                    let rtc_send_sender_copy = rtc_send_sender.clone();
                    let send_func = move |msg: &str| {
                        let total_package = ClientSocketMessage {
                            ip_address,
                            message: String::from(msg)
                        };
                        rtc_send_sender_copy.send(total_package);
                    };

                    let client_socket = ClientSocket::new(
                        ip_address.ip(),
                        send_func);

                    (self.receive_function.as_ref().unwrap())(&client_socket, &incoming_message.message);
                }
                Err(_) => {
                    warn!("main receive_receive loop error")
                }
            }

            match rtc_connect_receiver.recv() {
                Ok(connect_addr) => {
                    let rtc_send_sender_copy = rtc_send_sender.clone();
                    let send_func = move |msg: &str| {
                        let total_package = ClientSocketMessage {
                            ip_address: SocketAddr::from(connect_addr),
                            message: String::from(msg)
                        };
                        rtc_send_sender_copy.send(total_package);
                    };

                    let client_socket = ClientSocket::new(
                        connect_addr.ip(),
                        send_func);

                    (self.connect_function.as_ref().unwrap())(&client_socket);
                }
                Err(_) => {

                }
            }
        }

        /// End of Blocking Loop to send/receive with WebRtc thread ///

        runtime.shutdown_on_idle().wait().unwrap();
    }

    fn on_connection(&mut self, func: impl Fn(&ClientSocket) + 'static) {
        self.connect_function = Some(Box::new(func));
    }

    fn on_receive(&mut self, func: impl Fn(&ClientSocket, &str) + 'static) {
        self.receive_function = Some(Box::new(func));
    }

    fn on_disconnection(&mut self, func: impl Fn(IpAddr) + 'static) {
        self.disconnect_function = Some(Box::new(func));
    }
}

fn get_available_port(ip: &str) -> Option<u16> {
    (8000..9000)
        .find(|port| port_is_available(ip, *port))
}

fn port_is_available(ip: &str, port: u16) -> bool {
    match TcpListener::bind((ip, port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}