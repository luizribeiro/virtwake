use virt::connect::Connect;

pub trait FindByMAC {
    fn find_domain_by_mac(&self, mac_addr: String) -> Option<virt::domain::Domain>;
}

impl FindByMAC for Connect {
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
