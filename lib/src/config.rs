use std::env::args;

use crate::{AppError, utils};

pub trait Config: Default + Sized {
    fn new() -> Result<Self, AppError> {
        Self::from_args(args())
    }

    fn from_args<I, S>(iter: I) -> Result<Self, AppError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut config = Self::default();
        for (key, value) in utils::collect_args(iter)? {
            config.set_param(&key, &value)?;
        }
        Ok(config)
    }

    fn set_param(&mut self, key: &str, value: &str) -> Result<(), AppError>;
    fn validate(&mut self) -> Result<(), AppError>;
}