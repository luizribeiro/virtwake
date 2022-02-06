use virt::connect::Connect;
use wake_on_lan::Receiver;

trait Finder {
    fn find_domain_by_mac(&self, mac_addr: String) -> Option<virt::domain::Domain>;
}

impl Finder for Connect {
    fn find_domain_by_mac(&self, mac_addr: String) -> Option<virt::domain::Domain> {
        let domains = self
            .list_all_domains(
                virt::connect::VIR_CONNECT_LIST_DOMAINS_ACTIVE
                    | virt::connect::VIR_CONNECT_LIST_DOMAINS_INACTIVE,
            )
            .unwrap();
        for domain in domains {
            let xml = domain
                .get_xml_desc(virt::domain::VIR_DOMAIN_NONE)
                .unwrap()
                .as_str()
                .to_lowercase();
            if xml.contains(&mac_addr) {
                return Some(domain);
            }
        }
        return None;
    }
}

fn main() {
    let receiver = Receiver::from("0.0.0.0", 9);
    let conn = Connect::open("qemu:///system").unwrap();
    loop {
        let payload = receiver.listen(None).unwrap();
        if payload.len() < 12 {
            return;
        }
        let mac_address = &payload[6..12];
        let mac_addr_as_str = format!(
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            mac_address[0],
            mac_address[1],
            mac_address[2],
            mac_address[3],
            mac_address[4],
            mac_address[5]
        );
        println!("Received WoL packet for {}", mac_addr_as_str);

        let optional_domain = conn.find_domain_by_mac(mac_addr_as_str);
        if optional_domain.is_none() {
            continue;
        }

        let domain = optional_domain.unwrap();
        println!("Starting domain {}...", domain.get_name().unwrap());
        let ret = domain.create();
        if ret.is_err() {
            eprintln!("Error: {}", ret.unwrap_err());
        }
    }
}
