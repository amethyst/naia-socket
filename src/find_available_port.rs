use std::net::{Ipv4Addr, SocketAddrV4, ToSocketAddrs, UdpSocket};

pub fn get(ip_addr: &String) -> Option<u16> {
    let ipv4addr: Ipv4Addr = ip_addr.parse().expect("cannot parse");
    (1025..65535).find(|port| port_is_available(&ipv4addr, *port))
}

fn port_is_available(ip_addr: &Ipv4Addr, port: u16) -> bool {
    let ipv4 = SocketAddrV4::new(*ip_addr, port);
    test_bind_udp(ipv4).is_some()
}

fn test_bind_udp<A: ToSocketAddrs>(addr: A) -> Option<u16> {
    Some(UdpSocket::bind(addr).ok()?.local_addr().ok()?.port())
}
