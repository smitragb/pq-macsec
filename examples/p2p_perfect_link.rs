use pq_macsec::{init_logging, nodes::Node, simulator::Simulator};

fn main() {
    init_logging();
    let n0 = {
        let mac = [0x00, 0x1a, 0x2b, 0x3c, 0x4d, 0x5e];
        Node::new(0, mac)
    };

    let n1 = {
        let mac = [0x01, 0x1b, 0x2c, 0x3d, 0x4e, 0x5f];
        Node::new(1, mac)
    };

    let mut sim = Simulator::new();
    sim.add_node(&n0);
    sim.add_node(&n1);

    // Connects over instantaneous links
    sim.connect(&n0, &n1);

    sim.schedule_send(1, &n0, &n1.mac, b"Hello n1".to_vec());
    sim.schedule_send(2, &n1, &n0.mac, b"Hello n0".to_vec());

    sim.run();
}

