pub mod link;
pub mod simulator;
pub mod nodes;
pub mod packet;

pub fn init_logging() {
    tracing_subscriber::fmt::init();
}
