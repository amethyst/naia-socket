//
//use clap::{App, Arg};
//use hyper::{
//    header::{self, HeaderValue},
//    server::conn::AddrStream,
//    service::{make_service_fn, service_fn},
//    Body, Error, Method, Response, Server, StatusCode,
//};
//use log::{info, warn};
//use std::net::{ IpAddr, SocketAddr, TcpListener };
//use async_trait::async_trait;
//use webrtc_unreliable::Server as RtcServer;
//
//use crate::server::ServerSocket;
//use super::client_socket::ClientSocket;
//
//pub struct WebrtcServerSocket {
//    connect_function: Option<Box<dyn Fn(&ClientSocket)>>,
//    receive_function: Option<Box<dyn Fn(&ClientSocket, &str)>>,
//    disconnect_function: Option<Box<dyn Fn(&ClientSocket)>>,
//}
//
//#[async_trait]
//impl ServerSocket for WebrtcServerSocket {
//    fn new() -> WebrtcServerSocket {
//        println!("Hello WebrtcServerSocket!");
//
//        let new_server_socket = WebrtcServerSocket {
//            connect_function: None,
//            receive_function: None,
//            disconnect_function: None
//        };
//
//        new_server_socket
//    }
//
//    async fn listen(&self, address: &str) {
//        let session_listen_addr: SocketAddr = address
//            .parse()
//            .expect("could not parse HTTP address/port");
//        let webrtc_listen_ip: IpAddr = session_listen_addr.ip();
//        let webrtc_listen_port = get_available_port(webrtc_listen_ip.to_string().as_str())
//            .expect("no available port");
//        let webrtc_listen_addr = SocketAddr::new(webrtc_listen_ip, webrtc_listen_port);
//
//        let mut rtc_server = RtcServer::new(webrtc_listen_addr, webrtc_listen_addr)
//            .await
//            .expect("could not start RTC server");
//
//        let session_endpoint = rtc_server.session_endpoint();
//        let make_svc = make_service_fn(move |addr_stream: &AddrStream| {
//            let session_endpoint = session_endpoint.clone();
//            let remote_addr = addr_stream.remote_addr();
//            async move {
//                Ok::<_, Error>(service_fn(move |req| {
//                    let mut session_endpoint = session_endpoint.clone();
//                    async move {
//                        if req.uri().path() == "/"
//                            || req.uri().path() == "/index.html" && req.method() == Method::GET
//                        {
//                            info!("serving example index HTML to {}", remote_addr);
//                            Response::builder().body(Body::from(include_str!("./echo_server.html")))
//                        } else if req.uri().path() == "/new_rtc_session" && req.method() == Method::POST
//                        {
//                            info!("WebRTC session request from {}", remote_addr);
//                            match session_endpoint.http_session_request(req.into_body()).await {
//                                Ok(mut resp) => {
//                                    resp.headers_mut().insert(
//                                        header::ACCESS_CONTROL_ALLOW_ORIGIN,
//                                        HeaderValue::from_static("*"),
//                                    );
//                                    Ok(resp.map(Body::from))
//                                }
//                                Err(err) => Response::builder()
//                                    .status(StatusCode::BAD_REQUEST)
//                                    .body(Body::from(format!("error: {}", err))),
//                            }
//                        } else {
//                            Response::builder()
//                                .status(StatusCode::NOT_FOUND)
//                                .body(Body::from("not found"))
//                        }
//                    }
//                }))
//            }
//        });
//
//        tokio::spawn(async move {
//            Server::bind(&session_listen_addr)
//                .serve(make_svc)
//                .await
//                .expect("HTTP session server has died");
//        });
//
//        let mut message_buf = vec![0; 0x10000];
//        loop {
//            match rtc_server.recv(&mut message_buf).await {
//                Ok(received) => {
//                    if let Err(err) = rtc_server
//                        .send(
//                            &message_buf[0..received.message_len],
//                            received.message_type,
//                            &received.remote_addr,
//                        )
//                        .await
//                    {
//                        warn!(
//                            "could not send message to {}: {}",
//                            received.remote_addr, err
//                        )
//                    }
//                }
//                Err(err) => warn!("could not receive RTC message: {}", err),
//            }
//        }
//    }
//
//    fn on_connection(&mut self, func: impl Fn(&ClientSocket) + 'static) {
//        self.connect_function = Some(Box::new(func));
//    }
//
//    fn on_receive(&mut self, func: impl Fn(&ClientSocket, &str) + 'static) {
//        self.receive_function = Some(Box::new(func));
//    }
//
//    fn on_error(&mut self, func: impl Fn(&ClientSocket, &str) + 'static) {
//        unimplemented!()
//    }
//
//    fn on_disconnection(&mut self, func: impl Fn(&ClientSocket) + 'static) {
//        self.disconnect_function = Some(Box::new(func));
//    }
//}
//
//fn get_available_port(ip: &str) -> Option<u16> {
//    (8000..9000)
//        .find(|port| port_is_available(ip, *port))
//}
//
//fn port_is_available(ip: &str, port: u16) -> bool {
//    match TcpListener::bind((ip, port)) {
//        Ok(_) => true,
//        Err(_) => false,
//    }
//}