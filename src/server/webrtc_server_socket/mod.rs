
use hyper::{
    header::{self, HeaderValue},
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
    Body, Error as HyperError, Method, Response, Server, StatusCode,
};
use log::{info, warn};
use std::{
    error::Error,
    net::{ IpAddr, SocketAddr, TcpListener },
    time::{ Duration },
};
use async_trait::async_trait;
use webrtc_unreliable::{Server as RtcServer, MessageType, MessageResult, RecvError, ClientEvent as RtcEvent};

use futures_channel::mpsc;
use futures_util::{pin_mut, select, FutureExt, StreamExt};
use tokio::time::{self, Interval};

use crate::server::ServerSocket;
use super::client_event::ClientEvent;

const MESSAGE_BUFFER_SIZE: usize = 8;
const EVENT_BUFFER_SIZE: usize = 8;
const PERIODIC_TIMER_INTERVAL: Duration = Duration::from_secs(1);

pub struct WebrtcServerSocket {
    to_server_sender: mpsc::Sender<ClientEvent>,
    to_server_receiver: mpsc::Receiver<ClientEvent>,
    to_client_sender: mpsc::Sender<ClientEvent>,
    to_client_receiver: mpsc::Receiver<ClientEvent>,
    to_client_event_receiver: mpsc::Receiver<RtcEvent>,
    periodic_timer: Interval,
    rtc_server: RtcServer,
    message_buf: Vec<u8>,
}

#[async_trait]
impl ServerSocket for WebrtcServerSocket {
    async fn bind(address: &str) -> WebrtcServerSocket {
        println!("Hello WebrtcServerSocket!");

        env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

        let session_listen_addr: SocketAddr = address
            .parse()
            .expect("could not parse HTTP address/port");
        let webrtc_listen_ip: IpAddr = session_listen_addr.ip();
        let webrtc_listen_port = get_available_port(webrtc_listen_ip.to_string().as_str())
            .expect("no available port");
        let webrtc_listen_addr = SocketAddr::new(webrtc_listen_ip, webrtc_listen_port);

        let (to_server_sender, to_server_receiver) = mpsc::channel(MESSAGE_BUFFER_SIZE);
        let (to_client_sender, to_client_receiver) = mpsc::channel(MESSAGE_BUFFER_SIZE);

        let (rtc_server, to_client_event_receiver) = RtcServer::new(webrtc_listen_addr, webrtc_listen_addr).await
            .expect("could not start RTC server");
        let socket = WebrtcServerSocket {
            to_server_sender,
            to_server_receiver,
            to_client_sender,
            to_client_receiver,
            rtc_server,
            to_client_event_receiver,
            message_buf: vec![0; 0x10000],
            periodic_timer: time::interval(PERIODIC_TIMER_INTERVAL),
        };

        let session_endpoint = socket.rtc_server.session_endpoint();
        let make_svc = make_service_fn(move |addr_stream: &AddrStream| {
            let session_endpoint = session_endpoint.clone();
            let remote_addr = addr_stream.remote_addr();
            async move {
                Ok::<_, HyperError>(service_fn(move |req| {
                    let mut session_endpoint = session_endpoint.clone();
                    async move {
                        if req.uri().path() == "/"
                            || req.uri().path() == "/index.html" && req.method() == Method::GET
                        {
                            info!("serving example index HTML to {}", remote_addr);
                            Response::builder().body(Body::from(include_str!("./echo_server.html")))
                        } else if req.uri().path() == "/new_rtc_session" && req.method() == Method::POST
                        {
                            info!("WebRTC session request from {}", remote_addr);
                            match session_endpoint.http_session_request(req.into_body()).await {
                                Ok(mut resp) => {
                                    resp.headers_mut().insert(
                                        header::ACCESS_CONTROL_ALLOW_ORIGIN,
                                        HeaderValue::from_static("*"),
                                    );
                                    Ok(resp.map(Body::from))
                                }
                                Err(err) => Response::builder()
                                    .status(StatusCode::BAD_REQUEST)
                                    .body(Body::from(format!("error: {}", err))),
                            }
                        } else {
                            Response::builder()
                                .status(StatusCode::NOT_FOUND)
                                .body(Body::from("not found"))
                        }
                    }
                }))
            }
        });

        tokio::spawn(async move {
            Server::bind(&session_listen_addr)
                .serve(make_svc)
                .await
                .expect("HTTP session server has died");
        });

        socket
    }

    fn get_sender(&mut self) -> mpsc::Sender<ClientEvent> {
        return self.to_server_sender.clone();
    }

    async fn receive(&mut self) -> Result<ClientEvent, Box<dyn Error>> {

        enum Next {
            IncomingEvent(RtcEvent),
            IncomingMessage(Result<MessageResult, RecvError>),
            OutgoingMessage(ClientEvent),
            PeriodicTimer,
        }

        loop {
            let next = {
                let timer_next = self.periodic_timer.tick().fuse();
                pin_mut!(timer_next);

                let to_client_receiver_next = self.to_client_receiver.next().fuse();
                pin_mut!(to_client_receiver_next);

                let rtc_event_receiver_next = self.to_client_event_receiver.next().fuse();
                pin_mut!(rtc_event_receiver_next);

                let rs = &mut self.rtc_server;
                let something2 = rs.recv(&mut self.message_buf);
                let rtc_msg_receiver_next = something2.fuse();
                pin_mut!(rtc_msg_receiver_next);

                select! {
                    incoming_message = rtc_msg_receiver_next => {
                        Next::IncomingMessage(
                            incoming_message
                        )
                    }
                    outgoing_message = to_client_receiver_next => {
                        Next::OutgoingMessage(
                            outgoing_message.expect("to client message receiver closed")
                        )
                    }
                    incoming_event = rtc_event_receiver_next => {
                        Next::IncomingEvent(
                            incoming_event.expect("from server event receiver closed")
                        )
                    }
                    _ = timer_next => {
                        Next::PeriodicTimer
                    }
                }
            };

            match next {
                Next::IncomingEvent(incoming_event) => {
                    match incoming_event {
                        RtcEvent::Connection(address) => {
                            return Ok(ClientEvent::Connection(address));
                        }
                        RtcEvent::Disconnection(address) => {
                            return Ok(ClientEvent::Disconnection(address));
                        }
                    }
                }
                Next::IncomingMessage(incoming_message) => {
                    match incoming_message {
                        Ok(message_result) => {
                            let packet_payload = &self.message_buf[0..message_result.message_len];
                            //let message_type = message_result.message_type;
                            let address = message_result.remote_addr;

                            let message = String::from_utf8_lossy(packet_payload);

                            return Ok(ClientEvent::Message(address, message.to_string()))
                        }
                        Err(err) => {
                            warn!("could not receive RTC message: {}", err);
                        }
                    }
                }
                Next::OutgoingMessage(ClientEvent::Message(address, message)) => {
                    self.rtc_server.send(
                        message.into_bytes().as_slice(),
                        MessageType::Text,
                        &address
                    ).await;
                }
                Next::PeriodicTimer => {
                    return Ok(ClientEvent::Tick);
                }
                _ => {
                    println!("How did we get here?");
                }
            }
        }
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