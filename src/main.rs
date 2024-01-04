mod param_init;

use is_prime::is_prime;
use num::bigint::*;
use std::str::FromStr;

use crate::param_init::{check_if_group_prime_order, find_generators};

// https://docs.rs/static-dh-ecdh/latest/static_dh_ecdh/constants/constant.DH_GROUP_5_EXPONENT_LENGTH.html
// RFC 3526 - https://www.rfc-editor.org/rfc/rfc3526#section-2
// static DH_GROUP_5_PRIME: &str = "FFFFFFFFFFFFFFFFC90FDAA22168C234C4C6628B80DC1CD129024E088A67CC74020BBEA63B139B22514A08798E3404DDEF9519B3CD3A431B302B0A6DF25F14374FE1356D6D51C245E485B576625E7EC6F44C42E9A637ED6B0BFF5CB6F406B7EDEE386BFB5A899FA5AE9F24117C4B1FE649286651ECE45B3DC2007CB8A163BF0598DA48361C55D39A69163FA8FD24CF5F83655D23DCA3AD961C62F356208552BB9ED529077096966D670C354E4ABC9804F1746C08CA237327FFFFFFFFFFFFFFFF";
// static DH_GROUP_5_GENERATOR: usize = 2;
// static DH_GROUP_5_EXPONENT_LENGTH: usize = 192;

/**
*      p: BigInt::from_str("42765216643065397982265462252423826320512529931694366715111734768493812630447").unwrap(),
       q: BigInt::from_str("21382608321532698991132731126211913160256264965847183357555867384246906315223").unwrap(),
       g: BigInt::from_str("4").unwrap(),
       h: BigInt::from_str("9").unwrap(),
*/

fn main() {
    // Example use (replace with actual prime generation)
    // let p: BigUint = 101.to_biguint().unwrap(); // A small prime number for demonstration

    let big_prime_str =
        "42765216643065397982265462252423826320512529931694366715111734768493812630447";

    if !is_prime(big_prime_str) {
        panic!("{} is not prime", big_prime_str);
    }

    let p = BigUint::from_str(big_prime_str).unwrap();

    println!("prime: {}", p);
    // match find_generators(&p) {
    //     Some((g, h)) => println!("Generators found: g = {}, h = {}", g, h),
    //     None => println!("No generators found"),
    // }

    let test = check_if_group_prime_order(&p);
    println!("Is group prime order? {}", test);

    // println!("Hello, world!");

    // let p: crypto_bigint::Uint<2> = generate_prime(Some(128));

    // // Setup phase
    // // 1536-bit group
    // // let p = 2u128.pow(1536) - 2u128.pow(1472) - 1 + 2u128.pow(64) * ((2u128.pow(1406) + 741804) / 2u128.pow(1406));
    // // let p = U1536::from_be_hex(DH_GROUP_5_PRIME);
    // // let g = DH_GROUP_5_GENERATOR;
    // println!("p: {}", p);
    // // let g = 2;

    // log::debug!("Commitment phase");
}
