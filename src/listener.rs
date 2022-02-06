use wake_on_lan::Receiver;

pub struct WakeListener {
    receiver: Receiver,
}

impl WakeListener {
    pub fn new() -> WakeListener {
        WakeListener {
            receiver: Receiver::from("0.0.0.0", 9),
        }
    }

    pub fn wait(&self) -> Option<String> {
        let payload = self.receiver.listen(None).unwrap();
        if payload.len() < 12 {
            return None;
        }
        let mac_address = &payload[6..12];
        return Some(format!(
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            mac_address[0],
            mac_address[1],
            mac_address[2],
            mac_address[3],
            mac_address[4],
            mac_address[5]
        ));
    }
}
