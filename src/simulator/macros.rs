use crate::packet::MacAddress;

pub fn format_mac(mac: &MacAddress) -> String {
    format!(
        "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
    )
}

#[macro_export]
macro_rules! log_frame {
    ($action:expr, $time:expr, $frame:expr, $port:expr) => {
        let src_mac = $crate::simulator::macros::format_mac(&$frame.src_mac);
        let dst_mac = $crate::simulator::macros::format_mac(&$frame.dst_mac);
        let ethertype = format!("0x{:04x}", $frame.ethertype);
        match $action {
            "SEND" => {
                tracing::info!(
                    time = $time, outgoing_port = $port, src_mac = src_mac,
                    dst_mac = dst_mac, ethertype = ethertype,
                    payload = String::from_utf8_lossy(&$frame.payload).as_ref(),
                    $action,
                );
            },
            "RECV" => {
                tracing::info!(
                    time = $time, incoming_port = $port, src_mac = src_mac,
                    dst_mac = dst_mac, ethertype = ethertype,
                    payload = String::from_utf8_lossy(&$frame.payload).as_ref(),
                    $action,
                );
            },
            _      => {
                tracing::info!(
                    time = $time, src_mac = src_mac, dst_mac = dst_mac, 
                    ethertype = ethertype,
                    payload = String::from_utf8_lossy(&$frame.payload).as_ref(),
                    $action,
                );
            },
        }
    };
}
