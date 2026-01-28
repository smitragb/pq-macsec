use clap::Parser;
use pq_macsec::{
    init_logging,
    link::{Link, config::LinkConfig},
    nodes::simple::SimpleNode, simulator::{Simulator, topology::p2p::P2PConnection},
};

#[derive(Parser)]
struct Args {
    /// Delay introduced by the link
    #[arg(long, default_value_t = 0)]
    delay: u32,

    /// The packet number on the link that gets dropped
    #[arg(long, default_value_t = 0)]
    drop: u32,

    /// The packet number on the link that get corrupted
    #[arg(long, default_value_t = 0)]
    tamper: u32,
}

fn run(args: &Args) {
    let n0 = {
        let mac = [0x00, 0x1a, 0x2b, 0x3c, 0x4d, 0x5e];
        SimpleNode::new(0, &mac, 10)
    };

    let n1 = {
        let mac = [0x01, 0x1b, 0x2c, 0x3d, 0x4e, 0x5f];
        SimpleNode::new(1, &mac, 20)
    };

    let link = {
        let config = LinkConfig::new(n0.id, n0.port, n1.id, n1.port)
            .with_delay(args.delay)
            .with_drop(args.drop)
            .with_corrupt(args.tamper);
        Link::new(config)
    };

    let p2p = P2PConnection::with_link(&n0, &n1, &link);
    let mut sim = Simulator::new(p2p);
    sim.schedule_send(1, &n0, n0.port, &n1.mac);
    sim.schedule_send(2, &n1, n1.port, &n0.mac);
    sim.run(); 
}

fn main() {
    init_logging();
    let args = Args::parse();
    run(&args);
}
