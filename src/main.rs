mod find_by_mac;
mod listener;

use crate::find_by_mac::FindByMAC;
use crate::listener::WakeListener;
use virt::connect::Connect;

fn main() {
    let listener = WakeListener::new();
    let conn = Connect::open("qemu:///system").unwrap();
    loop {
        let mac_addr = listener.wait().unwrap();
        println!("Received WoL packet for {}", mac_addr);

        let optional_domain = conn.find_domain_by_mac(mac_addr);
        if optional_domain.is_none() {
            continue;
        }

        let domain = optional_domain.unwrap();
        println!("Starting domain {}...", domain.get_name().unwrap());
        let ret = if domain.is_active().unwrap() {
            domain.create()
        } else {
            domain.resume()
        };
        if ret.is_err() {
            eprintln!("Error: {}", ret.unwrap_err());
        }
    }
}
