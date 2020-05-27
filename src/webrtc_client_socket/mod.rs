
extern crate log;
use log::info;

use std::net::SocketAddr;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::VecDeque;

use super::socket_event::SocketEvent;
use super::message_sender::MessageSender;
use crate::error::GaiaClientSocketError;
use crate::Packet;
use std::time::Duration;

use gaia_socket_shared::{MessageHeader, Config, ConnectionManager, Timer};

pub struct WebrtcClientSocket {
    address: SocketAddr,
    data_channel: RtcDataChannel,
    message_queue: Rc<RefCell<VecDeque<Result<SocketEvent, GaiaClientSocketError>>>>,
    connected: bool,
    connection_manager: Rc<RefCell<ConnectionManager>>,
    message_sender: MessageSender,
    config: Config,
    handshake_timer: Option<Timer>,
    dropped_outgoing_messages: Rc<RefCell<VecDeque<Packet>>>,
}

impl WebrtcClientSocket {

    pub fn connect(server_address: &str, config: Option<Config>) -> WebrtcClientSocket {
        let message_queue = Rc::new(RefCell::new(VecDeque::new()));
        let data_channel = webrtc_initialize(server_address, message_queue.clone());

        let mut some_config = match config {
            Some(config) => config,
            None => Config::default(),
        };
        some_config.heartbeat_interval /= 2;

        let dropped_outgoing_messages = Rc::new(RefCell::new(VecDeque::new()));
        let connection_manager = match some_config.connectionless {
            false => Rc::new(RefCell::new(ConnectionManager::new(some_config.heartbeat_interval, some_config.disconnection_timeout_duration))),
            true => Rc::new(RefCell::new(ConnectionManager::connectionless())),
        };
        let message_sender = MessageSender::new(data_channel.clone(), connection_manager.clone(), dropped_outgoing_messages.clone());

        let mut handshake_timer = None;
        let mut connected= true;
        if !some_config.connectionless {
            handshake_timer = Some(Timer::new(some_config.send_handshake_interval));
            handshake_timer.as_mut().unwrap().ring_manual();
            connected = false;
        }

        WebrtcClientSocket {
            address: server_address.parse().unwrap(),
            data_channel,
            message_queue,
            connected,
            handshake_timer,
            connection_manager,
            message_sender,
            config: some_config,
            dropped_outgoing_messages,
        }
    }

    pub fn receive(&mut self) -> Result<SocketEvent, GaiaClientSocketError> {

        if !self.config.connectionless {
            if self.connected {
                if self.connection_manager.borrow().should_drop() {
                    self.connected = false;
                    return Ok(SocketEvent::Disconnection);
                }
                if self.connection_manager.borrow().should_send_heartbeat() {
                    self.data_channel.send_with_str(std::str::from_utf8(&[MessageHeader::Heartbeat as u8]).unwrap());
                    self.connection_manager.borrow_mut().mark_sent();
                }
            } else {
                if self.handshake_timer.as_ref().unwrap().ringing() {
                    self.data_channel.send_with_str(std::str::from_utf8(&[MessageHeader::ClientHandshake as u8]).unwrap());
                    self.handshake_timer.as_mut().unwrap().reset();
                }
            }
        }

        if !self.dropped_outgoing_messages.borrow().is_empty() {
            let mut some_dropped_packets: Option<Vec<Packet>> = None;
            {
                let mut dom = self.dropped_outgoing_messages.borrow_mut();
                let dropped_packets: Vec<Packet> = dom.drain(..).collect::<Vec<Packet>>();
                some_dropped_packets = Some(dropped_packets);
            }
            if let Some(dropped_packets) = some_dropped_packets {
                for dropped_packet in dropped_packets {
                    self.message_sender.send(dropped_packet);
                }
            }
        }

        loop {
            if self.message_queue.borrow().is_empty() {
                return Ok(SocketEvent::None);
            }

            match self.message_queue.borrow_mut()
                .pop_front()
                .expect("message queue shouldn't be empty!") {
                Ok(SocketEvent::Packet(packet)) => {

                    if self.config.connectionless {
                        return Ok(SocketEvent::Packet(packet));
                    }
                    else {
                        self.connection_manager.borrow_mut().mark_heard();

                        let payload = packet.payload();
                        let header: MessageHeader = payload[0].into();
                        match header {
                            MessageHeader::ServerHandshake => {
                                if !self.connected {
                                    self.connected = true;
                                    return Ok(SocketEvent::Connection);
                                }
                            }
                            MessageHeader::Data => {
                                let boxed = payload[1..].to_vec().into_boxed_slice();
                                let packet = Packet::new_raw(boxed);
                                return Ok(SocketEvent::Packet(packet));
                            }
                            MessageHeader::Heartbeat => {
                                // Already registered heartbeat, no need for more
                                info!("Heartbeat");
                            }
                            _ => {}
                        }
                    }
                }
                Ok(inner) => { return Ok(inner); }
                Err(err) => { return Err(err); }
            }
        }
    }

    pub fn get_sender(&mut self) -> MessageSender {
        return self.message_sender.clone();
    }

    pub fn server_address(&self) -> SocketAddr {
        return self.address;
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

use wasm_bindgen::prelude::*;
use wasm_bindgen::{ JsCast, JsValue };
use web_sys::{ RtcConfiguration, RtcDataChannel, RtcDataChannelInit, RtcDataChannelType,
               RtcIceCandidate, RtcIceCandidateInit,
               RtcPeerConnection, RtcSdpType,
               RtcSessionDescription, RtcSessionDescriptionInit,
               XmlHttpRequest, MessageEvent, ProgressEvent, ErrorEvent };

#[derive(Deserialize, Debug, Clone)]
pub struct SessionAnswer {
    pub sdp: String,

    #[serde(rename = "type")]
    pub _type: String,
}

#[derive(Deserialize, Debug)]
pub struct SessionCandidate {
    pub candidate: String,
    pub sdpMLineIndex: u16,
    pub sdpMid: String,
}

#[derive(Deserialize, Debug)]
pub struct JsSessionResponse {
    pub answer: SessionAnswer,
    pub candidate: SessionCandidate,
}

#[derive(Serialize)]
pub struct IceServerConfig {
    pub urls: [String; 1],
}

fn webrtc_initialize(address: &str, msg_queue: Rc<RefCell<VecDeque<Result<SocketEvent, GaiaClientSocketError>>>>) -> RtcDataChannel {

    let server_url_str: String = "http://".to_string() + address + "/new_rtc_session";

    let mut peer_config: RtcConfiguration = RtcConfiguration::new();
    let ice_server_config = IceServerConfig {
        urls: ["stun:stun.l.google.com:19302".to_string()]
    };
    let ice_server_config_list = [ ice_server_config ];

    peer_config.ice_servers(&JsValue::from_serde(&ice_server_config_list).unwrap());

    let peer: RtcPeerConnection = RtcPeerConnection::new_with_configuration(&peer_config).unwrap();

    let mut data_channel_config: RtcDataChannelInit = RtcDataChannelInit::new();
    data_channel_config.ordered(false);
    data_channel_config.max_retransmits(0);

    let channel: RtcDataChannel = peer.create_data_channel_with_data_channel_dict("webudp", &data_channel_config);
    channel.set_binary_type(RtcDataChannelType::Arraybuffer);

    let cloned_channel = channel.clone();
    let msg_queue_clone = msg_queue.clone();
    let channel_onopen_closure = Closure::wrap(Box::new(move |_| {

        let msg_queue_clone_2 = msg_queue_clone.clone();
        let channel_onmsg_closure = Closure::wrap(Box::new(move |evt: MessageEvent| {

            if let Ok(arraybuf) = evt.data().dyn_into::<js_sys::ArrayBuffer>() {
                let uarray: js_sys::Uint8Array = js_sys::Uint8Array::new(&arraybuf);
                let mut body = vec![0; uarray.length() as usize];
                uarray.copy_to(&mut body[..]);
                msg_queue_clone_2
                    .borrow_mut()
                    .push_back(Ok(SocketEvent::Packet(Packet::new(body))));
            }

        }) as Box<dyn FnMut(MessageEvent)>);

        cloned_channel.set_onmessage(Some(channel_onmsg_closure.as_ref().unchecked_ref()));
        channel_onmsg_closure.forget();

    }) as Box<dyn FnMut(JsValue)>);
    channel.set_onopen(Some(channel_onopen_closure.as_ref().unchecked_ref()));
    channel_onopen_closure.forget();

    let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
        info!("data channel error event: {:?}", e);
    }) as Box<dyn FnMut(ErrorEvent)>);
    channel.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    onerror_callback.forget();

    let peer_clone = peer.clone();
    let server_url_msg = Rc::new(server_url_str);
    let peer_offer_callback = Closure::wrap(Box::new(move |e: JsValue| {

        let session_description = e.dyn_into::<RtcSessionDescription>().unwrap();
        let peer_clone_2 = peer_clone.clone();
        let server_url_msg_clone = server_url_msg.clone();
        let peer_desc_callback = Closure::wrap(Box::new(move |_: JsValue| {

            let request = XmlHttpRequest::new()
                .expect("can't create new XmlHttpRequest");

            request.open("POST", &server_url_msg_clone);

            let request_2 = request.clone();
            let peer_clone_3 = peer_clone_2.clone();
            let request_callback = Closure::wrap(Box::new(move |_: ProgressEvent| {

                if request_2.status().unwrap() == 200 {
                    let response_string = request_2.response_text().unwrap().unwrap();
                    let response_js_value = js_sys::JSON::parse(response_string.as_str()).unwrap();
                    let session_response: JsSessionResponse = response_js_value.into_serde().unwrap();
                    let session_response_answer: SessionAnswer = session_response.answer.clone();

                    let peer_clone_4 = peer_clone_3.clone();
                    let remote_desc_success_callback = Closure::wrap(Box::new(move |e: JsValue| {

                        let mut candidate_init_dict: RtcIceCandidateInit = RtcIceCandidateInit::new(session_response.candidate.candidate.as_str());
                        candidate_init_dict.sdp_m_line_index(Some(session_response.candidate.sdpMLineIndex));
                        candidate_init_dict.sdp_mid(Some(session_response.candidate.sdpMid.as_str()));
                        let candidate: RtcIceCandidate = RtcIceCandidate::new(&candidate_init_dict).unwrap();

                        let peer_add_success_callback = Closure::wrap(Box::new(move |_: JsValue| {
                            //Client add ice candidate success
                        }) as Box<dyn FnMut(JsValue)>);
                        let peer_add_failure_callback = Closure::wrap(Box::new(move |_: JsValue| {
                            info!("Client error during 'addIceCandidate': {:?}", e);
                        }) as Box<dyn FnMut(JsValue)>);

                        peer_clone_4.add_ice_candidate_with_rtc_ice_candidate_and_success_callback_and_failure_callback(
                            &candidate,
                            peer_add_success_callback.as_ref().unchecked_ref(),
                            peer_add_failure_callback.as_ref().unchecked_ref());
                        peer_add_success_callback.forget();
                        peer_add_failure_callback.forget();

                    }) as Box<dyn FnMut(JsValue)>);

                    let remote_desc_failure_callback = Closure::wrap(Box::new(move |_: JsValue| {
                        info!("Client error during 'setRemoteDescription': TODO, put value here");
                    }) as Box<dyn FnMut(JsValue)>);

                    let mut rtc_session_desc_init_dict: RtcSessionDescriptionInit = RtcSessionDescriptionInit::new(RtcSdpType::Answer);

                    rtc_session_desc_init_dict.sdp(session_response_answer.sdp.as_str());

                    peer_clone_3.set_remote_description_with_success_callback_and_failure_callback(
                        &rtc_session_desc_init_dict,
                        remote_desc_success_callback.as_ref().unchecked_ref(),
                        remote_desc_failure_callback.as_ref().unchecked_ref());
                    remote_desc_success_callback.forget();
                    remote_desc_failure_callback.forget();
                }
            }) as Box<dyn FnMut(ProgressEvent)>);
            request.set_onload(Some(request_callback.as_ref().unchecked_ref()));
            request_callback.forget();

            request.send_with_opt_str(Some(peer_clone_2.local_description().unwrap().sdp().as_str()));

        }) as Box<dyn FnMut(JsValue)>);

        let mut session_description_init: RtcSessionDescriptionInit = RtcSessionDescriptionInit::new(session_description.type_());
        session_description_init.sdp(session_description.sdp().as_str());
        peer_clone.set_local_description(&session_description_init)
            .then(&peer_desc_callback);
        peer_desc_callback.forget();
    }) as Box<dyn FnMut(JsValue)>);

    let peer_error_callback = Closure::wrap(Box::new(move |_: JsValue| {
        info!("Client error during 'createOffer': e value here? TODO");
    }) as Box<dyn FnMut(JsValue)>);

    peer.create_offer()
        .then(&peer_offer_callback);

    peer_offer_callback.forget();
    peer_error_callback.forget();

    return channel;
}