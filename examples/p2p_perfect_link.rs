use pq_macsec::{
    init_logging,
    nodes::SimpleNode,
    simulator::{Simulator, topology::P2PConnection},
};

fn main() {
    init_logging();
    let n0 = {
        let mac = [0x00, 0x1a, 0x2b, 0x3c, 0x4d, 0x5e];
        SimpleNode::new(0, mac, 10)
    };

    let n1 = {
        let mac = [0x01, 0x1b, 0x2c, 0x3d, 0x4e, 0x5f];
        SimpleNode::new(1, mac, 20)
    };

    let p2p = P2PConnection::with_nodes(&n0, &n1);
    let mut sim = Simulator::new(p2p);
    sim.schedule_send(1, &n0, n0.port, &n1.mac);
    sim.schedule_send(2, &n1, n1.port, &n0.mac);
    sim.run();
}
