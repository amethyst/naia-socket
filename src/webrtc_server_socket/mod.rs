
use hyper::{
    header::{self, HeaderValue},
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
    Body, Error as HyperError, Method, Response, Server, StatusCode,
};
use log::{info};
use std::{
    net::{ IpAddr, SocketAddr, TcpListener },
    io::{Error as IoError},
    collections::{HashSet},
};
use webrtc_unreliable::{Server as RtcServer, MessageType};

use futures_channel::mpsc;
use futures_util::{pin_mut, select, FutureExt, StreamExt};
use tokio::time::{self, Interval};

use super::socket_event::SocketEvent;
use super::message_sender::MessageSender;
use crate::error::NaiaServerSocketError;
use crate::Packet;
use naia_socket_shared::{Config};

const MESSAGE_BUFFER_SIZE: usize = 8;

pub struct WebrtcServerSocket {
    to_client_sender: mpsc::Sender<Packet>,
    to_client_receiver: mpsc::Receiver<Packet>,
    tick_timer: Interval,
    rtc_server: RtcServer,
    clients: HashSet<SocketAddr>,
}

impl WebrtcServerSocket {
    pub async fn listen(address: &str, config: Option<Config>) -> WebrtcServerSocket {
        let session_listen_addr: SocketAddr = address
            .parse()
            .expect("could not parse HTTP address/port");
        let webrtc_listen_ip: IpAddr = session_listen_addr.ip();
        let webrtc_listen_port = get_available_port(webrtc_listen_ip.to_string().as_str())
            .expect("no available port");
        let webrtc_listen_addr = SocketAddr::new(webrtc_listen_ip, webrtc_listen_port);

        let (to_client_sender, to_client_receiver) = mpsc::channel(MESSAGE_BUFFER_SIZE);

        let rtc_server = RtcServer::new(webrtc_listen_addr, webrtc_listen_addr).await
            .expect("could not start RTC server");

        let tick_interval = match config {
            Some(config) => config.tick_interval,
            None => Config::default().tick_interval,
        };

        let socket = WebrtcServerSocket {
            to_client_sender,
            to_client_receiver,
            rtc_server,
            tick_timer: time::interval(tick_interval),
            clients: HashSet::new(),
        };

        let session_endpoint = socket.rtc_server.session_endpoint();
        let make_svc = make_service_fn(move |addr_stream: &AddrStream| {
            let session_endpoint = session_endpoint.clone();
            let remote_addr = addr_stream.remote_addr();
            async move {
                Ok::<_, HyperError>(service_fn(move |req| {
                    let mut session_endpoint = session_endpoint.clone();
                    async move {
                        if req.uri().path() == "/new_rtc_session" && req.method() == Method::POST
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

    pub async fn receive(&mut self) -> Result<SocketEvent, NaiaServerSocketError> {

        enum Next {
            FromClientMessage(Result<Packet, IoError>),
            ToClientMessage(Packet),
            PeriodicTimer,
        }

        loop {
            let next = {
                let timer_next = self.tick_timer.tick().fuse();
                pin_mut!(timer_next);

                let to_client_receiver_next = self.to_client_receiver.next().fuse();
                pin_mut!(to_client_receiver_next);

                let rtc_server = &mut self.rtc_server;
                let from_client_message_receiver_next = rtc_server.recv().fuse();
                pin_mut!(from_client_message_receiver_next);

                select! {
                    from_client_result = from_client_message_receiver_next => {
                        Next::FromClientMessage(
                            match from_client_result {
                                Ok(msg) => {
                                    Ok(Packet::new(msg.remote_addr, msg.message.as_ref().to_vec()))
                                }
                                Err(err) => { Err(err) }
                            }
                        )
                    }
                    to_client_message = to_client_receiver_next => {
                        Next::ToClientMessage(
                            to_client_message.expect("to server message receiver closed")
                        )
                    }
                    _ = timer_next => {
                        Next::PeriodicTimer
                    }
                }
            };

            match next {
                Next::FromClientMessage(from_client_message) => {
                    match from_client_message {
                        Ok(packet) => {
                            let address = packet.address();
                            if !self.clients.contains(&address) {
                                self.clients.insert(address);
                            }
                            return Ok(SocketEvent::Packet(packet));
                        }
                        Err(err) => {
                            return Err(NaiaServerSocketError::Wrapped(Box::new(err)));
                        }
                    }
                }
                Next::ToClientMessage(packet) => {
                    let address = packet.address();

                    match self.rtc_server.send(
                        packet.payload(),
                        MessageType::Binary,
                        &address)
                        .await
                    {
                        Err(_) => {
                            return Err(NaiaServerSocketError::SendError(address));
                        }
                        _ => {}
                    }
                }
                Next::PeriodicTimer => {
                    return Ok(SocketEvent::Tick);
                }
            }
        }
    }

    pub fn get_sender(&mut self) -> MessageSender {
        return MessageSender::new(self.to_client_sender.clone());
    }

    pub fn get_clients(&mut self) -> Vec<SocketAddr> {
        self.clients.iter().cloned().collect()
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