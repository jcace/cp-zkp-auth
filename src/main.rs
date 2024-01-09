use std::{env, fs::File, io::stdin};

use clap::{command, Arg, Command};
use dotenv::dotenv;
use rpassword::read_password;
use zkp_auth::{client, cp_params, server};

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv().ok(); // for convenience, auto-load from .env file if it exists
    let params = cp_params::ChaumPedersenParams::new_from_env();

    let matches = command!()
        .subcommand_required(true)
        .subcommand(
            Command::new("server").about("run zkp auth server").arg(
                Arg::new("addr")
                    .short('a')
                    .long("addr")
                    .default_value("127.0.0.1:8080"),
            ),
        )
        .subcommand(
            Command::new("client").about("run zkp auth client").args([
                Arg::new("server")
                    .short('s')
                    .long("server")
                    .default_value("127.0.0.1:8080"),
                Arg::new("user").short('u').long("user"),
                Arg::new("password").short('p').long("password"),
            ]),
        )
        .subcommand(
            Command::new("generate")
                .about("generate a fresh set of Chaum-Pederson params")
                .arg(
                    Arg::new("out")
                        .short('o')
                        .long("out")
                        .required(false)
                        .help("output .env file directory"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("server", sub_matches)) => {
            let addr = sub_matches
                .get_one::<String>("addr")
                .expect("server listen address is required");

            server::run_server(addr, params).await;
        }
        Some(("client", sub_matches)) => {
            let addr = sub_matches
                .get_one::<String>("server")
                .expect("server address is required");

            let username = sub_matches.get_one::<String>("user");
            let username = match username {
                Some(u) => Some(u.to_owned()), // If username is already provided via args, use it.
                None => {
                    let mut buffer = String::new();
                    println!("Enter username: ");
                    stdin()
                        .read_line(&mut buffer)
                        .expect("Failed to read username");

                    Some(buffer)
                }
            };

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

            client::run_client(addr, &username.unwrap(), &password.unwrap(), params)
                .await
                .unwrap();
        }
        Some(("generate", sub_matches)) => {
            let out = sub_matches.get_one::<String>("out");

            let p = cp_params::generate_params().unwrap();

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
}
