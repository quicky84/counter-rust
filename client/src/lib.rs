use std::net::SocketAddr;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub address: SocketAddr,
    pub n_tasks: u32,
    /// Minimum value for generated random number
    pub min: u32,
    // Maximum value for generated random number
    pub max: u32,
}

impl Config {
    /// Returns a config parsed from the input
    ///
    /// # Arguments
    ///
    /// * `args` - Vector of strings to parse;
    /// Expected:
    ///     args[1] - ip-address:port
    ///     args[2] - number of tasks sent to server
    ///     args[3] - minimal value
    ///     args[4] - maximum value
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 5 {
            return Err("Not enough arguments: n_tasks min_value max_value");
        }

        let address: SocketAddr = match args[1].parse() {
            Ok(value) => value,
            Err(_) => return Err("IP address is invalid"),
        };

        let n_tasks: u32 = match args[2].parse() {
            Ok(value) => value,
            Err(_) => return Err("Number of tasks must be a positive integer"),
        };

        let min: u32 = match args[3].parse() {
            Ok(value) => value,
            Err(_) => return Err("Min must be a positive integer"),
        };

        let max: u32 = match args[4].parse() {
            Ok(value) => value,
            Err(_) => return Err("Max must be a positive integer"),
        };

        if min < max {
            Ok(Config {
                address,
                n_tasks,
                min,
                max,
            })
        } else {
            Err("min_value must be smaller than max_value")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};

    #[test]
    fn parses() {
        let attempt = Config::new(
            vec![
                String::from("executable"),
                String::from("127.0.0.1:8000"),
                String::from("10"),
                String::from("1"),
                String::from("20"),
            ].as_slice(),
        );

        let value = Config {
            address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8000),
            n_tasks: 10,
            min: 1,
            max: 20,
        };

        println!("{:?}", attempt);

        assert!(attempt.is_ok());

        let attempt = attempt.unwrap();

        assert_eq!(attempt, value);
    }
}
