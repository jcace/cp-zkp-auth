use crypto_primes::generate_prime;
use is_prime::is_prime;
use num::{
    bigint::{self, ToBigInt},
    BigInt, Integer,
};
use num_bigint::RandBigInt;
use std::{fmt::Display, io::Write, ops::Sub, str::FromStr};
static MAX_GENERATION_ATTEMPTS: i32 = 10;

static ENV_PARAMS_P: &str = "CP_P";
static ENV_PARAMS_Q: &str = "CP_Q";
static ENV_PARAMS_G: &str = "CP_G";
static ENV_PARAMS_H: &str = "CP_H";

#[derive(Debug)]
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
        let p = BigInt::from_str(&std::env::var(ENV_PARAMS_P).unwrap()).unwrap();
        let q = BigInt::from_str(&std::env::var(ENV_PARAMS_Q).unwrap()).unwrap();
        let g = BigInt::from_str(&std::env::var(ENV_PARAMS_G).unwrap()).unwrap();
        let h = BigInt::from_str(&std::env::var(ENV_PARAMS_H).unwrap()).unwrap();

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
        let test = check_if_group_prime_order(&p);
        log::debug!("is group prime order? {}", test);
        if !test {
            continue;
        }

        let q = (&p)
            .sub(1i128.to_bigint().unwrap())
            .div_floor(&2i128.to_bigint().unwrap());

        let mut rng = rand::thread_rng();
        let g = rng.gen_bigint_range(&2.to_bigint().unwrap(), &p);
        let h = rng.gen_bigint_range(&g, &p);

        return ChaumPedersenParams::new(p, q, g, h);
    }
}

pub fn check_if_group_prime_order(p: &BigInt) -> bool {
    let q = p
        .sub(1i128.to_bigint().unwrap())
        .div_floor(&2i128.to_bigint().unwrap());
    let one = 1i128.to_bigint().unwrap();

    // any two generators other than 1 should both have % q == 1
    let g = 2.to_bigint().unwrap();
    let h = 3.to_bigint().unwrap();

    g.modpow(&q, p) == one && h.modpow(&q, p) == one
}
