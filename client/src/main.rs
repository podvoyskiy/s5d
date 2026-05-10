mod prelude;

use prelude::*;
use std::env::args;

fn main() -> Result<(), AppError> {
    let test = utils::collect_args(args())?;
    println!("{:?}", test);

    Ok(())
}