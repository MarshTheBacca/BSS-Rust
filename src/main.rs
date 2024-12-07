use chrono::Local;
use fern::Dispatch;
use log::{debug, error, info, warn};

mod node;
use node::Node;

fn main() {
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}] [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()
        .unwrap();
    // Log some messages
    debug!("This is a debug message");
    info!("This is an info message");
    warn!("This is a warning message");
    error!("This is an error message");
    // Create a node instance
    let node = Node::new(0, vec![0.0, 0.0], vec![1, 2], vec![3, 4]);
    // Log the node
    info!("{}", node);
    debug!("{:?}", node);
}
