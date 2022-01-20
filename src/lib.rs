#[derive(Debug)]
pub struct Config {
    pub secret: String,  // The secret which will be split
    pub shares: u64,  // Number of pieces the secret will be split into
    pub threshold: u64,  // number of pieces needed to reconstruct the secret
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 4 {
            return Err("Not enough arguments.");
        }

        let secret = args[1].clone();
        let shares = args[2].parse::<u64>().unwrap();  // TODO: handle error case properly here
        let threshold = args[3].parse::<u64>().unwrap();  // TODO: handle error case properly here

        Ok(Config {
            secret,
            shares,
            threshold,
        })
    }
}