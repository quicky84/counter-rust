#[derive(Debug, PartialEq)]
pub struct Config {
    pub port: u16,
    pub n_kernels: usize,
    pub timeout: u64,
}

impl Config {
    /// Returns a config parsed from the input
    ///
    /// # Arguments
    ///
    /// * `args` - Vector of strings to parse;
    /// Expected:
    ///     args[1] - ip
    ///     args[2] - number of kernels to use
    ///     args[3] - timeout for completion
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 4 {
            return Err(
                "Not enough arguments: port n_kernels completion_timeout_secs",
            );
        }

        let port: u16 = match args[1].parse() {
            Ok(value) => value,
            Err(_) => return Err("Port must be a positive integer"),
        };

        let n_kernels: usize = match args[2].parse() {
            Ok(value) => value,
            Err(_) => return Err("Number of tasks must be a positive integer"),
        };

        let timeout: u64 = match args[3].parse() {
            Ok(value) => value,
            Err(_) => return Err("Completion timeout must be a positive integer"),
        };

        Ok(Config {
            port,
            n_kernels,
            timeout,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses() {
        let attempt = Config::new(
            vec![
                String::from("executable"),
                String::from("8000"),
                String::from("3"),
                String::from("1"),
            ].as_slice(),
        );

        let value = Config {
            port: 8000,
            n_kernels: 3,
            timeout: 1,
        };

        assert!(attempt.is_ok());

        let attempt = attempt.unwrap();

        assert_eq!(attempt, value);
    }
}
