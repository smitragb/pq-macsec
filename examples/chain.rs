use clap::Parser;
use pq_macsec::{
    init_logging,
    link::PortId,
    nodes::simple::SimpleNode,
    packet::MacAddress,
    simulator::{SimTime, Simulator, topology::chain::ChainTopology},
};

#[derive(Parser)]
struct Args {
    #[arg(
        long,
        value_parser = parse_mac,
        default_value = "00:01:00:02:00:01",
        help = "Starting Mac address of the chain.",
        long_help = "Starting Mac address of the chain.\n\
                        Accepted formats:\n\
                        \t- aa:bb:cc:dd:ee:ff\n\
                        \t- aabb.ccdd.eeff\n\
                        Example:\n\
                        \t--mac d4af.e24f.1265"
    )]
    mac: MacAddress,

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
    let start_mac = args.mac;
    let macs = generate_macs(&start_mac, nodes);
    let chain = {
        let ports: Vec<PortId> = (0..(2 * nodes - 2)).collect();
        ChainTopology::new(nodes).build(&macs, &ports)
    };
    let start_node = SimpleNode::new(0, start_mac, 0);
    let mut sim = Simulator::new(chain);
    for id in 1..nodes {
        let dst_mac = macs[id as usize];
        sim.schedule_send(id as SimTime, &start_node, start_node.port, &dst_mac);
    }
    sim.run();
}

fn main() {
    init_logging();
    let args = Args::parse();
    run(&args);
}
