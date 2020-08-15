use hex;
use indicatif::{ProgressBar, ProgressStyle};
use md5::{Digest, Md5};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let name: &String = args
        .get(1)
        .expect("error: a name is required as the first argument.");
    let id: &String = args
        .get(2)
        .expect("error: an ID is required as the second argument.");
    let number_of_ipv4_addresses: u64 = u64::pow(256, 4);
    println!("number of IPv4 addresses: {}", number_of_ipv4_addresses);
    println!(
        "trying if any IPv4 addresses match \"{}\" (ID: \"{}\")",
        name, id
    );
    let bar = ProgressBar::new(256).with_style(
        ProgressStyle::default_bar()
            .template(
                "{msg}ETA {eta_precise}, elapsed {elapsed_precise} [{wide_bar}] {pos}/{len} {percent}%",
            )
            .progress_chars("=- "),
    );
    bar.set_message("started, ");
    for a in 0..=255 {
        for b in 0..=255 {
            for c in 0..=255 {
                for d in 0..=255 {
                    let ip = format!("{}.{}.{}.{}", a, b, c, d);
                    if ip_matches_name_and_id(&ip, name, id) {
                        bar.finish_with_message(&format!("found {}, ", ip));
                        println!("{}", ip);
                        return;
                    }
                }
            }
        }
        bar.inc(1);
        bar.set_message(&format!("done: {}.255.255.255, ", a))
    }
    bar.finish();
    println!("not found");
}

fn hash_name_and_ip(name: &str, ip: &str) -> String {
    hex::encode(Md5::digest(format!("{}{}", name, ip).as_bytes()))
}

fn ip_matches_name_and_id(ip: &str, name: &str, id: &str) -> bool {
    hash_name_and_ip(name, ip) == id
}
