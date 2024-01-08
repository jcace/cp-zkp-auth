use std::{env, fs::File};

use num::{bigint::ToBigInt, BigInt, One};

mod client;
mod cp_params;
mod server;
use clap::{arg, command, Arg, Command};
use dotenv::dotenv;
use rpassword::read_password;

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv().ok(); // for convenience, auto-load from .env file if it exists

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
            Command::new("client").about("run zkp auth client").args([
                Arg::new("server")
                    .default_value("127.0.0.1:8080")
                    .alias("s"),
                Arg::new("user").default_value("user").alias("u"),
                Arg::new("password").alias("p"),
            ]),
        )
        .subcommand(
            Command::new("generate")
                .about("generate a fresh set of Chaum-Pederson params")
                .args([Arg::new("out")
                    .alias("o")
                    .required(false)
                    .help("output .env file directory")]),
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

            let user = sub_matches
                .get_one::<String>("user")
                .expect("username is required");

            let password = sub_matches.get_one::<i64>("password");

            let password = match password {
                Some(p) => Some(p.to_owned()), // If password is already provided via args, use it.
                None => {
                    println!("Enter password: ");
                    match read_password()
                        .expect("Failed to read password")
                        .parse::<i64>()
                    {
                        Ok(parsed_password) => Some(parsed_password),
                        Err(_) => {
                            eprintln!("Error: Password must be a number.");
                            None
                        }
                    }
                }
            };

            client::run_client(addr, user, &password.unwrap()).await;
        }
        Some(("generate", sub_matches)) => {
            let out = sub_matches.get_one::<String>("out");

            let p = cp_params::generate_params();

            match out {
                Some(out) => {
                    let current_dir = env::current_dir().expect("Failed to get current directory");
                    let full_path = current_dir.join(out);
                    let mut file = File::create(full_path).expect("Failed to open file");

                    p.to_env_file(&mut file)
                        .expect("failed to params to env file");
                }
                None => {
                    println!("{}", p);
                }
            }
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }

    // sandbox_run();
}

fn sandbox_run() {
    let params = cp_params::generate_params();
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
