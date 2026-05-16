use url::Url;

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

pub fn parse_url(target: &str) -> Result<(String, u16), AppError> {
    let url = Url::parse(target).map_err(|_| AppError::InvalidDomain)?;

    let host = url.host_str().ok_or(AppError::InvalidDomain)?.to_string();

    let port = match url.port() {
        Some(p) => p,
        None => match url.scheme() {
            "https" => 443,
            "http" => 80,
            _ => return Err(AppError::InvalidDomain),
        },
    };

    Ok((host, port))
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
    fn test_invalid_args() {
        let args = vec!["program", "key", "value"];
        assert!(collect_args(args).is_err());
    }

    #[test]
    fn test_missing_value() {
        let args = vec!["program", "--key"];
        assert!(collect_args(args).is_err());
    }
}