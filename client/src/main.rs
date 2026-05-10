use std::env::args;

use s5d_lib::{AppError, utils};

fn main() -> Result<(), AppError> {
    let test = utils::collect_args(args())?;
    println!("{:?}", test);

    Ok(())
}