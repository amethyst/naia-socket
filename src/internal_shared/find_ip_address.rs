use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::process::Command;

pub fn get() -> String {
    let output = match Command::new("hostname").args(&["-I"]).output() {
        Ok(ok) => ok,
        Err(_) => {
            return String::from("");
        }
    };

    let stdout = match String::from_utf8(output.stdout) {
        Ok(ok) => ok,
        Err(_) => {
            return String::from("");
        }
    };

    let ips: Vec<&str> = stdout.trim().split(" ").collect::<Vec<&str>>();
    let first = ips.first();
    match first {
        Some(first) => {
            if !first.is_empty() {
                if let Ok(addr) = first.parse::<Ipv4Addr>() {
                    return addr.to_string();
                } else if let Ok(addr) = first.parse::<Ipv6Addr>() {
                    return addr.to_string();
                } else {
                    return String::from("");
                }
            } else {
                return String::from("");
            }
        }
        None => return String::from(""),
    }
}