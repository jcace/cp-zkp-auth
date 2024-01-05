use num::{abs, bigint::ToBigInt, BigInt, One};

mod param_init;

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
    env_logger::init();

    let params = param_init::generate_params();
    println!("p: {:?}", params);

    // secret
    let x = 6969u128.to_bigint().unwrap();

    // y1, y2
    let y1 = params.g.modpow(&x, &params.p);
    let y2 = params.h.modpow(&x, &params.p);

    // k
    let k = 420u128.to_bigint().unwrap(); // todo: random

    // r1, r2
    let r1 = params.g.modpow(&k, &params.p);
    let r2 = params.h.modpow(&k, &params.p);

    //* r1, r2 ----> verifier
    // on verifier
    let c = 1288u128.to_bigint().unwrap(); // todo: random

    //* c ----> prover
    let s;
    let c_mul_x = &c * &x;
    if k > c_mul_x {
        s = (k - &c_mul_x) % &params.q;
    } else {
        s = &params.q - (c_mul_x - k) % &params.q;
    }
    // let s = abs(k - &c * x) % params.q;

    //* s ---> verifier
    // on verifier
    let y1_prime = (params.g.modpow(&s, &params.p) * &y1.modpow(&c, &params.p))
        .modpow(&BigInt::one(), &params.p);

    let y2_prime = (params.h.modpow(&s, &params.p) * &y2.modpow(&c, &params.p))
        .modpow(&BigInt::one(), &params.p);

    log::debug!(
        "
    y1: {}
    y2: {}
    r1: {}
    r2: {}
    c: {}
    s: {}
    y1_prime: {}
    y2_prime: {}
    ",
        &y1,
        &y2,
        r1,
        r2,
        c,
        s,
        y1_prime,
        y2_prime
    );

    assert_eq!(r1, y1_prime);
    assert_eq!(r2, y2_prime);

    log::info!("success! {} == {}, {} == {}", r1, y1_prime, r2, y2_prime);
}
