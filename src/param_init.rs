use std::ops::Sub;

use num::{bigint::ToBigUint, BigUint, Integer};

pub fn find_generators(p: &BigUint) -> Option<(BigUint, BigUint)> {
    let one = 1u128.to_biguint().unwrap();
    let p_minus_1 = p - &one;

    let mut g = 2.to_biguint().unwrap();

    loop {
        let one = one.clone();
        if g == p_minus_1 {
            break;
        }
        // Check if g is a generator
        if g.modpow(&p_minus_1, p) != one {
            g += one;
            continue;
        }

        log::debug!("found first generator {}", g);
        let mut h = g.clone() + &one;
        loop {
            let one = one.clone();
            // Check if h is a generator and independent from g
            if h.modpow(&p_minus_1, p) == one {
                log::debug!("found second generator {}", h);
                return Some((g, h));
            }
            h += one;
        }
    }
    None
}

pub fn check_if_group_prime_order(p: &BigUint) -> bool {
    let q = p
        .sub(1u128.to_biguint().unwrap())
        .div_floor(&2u128.to_biguint().unwrap());
    let one = 1u128.to_biguint().unwrap();

    // any two generators other than 1 should both have % q == 1
    let g = 2.to_biguint().unwrap();
    let h = 3.to_biguint().unwrap();

    g.modpow(&q, p) == one && h.modpow(&q, p) == one
}
