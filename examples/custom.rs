use std::{fs, path::PathBuf};

use clap::Parser;
use pq_macsec::{
    init_logging,
    link::LinkBuilder,
    nodes::NodeBuilder,
    simulator::{
        Simulator,
        topology::custom::{CustomTopology, TopologyConfig},
    },
};

#[derive(Parser)]
struct Args {
    /// Path to config file
    #[arg(long)]
    pub config: PathBuf,
}

fn run(args: Args) {
    let config_path = args.config;
    let toml_str = fs::read_to_string(&config_path).expect("Failed to read topology config");
    let topo_cfg: TopologyConfig = toml::from_str(&toml_str).expect("Invalid topology toml");
    let TopologyConfig { nodes, links } = topo_cfg;
    let n0 = NodeBuilder::build(&nodes[0]);
    let n1 = NodeBuilder::build(&nodes[1]);
    let (end_a, end_b) = LinkBuilder::build(&links[0]).get_ends();
    let p2p = CustomTopology::new().with_nodes(nodes).with_links(links);
    let mut sim = Simulator::new(p2p);
    sim.schedule_send(1, &n0, end_a.1, &n1.mac());
    sim.schedule_send(2, &n1, end_b.1, &n0.mac());
    sim.run();
}

fn main() {
    init_logging();
    let args = Args::parse();
    run(args);
}
