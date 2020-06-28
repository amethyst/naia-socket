use std::net::{IpAddr, SocketAddrV4, SocketAddrV6, ToSocketAddrs, UdpSocket};

/// Given an IPv4 Address, attempt to find an available port on the current host
pub fn find_available_port(ip_addr: &IpAddr) -> Option<u16> {
    (1025..65535).find(|port| port_is_available(ip_addr, *port))
}

fn port_is_available(ip_addr: &IpAddr, port: u16) -> bool {
    match ip_addr {
        IpAddr::V4(v4_addr) => {
            let socket_addr = SocketAddrV4::new(*v4_addr, port);
            test_bind_udp(socket_addr).is_some()
        }
        IpAddr::V6(v6_addr) => {
            let socket_addr = SocketAddrV6::new(*v6_addr, port, 0, 0);
            test_bind_udp(socket_addr).is_some()
        }
    }
}

fn test_bind_udp<A: ToSocketAddrs>(addr: A) -> Option<u16> {
    Some(UdpSocket::bind(addr).ok()?.local_addr().ok()?.port())
}
