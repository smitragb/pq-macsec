use pq_macsec::{
    init_logging,
    link::PortId,
    nodes::simple::SimpleNode,
    packet::MacAddress,
    simulator::{Simulator, topology::chain::ChainTopology},
};

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

fn offset_mac(start: &MacAddress, offset: u8, num_nodes: u8) -> MacAddress {
    assert!(offset <= num_nodes);
    let mac_as_u64 = u64::from_be_bytes([
        0, 0, start[0], start[1], start[2], start[3], start[4], start[5],
    ]);

    let last_mac = mac_as_u64 + (offset - 1) as u64;
    let bytes = last_mac.to_be_bytes();

    [bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]]
}

fn main() {
    init_logging();
    let num_nodes = 3;
    let start = [0x00, 0x01, 0x00, 0x02, 0x00, 0x01];
    let chain = {
        let macs = generate_macs(&start, num_nodes);
        let ports: Vec<PortId> = (0..(2 * num_nodes - 2)).collect();
        ChainTopology::new(num_nodes).build(&macs, &ports)
    };
    let start_node = SimpleNode::new(0, start, 0);
    let mut sim = Simulator::new(chain);
    let dst = offset_mac(&start, num_nodes, num_nodes);
    sim.schedule_send(1, &start_node, start_node.port, &dst);
    let dst = offset_mac(&start, 2, num_nodes);
    sim.schedule_send(2, &start_node, start_node.port, &dst);
    sim.run();
}
