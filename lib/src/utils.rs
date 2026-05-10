use std::net::IpAddr;

use crate::AppError;

pub fn collect_args<I, S>(iter: I) -> Result<Vec<(String, String)>, AppError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    iter.into_iter()
        .skip(1)
        .map(|s| s.as_ref().to_string())
        .collect::<Vec<String>>()
        .chunks(2)
        .map(|chunk| {
            let [key, value] = chunk else { 
                return Err(AppError::Arguments(format!("invalid argument format {chunk:?} (expected --key value)"))); 
            };
            if !key.starts_with("--") { return Err(AppError::Arguments(format!("invalid argument syntax: {key} (must start with --)"))); }
            Ok((key.to_string(), value.to_string()))
        })
        .collect()
}

//TODO сделать и валидацию и сразу парсинг домена. убрав схему и получив порт + пока из-за точки даже невалдиный IP проходит
pub fn target_is_valid(target: &str) -> bool {
    target.parse::<IpAddr>().is_ok() || target.contains('.')
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_args() {
        let args = vec!["program", "--key", "value"];
        assert!(collect_args(args).is_ok());
    }

    #[test]
    fn test_missing_value() {
        let args = vec!["program", "--key"];
        assert!(collect_args(args).is_err());
    }
}