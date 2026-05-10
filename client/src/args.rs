use std::{env::args, net::Ipv4Addr};

use crate::{mode::Mode, prelude::*};


#[derive(Debug, PartialEq)]
pub enum Arg {
    Mode(Mode),
    Server(Ipv4Addr),
}

impl Arg {
    pub fn init() -> Result<Vec<Self>, AppError> {
        Self::from_args(args())
    }

    fn from_args<I, S>(iter: I) -> Result<Vec<Self>, AppError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        utils::collect_args(iter)?
            .into_iter()
            .map(|(key, value)| Self::from_string(&key, &value))
            .collect()
    }

    fn from_string(key: &str, value: &str) -> Result<Self, AppError> {
        match key {
            "--mode" => Ok(Self::Mode(Mode::try_from(value)?)),
            _ => Err(AppError::Arguments(format!("unknown argument {key}")))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_args() {
        let args = vec!["program", "--mode", "proxy", "--server", "127.0.0.1:443"];
        let result = Arg::from_args(args).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], Arg::Server(Ipv4Addr::new(127, 0, 0, 1)));
        //assert_eq!(result[1], Arg::Port(3000));
    }
}