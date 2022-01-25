use rand::distributions::Uniform;
use rand::Rng;
use std::vec;

const BASE: i128 = 2;
const MERSENNE_PRIME: i128 = BASE.pow(107) - 1; // used during development until bigints are used (afterwards use the recommended prime for secp256k1 below)

// The following prime number is the recommended one for secp256k1 ECDSA: http://www.secg.org/sec2-v2.pdf. Here we use a finite field of the same size.
// const FIELD_PRIME: i128 = BASE.pow(256) - BASE.pow(32) - 977;

pub fn run(config: Config) {
    println!("Mersenne prime: {}", MERSENNE_PRIME);
    println!("{:#?}", config);
    let coefficients = generate_polygon_coefficients(&config);
    let shares = generate_shares(&config, &coefficients);
    println!("shares: {:?}", shares)
}

fn generate_polygon_coefficients(config: &Config) -> Vec<i128> {
    let mut rng = rand::thread_rng();
    let range = Uniform::new(-1_000_000_000, 1_000_000_000);

    let coefficients: Vec<i128> = (0..config.threshold).map(|_| rng.sample(&range)).collect();

    println!("coefficients: {:?}", coefficients);

    [vec![config.secret], coefficients].concat()
}

fn generate_shares(config: &Config, coefficients: &Vec<i128>) -> Vec<(i128, i128)> {
    let mut points: Vec<(i128, i128)> = Vec::new();

    for i in 1..config.shares + 1 {
        let y = evaluate_polygon(i as i128, &coefficients);
        points.push((i as i128, y));
    }
    points
}

fn evaluate_polygon(x: i128, coefficients: &Vec<i128>) -> i128 {
    let mut y: i128 = 0; // The y value of the polygon evaluated at x
    for (i, coef) in coefficients.iter().enumerate() {
        y += coef * x.pow(i as u32);
    }

    y
}

#[derive(Debug)]
pub struct Config {
    pub secret: i128,    // The secret which will be split
    pub shares: u32,    // Number of pieces the secret will be split into
    pub threshold: u32, // number of pieces needed to reconstruct the secret
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        //TODO: use proper library for parsing
        if args.len() < 4 {
            return Err("Not enough arguments.");
        }

        let secret = args[1]
            .clone()
            .parse::<i128>()
            .expect("Could not parse the first argument (needs to be integer).");
        let shares = args[2]
            .clone()
            .parse::<u32>()
            .expect("Could not parse the second argument (needs to be positive integer).");
        let threshold = args[3]
            .parse::<u32>()
            .expect("Could not parse the third argument needs to be positive integer).");

        assert!(
            threshold <= shares,
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
        let coefficients = generate_polygon_coefficients(&config);
        assert_eq!(
            coefficients.len() as u32,
            config.threshold + 1,
            "Vector of coefficients has the wrong length."
        );
    }
    #[test]
    fn number_of_generated_shares() {
        let config = Config {
            secret: 123456789,
            shares: 6,
            threshold: 4,
        };
        let coefficients = generate_polygon_coefficients(&config);
        let shares = generate_shares(&config, &coefficients);
        assert_eq!(
            shares.len() as u32,
            config.shares,
            "wrong number of shares generated."
        );
    }
}
