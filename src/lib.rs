use rand::distributions::Uniform;
use rand::Rng;

pub fn run(config: Config) {
    println!("{:#?}", config);
    generate_polygon(&config);
}

fn generate_polygon(config: &Config) -> Vec<i64> {
    let mut rng = rand::thread_rng();
    let random_number: i64 = rng.gen_range(-1_000_000..1_000_000);

    println!("Random number {}", random_number);

    let mut rng = rand::thread_rng();
    let range = Uniform::new(-1_000_000, 1_000_000);

    let coefficients: Vec<i64> = (0..config.threshold)
        .map(|_| rng.sample(&range))
        .collect();

    println!("coefficients: {:#?}", coefficients);

    [vec![config.secret], coefficients].concat()
}

#[derive(Debug)]
pub struct Config {
    pub secret: i64,    // The secret which will be split
    pub shares: u64,    // Number of pieces the secret will be split into
    pub threshold: u64, // number of pieces needed to reconstruct the secret
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        //TODO: use proper library for parsing
        if args.len() < 4 {
            return Err("Not enough arguments.");
        }

        let secret = args[1]
            .clone()
            .parse::<i64>()
            .expect("Could not parse the first argument (needs to be integer).");
        let shares = args[2]
            .clone()
            .parse::<u64>()
            .expect("Could not parse the second argument (needs to be positive integer).");
        let threshold = args[3]
            .parse::<u64>()
            .expect("Could not parse the third argument needs to be positive integer).");

        assert!(
            threshold < shares,
            "Threshold needs to be smaller or equal to the numbers of generated shares"
        );

        Ok(Config {
            secret,
            shares,
            threshold,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_of_generated_coefficients() {
        let config = Config {
            secret: 123456789,
            shares: 6,
            threshold: 4,
        };
        let coefficients = generate_polygon(&config);
        assert_eq!(
            coefficients.len() as u64,
            config.threshold + 1,
            "Vector of coefficients has the wrong length"
        );
    }
}
