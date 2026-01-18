use pq_macsec::{init_logging, link::{Link, LinkConfig}, nodes::Node, simulator::Simulator};

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

    let n0_port = 10;
    let n1_port = 11;
    let link = {
        let config = LinkConfig::new(n0.id, n0_port, n1.id, n1_port).with_delay(2);
        Link::new(config)
    };
    let nodes = vec![n0.clone(), n1.clone()];
    let links = vec![link];

    let mut sim = Simulator::new().with_nodes(nodes).with_links(links);
    sim.schedule_send(1, &n0, n0_port, &n1.mac, b"Hello n1".to_vec());
    sim.schedule_send(2, &n1, n1_port, &n0.mac, b"Hello n0".to_vec());
    sim.run();
}
