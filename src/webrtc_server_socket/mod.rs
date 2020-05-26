
use hyper::{
    header::{self, HeaderValue},
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
    Body, Error as HyperError, Method, Response, Server, StatusCode,
};
use log::info;
use std::{
    net::{ IpAddr, SocketAddr, TcpListener },
    time::Duration,
    io::{Error as IoError},
    collections::{VecDeque, HashMap},
};
use webrtc_unreliable::{Server as RtcServer, MessageType};

use futures_channel::mpsc;
use futures_util::{pin_mut, select, FutureExt, StreamExt};
use tokio::time::{self, Interval};

use super::socket_event::SocketEvent;
use super::message_sender::MessageSender;
use crate::error::GaiaServerSocketError;
use crate::Packet;
use gaia_socket_shared::{MessageHeader, Config, ConnectionManager};

const MESSAGE_BUFFER_SIZE: usize = 8;

pub struct WebrtcServerSocket {
    to_client_sender: mpsc::Sender<Packet>,
    to_client_receiver: mpsc::Receiver<Packet>,
    tick_timer: Interval,
    heartbeat_timer: Interval,
    heartbeat_interval: Duration,
    timeout_duration: Duration,
    rtc_server: RtcServer,
    clients: HashMap<SocketAddr, ConnectionManager>,
    outstanding_disconnects: VecDeque<SocketAddr>,
}

impl WebrtcServerSocket {
    pub async fn listen(address: &str, config: Option<Config>) -> WebrtcServerSocket {
        info!("Hello WebrtcServerSocket!");

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

        let some_config = config.unwrap();
        let heartbeat_interval = some_config.heartbeat_interval / 2;
        let timeout_duration = some_config.idle_connection_timeout;

        let socket = WebrtcServerSocket {
            to_client_sender,
            to_client_receiver,
            rtc_server,
            tick_timer: time::interval(some_config.tick_interval),
            heartbeat_timer: time::interval(heartbeat_interval),
            heartbeat_interval,
            timeout_duration,
            clients: HashMap::new(),
            outstanding_disconnects: VecDeque::new()
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

    pub async fn receive(&mut self) -> Result<SocketEvent, GaiaServerSocketError> {

        enum Next {
            FromClientMessage(Result<Packet, IoError>),
            ToClientMessage(Packet),
            PeriodicTimer,
            HeartbeatTimer,
        }

        if let Some(addr) = self.outstanding_disconnects.pop_front() {
            self.clients.remove(&addr);
            return Ok(SocketEvent::Disconnection(addr));
        }

        loop {
            let next = {
                let timer_next = self.tick_timer.tick().fuse();
                pin_mut!(timer_next);

                let heartbeater_next = self.heartbeat_timer.tick().fuse();
                pin_mut!(heartbeater_next);

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
                    _ = heartbeater_next => {
                        Next::HeartbeatTimer
                    }
                }
            };

            match next {
                Next::FromClientMessage(from_client_message) => {
                    match from_client_message {
                        Ok(packet) => {

                            let address = packet.address();

                            match self.clients.get_mut(&address) {
                                Some(connection) => {
                                    connection.mark_heard();
                                }
                                None => {
                                    //not yet established connection
                                }
                            }

                            let payload = packet.payload();
                            let header: MessageHeader = payload[0].into();
                            match header {
                                MessageHeader::ClientHandshake => {
                                    // Server Handshake
                                    match self.rtc_server.send(
                                        &[MessageHeader::ServerHandshake as u8],
                                        MessageType::Binary,
                                        &address)
                                        .await
                                    {
                                        Ok(_) => {
                                            match self.clients.get_mut(&address) {
                                                Some(connection) => {
                                                    connection.mark_sent();
                                                }
                                                None => {
                                                    //sending to an unknown address??
                                                }
                                            }
                                        }
                                        Err(error) => {
                                            return Err(GaiaServerSocketError::Wrapped(Box::new(error)));
                                        }
                                    }

                                    if !self.clients.contains_key(&address) {
                                        self.clients.insert(address, ConnectionManager::new(self.heartbeat_interval, self.timeout_duration));
                                        return Ok(SocketEvent::Connection(address));
                                    }
                                }
                                MessageHeader::Data => {
                                    let boxed = payload[1..].to_vec().into_boxed_slice();
                                    let packet = Packet::new_raw(address, boxed);
                                    return Ok(SocketEvent::Packet(packet)); // trimming gets rid of the header
                                }
                                MessageHeader::Heartbeat => {
                                    // Already registered heartbeat, no need for more
                                }
                                _ => {}
                            }
                        }
                        Err(err) => {
                            return Err(GaiaServerSocketError::Wrapped(Box::new(err)));
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
                        Ok(_) => {
                            match self.clients.get_mut(&address) {
                                Some(connection) => {
                                    connection.mark_sent();
                                }
                                None => {
                                    //sending to an unknown address??
                                }
                            }
                        }
                        Err(error) => {
                            return Err(GaiaServerSocketError::Wrapped(Box::new(error)));
                        }
                    }
                }
                Next::PeriodicTimer => {
                    return Ok(SocketEvent::Tick);
                }
                Next::HeartbeatTimer => {

                    for (address, connection) in self.clients.iter_mut() {
                        if connection.should_drop() {
                            self.outstanding_disconnects.push_back(*address);
                        }
                        else if connection.should_send_heartbeat() {
                            match self.rtc_server.send(
                                &[MessageHeader::Heartbeat as u8],
                                MessageType::Binary,
                                &address)
                                .await
                                {
                                    Ok(_) => {
                                        connection.mark_sent();
                                    }
                                    Err(error) => {
                                        return Err(GaiaServerSocketError::Wrapped(Box::new(error)));
                                    }
                                }
                        }
                    }

                    if let Some(addr) = self.outstanding_disconnects.pop_front() {
                        self.clients.remove(&addr);
                        return Ok(SocketEvent::Disconnection(addr));
                    }
                }
            }
        }
    }

    pub fn get_sender(&mut self) -> MessageSender {
        return MessageSender::new(self.to_client_sender.clone());
    }

    pub fn get_clients(&mut self) -> Vec<SocketAddr> {
        self.clients.keys().cloned().collect()
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