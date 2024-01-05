use std::{ops::Sub, str::FromStr};

use crypto_primes::generate_prime;
use is_prime::is_prime;
use num::{bigint::ToBigInt, BigInt, Integer};

#[derive(Debug)]
pub struct ChaumPedersenParams {
    pub p: BigInt,
    pub q: BigInt,
    pub g: BigInt,
    pub h: BigInt,
}

pub fn generate_test_params() -> ChaumPedersenParams {
    ChaumPedersenParams {
        p: BigInt::from_str("23").unwrap(),
        q: BigInt::from_str("11").unwrap(),
        g: BigInt::from_str("4").unwrap(),
        h: BigInt::from_str("9").unwrap(),
    }
}

pub fn generate_params() -> ChaumPedersenParams {
    // TODO: maybe dont infinite loop if it fails to generate for some reason
    loop {
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

        // todo: different g and h
        let g = 2.to_bigint().unwrap();
        let h = 3.to_bigint().unwrap();

        return ChaumPedersenParams { p, q, g, h };
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
