use crypto_primes::generate_prime;
use is_prime::is_prime;
use num::{bigint::ToBigInt, BigInt, Integer};
use std::{fmt::Display, io::Write, ops::Sub, str::FromStr};
static MAX_GENERATION_ATTEMPTS: i32 = 10;

static ENV_PARAMS_P: &str = "CP_P";
static ENV_PARAMS_Q: &str = "CP_Q";
static ENV_PARAMS_G: &str = "CP_G";
static ENV_PARAMS_H: &str = "CP_H";

#[derive(Debug, Clone)]
pub struct ChaumPedersenParams {
    pub p: BigInt,
    pub q: BigInt,
    pub g: BigInt,
    pub h: BigInt,
}

impl Display for ChaumPedersenParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "P={}\nQ={}\nG={}\nH={}", self.p, self.q, self.g, self.h)
    }
}

impl ChaumPedersenParams {
    pub fn new(p: BigInt, q: BigInt, g: BigInt, h: BigInt) -> Self {
        ChaumPedersenParams { p, q, g, h }
    }

    pub fn new_from_env() -> Self {
        let p = BigInt::from_str(
            &std::env::var(ENV_PARAMS_P)
                .expect("Environment variable for 'ENV_PARAMS_P' not found"),
        )
        .expect("Failed to parse 'ENV_PARAMS_P' as BigInt");

        let q = BigInt::from_str(
            &std::env::var(ENV_PARAMS_Q)
                .expect("Environment variable for 'ENV_PARAMS_Q' not found"),
        )
        .expect("Failed to parse 'ENV_PARAMS_Q' as BigInt");

        let g = BigInt::from_str(
            &std::env::var(ENV_PARAMS_G)
                .expect("Environment variable for 'ENV_PARAMS_G' not found"),
        )
        .expect("Failed to parse 'ENV_PARAMS_G' as BigInt");

        let h = BigInt::from_str(
            &std::env::var(ENV_PARAMS_H)
                .expect("Environment variable for 'ENV_PARAMS_H' not found"),
        )
        .expect("Failed to parse 'ENV_PARAMS_H' as BigInt");

        ChaumPedersenParams { p, q, g, h }
    }

    pub fn to_env_file(
        &self,
        out: &mut std::fs::File,
    ) -> std::result::Result<usize, std::io::Error> {
        out.write(
            format!(
                "{}={}\n{}={}\n{}={}\n{}={}",
                ENV_PARAMS_P,
                self.p,
                ENV_PARAMS_Q,
                self.q,
                ENV_PARAMS_G,
                self.g,
                ENV_PARAMS_H,
                self.h,
            )
            .as_bytes(),
        )
    }

    pub fn y1_y2(&self, x: &BigInt) -> (BigInt, BigInt) {
        let y1 = self.g.modpow(x, &self.p);
        let y2 = self.h.modpow(x, &self.p);

        (y1, y2)
    }
    pub fn r1_r2(&self, k: &BigInt) -> (BigInt, BigInt) {
        let r1 = self.g.modpow(k, &self.p);
        let r2 = self.h.modpow(k, &self.p);

        (r1, r2)
    }

    pub fn s(&self, k: &BigInt, c: &BigInt, x: &BigInt) -> BigInt {
        let c_mul_x = c * x;
        if k > &c_mul_x {
            (k - &c_mul_x) % &self.q
        } else {
            &self.q - (c_mul_x - k) % &self.q
        }
    }
}

/// Param generation
pub fn generate_params() -> ChaumPedersenParams {
    let mut i = 0;
    loop {
        i += 1;
        if i > MAX_GENERATION_ATTEMPTS {
            panic!("could not generate a valid prime after {} attempts. please try running the generator again", i);
        }
        // Create prime
        let p: crypto_bigint::Uint<2> = generate_prime(Some(128));
        let p_hex_str = p.to_string();

        // ! Warning - we can only do this because it fits in u128!
        // Not recommended as we should use the native type for such a large number instead of string-conversion, but just to get it going
        let p_str = u128::from_str_radix(&p_hex_str, 16).unwrap().to_string();

        log::debug!("trying prime: {}", p_str);
        if !is_prime(&p_str) {
            log::error!("{} is not prime. this should not happen!", p_str);
            continue;
        }

        let p = BigInt::from_str(&p_str).unwrap();
        let test = is_cyclic_group_of_prime_order(&p);
        log::debug!("is group prime order? {}", test);
        if !test {
            continue;
        }

        let q = (&p)
            .sub(1i128.to_bigint().unwrap())
            .div_floor(&2i128.to_bigint().unwrap());

        let g = 2u128.to_bigint().unwrap();
        let h = 3u128.to_bigint().unwrap();

        return ChaumPedersenParams::new(p, q, g, h);
    }
}

pub fn is_cyclic_group_of_prime_order(p: &BigInt) -> bool {
    let q = p
        .sub(1i128.to_bigint().unwrap())
        .div_floor(&2i128.to_bigint().unwrap());
    let one = 1i128.to_bigint().unwrap();

    // any two generators other than 1 should both have % q == 1
    let g = 2.to_bigint().unwrap();
    let h = 3.to_bigint().unwrap();

    g.modpow(&q, p) == one && h.modpow(&q, p) == one
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::FromPrimitive;
    use num_bigint::BigInt;

    fn create_test_params() -> ChaumPedersenParams {
        // Example parameters (usually these should be large prime numbers)
        let p = 10009.to_bigint().unwrap();
        let q = 5004.to_bigint().unwrap();
        let g = 2.to_bigint().unwrap();
        let h = 3.to_bigint().unwrap();

        ChaumPedersenParams::new(p, q, g, h)
    }

    #[test]
    fn test_y1_y2() {
        let params = create_test_params();
        let x = BigInt::from_u64(3).unwrap();
        let (y1, y2) = params.y1_y2(&x);

        assert_eq!(y1, BigInt::from_u64(8).unwrap()); // 2 ^ 3
        assert_eq!(y2, BigInt::from_u64(27).unwrap()); // 3 ^ 3
    }

    #[test]
    fn test_r1_r2() {
        let params = create_test_params();
        let k = BigInt::from_u64(4).unwrap();
        let (r1, r2) = params.r1_r2(&k);

        assert_eq!(r1, BigInt::from_u64(16).unwrap()); // 2 ^ 4
        assert_eq!(r2, BigInt::from_u64(81).unwrap()); // 3 ^ 4 = 81
    }

    #[test]
    fn test_s() {
        let params = create_test_params();
        let k = BigInt::from_u64(4).unwrap();
        let c = BigInt::from_u64(2).unwrap();
        let x = BigInt::from_u64(3).unwrap();
        let s = params.s(&k, &c, &x);

        assert_eq!(s, BigInt::from_u64(5002).unwrap()); // 4 - (2 * 3) % 5004 = 5002
    }

    #[test]
    fn test_with_prime_order_group() {
        //  10009 is a prime, and (10009-1)/2 = 5004 is a prime order for the group
        let prime = BigInt::from(10009);
        assert!(is_cyclic_group_of_prime_order(&prime));
    }

    #[test]
    fn test_with_non_prime_order_group() {
        // 11 is a prime, but (11-1)/2 = 5 is not a prime order for the group
        let prime = BigInt::from(11);
        assert!(!is_cyclic_group_of_prime_order(&prime));
    }
}
