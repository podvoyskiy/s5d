mod prelude;
mod args;
mod mode;

use prelude::*;
use tracing::Level;
use tracing_subscriber::fmt;

use crate::args::Arg;

fn main() -> Result<(), AppError> {
    fmt()
        .with_target(false)
        .with_max_level(Level::TRACE)
        .init();

    info!("socks5 client started");

    for arg in Arg::init()? {
        debug!(?arg);
    }

    Ok(())
}