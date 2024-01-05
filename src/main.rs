use num::{bigint::ToBigInt, BigInt, One};

mod client;
mod param_init;
mod server;
use clap::{command, Arg, Command};

#[tokio::main]
async fn main() {
    env_logger::init();

    let matches = command!()
        .subcommand_required(true)
        .subcommand(
            Command::new("server").about("run zkp auth server").arg(
                Arg::new("addr")
                    .default_value("127.0.0.1:8080")
                    // .required(true)
                    .alias("a"),
            ),
        )
        .subcommand(
            Command::new("client").about("run zkp auth client").arg(
                Arg::new("server")
                    .default_value("127.0.0.1:8080")
                    .alias("s"),
            ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("server", sub_matches)) => {
            let addr = sub_matches
                .get_one::<String>("addr")
                .expect("server listen address is required");

            server::run_server(addr).await;
        }
        Some(("client", sub_matches)) => {
            let addr = sub_matches
                .get_one::<String>("server")
                .expect("server address is required");

            client::run_client(addr).await;
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }

    sandbox_run();
}

fn sandbox_run() {
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
