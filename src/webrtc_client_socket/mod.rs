
use std::net::{SocketAddr, Ipv4Addr, IpAddr};
use std::rc::{Rc};

use crate::{ClientSocket};
use super::socket_event::SocketEvent;
use super::message_sender::MessageSender;
//use crate::internal_shared::{CLIENT_HANDSHAKE_MESSAGE, SERVER_HANDSHAKE_MESSAGE};

pub struct WebrtcClientSocket {
    address: SocketAddr,
    data_channel: RtcDataChannel,
}

impl ClientSocket for WebrtcClientSocket {

    fn bind(address: &str) -> WebrtcClientSocket {
        info!("Hello WebrtcClientSocket!");

        let data_channel = setup_webrtc_stuff(address);

        WebrtcClientSocket {
            address: address.parse().unwrap(),
            data_channel
        }
    }

    fn receive(&mut self) -> SocketEvent {
        return SocketEvent::None;
    }

    fn get_sender(&mut self) -> MessageSender {
        return MessageSender::new(self.data_channel.clone());
    }

    fn server_address(&self) -> SocketAddr {
        return self.address;
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

use log::{info};

use wasm_bindgen::prelude::*;
use wasm_bindgen::{ JsCast, JsValue };
use web_sys::{ RtcConfiguration, RtcDataChannel, RtcDataChannelInit, RtcDataChannelType, RtcIceCandidate, RtcIceCandidateInit,
               RtcPeerConnection, RtcPeerConnectionIceEvent, RtcSdpType, RtcSessionDescription, RtcSessionDescriptionInit,
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

pub fn setup_webrtc_stuff(address: &str) -> RtcDataChannel {

    let server_url_str: String = "http://".to_string() + address + "/new_rtc_session";

    info!("Server URL: {}", server_url_str);

    let server_url_msg = Rc::new(server_url_str);
    const PING_MSG: &str = "ping";
    const PONG_MSG: &str = "pong";

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

    info!("WebRTC Client initialized");

    let cloned_channel = channel.clone();
    let channel_onopen_closure = Closure::wrap(Box::new(move |_| {

        cloned_channel.send_with_str(PING_MSG);

        let cloned_channel_2 = cloned_channel.clone();
        let channel_onmsg_closure = Closure::wrap(Box::new(move |evt: MessageEvent| {
            if let Ok(abuf) = evt.data().dyn_into::<js_sys::ArrayBuffer>() {
                info!("message event, received arraybuffer: {:?}", abuf);
            } else if let Ok(blob) = evt.data().dyn_into::<web_sys::Blob>() {
                info!("message event, received blob: {:?}", blob);
            } else if let Ok(txt) = evt.data().dyn_into::<js_sys::JsString>() {
                info!("message event, received Text: {:?}", txt);
                cloned_channel_2.send_with_str(PING_MSG);
            } else {
                info!("message event, received Unknown: {:?}", evt.data());
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

    let onicecandidate_callback = Closure::wrap(Box::new(move |e: RtcPeerConnectionIceEvent| {
        match e.candidate() {
            Some(ice_candidate) => {
                info!("Client received ice candidate: {:?}", ice_candidate.candidate());
            }
            None => {
                info!("Client received all local candidates");
            }
        }
    }) as Box<dyn FnMut(RtcPeerConnectionIceEvent)>);
    peer.set_onicecandidate(Some(onicecandidate_callback.as_ref().unchecked_ref()));
    onicecandidate_callback.forget();

    let peer_clone = peer.clone();
    let peer_offer_callback = Closure::wrap(Box::new(move |e: JsValue| {

        let session_description = e.dyn_into::<RtcSessionDescription>().unwrap();
        let peer_clone_2 = peer_clone.clone();
        let server_url_msg_clone = server_url_msg.clone();
        let peer_desc_callback = Closure::wrap(Box::new(move |e: JsValue| {

            let request = XmlHttpRequest::new()
                .expect("can't create new XmlHttpRequest");

            request.open("POST", &server_url_msg_clone);

            let request_2 = request.clone();
            let peer_clone_3 = peer_clone_2.clone();
            let request_callback = Closure::wrap(Box::new(move |e: ProgressEvent| { //instead of ProgressEvent, XmlHttpRequestEventTarget?

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

                        let peer_add_success_callback = Closure::wrap(Box::new(move |e: JsValue| {
                            info!("Client add ice candidate success");
                        }) as Box<dyn FnMut(JsValue)>);
                        let peer_add_failure_callback = Closure::wrap(Box::new(move |e: JsValue| {
                            info!("Client error during 'addIceCandidate': {:?}", e);
                        }) as Box<dyn FnMut(JsValue)>);

                        peer_clone_4.add_ice_candidate_with_rtc_ice_candidate_and_success_callback_and_failure_callback(
                            &candidate,
                            peer_add_success_callback.as_ref().unchecked_ref(),
                            peer_add_failure_callback.as_ref().unchecked_ref());
                        peer_add_success_callback.forget();
                        peer_add_failure_callback.forget();

                    }) as Box<dyn FnMut(JsValue)>);

                    let remote_desc_failure_callback = Closure::wrap(Box::new(move |e: JsValue| {
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

    let peer_error_callback = Closure::wrap(Box::new(move |e: JsValue| {
        info!("Client error during 'createOffer': e value here? TODO");
    }) as Box<dyn FnMut(JsValue)>);

    peer.create_offer()
        .then(&peer_offer_callback);

    peer_offer_callback.forget();
    peer_error_callback.forget();

    return channel;
}