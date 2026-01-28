use clap::Parser;
use pq_macsec::{
    init_logging,
    nodes::simple::SimpleNode,
    packet::MacAddress,
    simulator::{Simulator, topology::star::StarTopology},
};

#[derive(Parser)]
struct Args {
    #[arg(
        long,
        value_parser = parse_mac,
        default_value = "d4:a7:c3:2f:4a:bc",
        help = "Mac address of the switch node.",
        long_help = "Mac address of the switch node.\n\
                        Accepted formats:\n\
                        \t- aa:bb:cc:dd:ee:ff\n\
                        \t- aabb.ccdd.eeff\n\
                        Example:\n\
                        \t--mac d4af.e24f.1265"
    )]
    switch_mac: MacAddress,

    #[arg(
        long,
        value_parser = parse_mac,
        default_value = "00:01:00:02:00:01",
        help = "Starting MAC address of nodes connected to the switch.",
        long_help = "Starting MAC address of nodes connected to the switch.\n\
                        Accepted formats:\n\
                        \t- aa:bb:cc:dd:ee:ff\n\
                        \t- aabb.ccdd.eeff\n\
                        Example:\n\
                        \t--mac d4af.e24f.1265"
    )]
    node_mac: MacAddress,

    /// Number of nodes in the chain
    #[arg(long, default_value_t = 3)]
    nodes: usize,
}

fn parse_mac(s: &str) -> Result<MacAddress, String> {
    let mut mac = [0u8; 6];
    if s.contains('.') {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 3 {
            return Err("MAC must be in aaaa.bbbb.cccc format".into());
        }
        for (i, p) in parts.iter().enumerate() {
            let val = u16::from_str_radix(p, 16).map_err(|_| format!("Invalid hex group: {p}"))?;
            mac[2 * i] = (val >> 8) as u8;
            mac[2 * i + 1] = val as u8;
        }
        Ok(mac)
    } else if s.contains(':') {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 6 {
            return Err("MAC must be in aa:bb:cc:dd:ee:ff format".into());
        }

        for (i, p) in parts.iter().enumerate() {
            mac[i] = u8::from_str_radix(p, 16).map_err(|_| format!("invalid hex byte: {p}"))?;
        }
        Ok(mac)
    } else {
        Err("Invalid MAC format".into())
    }
}

fn generate_macs(start: &MacAddress, num_nodes: u8) -> Vec<MacAddress> {
    let mac_as_u64 = u64::from_be_bytes([
        0, 0, start[0], start[1], start[2], start[3], start[4], start[5],
    ]);
    (0..num_nodes)
        .map(|i| {
            let new_mac = mac_as_u64 + i as u64;
            let bytes = new_mac.to_be_bytes();
            [bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]]
        })
        .collect()
}

fn run(args: &Args) {
    let nodes = args.nodes as u8;
    let switch_mac = args.switch_mac;
    let node_macs = generate_macs(&args.node_mac, nodes - 1);
    let switch_ports = (1..nodes).collect();
    let star = StarTopology::new(nodes)
        .add_switch(&switch_mac, &switch_ports)
        .add_nodes(&node_macs)
        .add_links();
    let mut sim = Simulator::new(star);
    let n = SimpleNode::new(1, &node_macs[0], 0);
    for i in 2..nodes {
        let idx = (i - 1) as usize;
        sim.schedule_send(idx as u64, &n, n.port, &node_macs[idx]);
    }
    sim.run();
}

fn main() {
    init_logging();
    let args = Args::parse();
    run(&args);
}
