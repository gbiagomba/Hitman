use clap::{App, Arg};
use std::fs;
use std::io::{self, Read};
use ripgrep::Regex;
use std::process;

fn main() {
    let matches = App::new("hitman")
        .version("1.0")
        .about("Extracts IPv4, IPv6 and FQDNs from text")
        .arg(Arg::new("file")
            .short('f')
            .long("file")
            .takes_value(true)
            .about("Input file to read"))
        .arg(Arg::new("out")
            .short('o')
            .long("out")
            .takes_value(true)
            .about("Output file to write results"))
        .arg(Arg::new("quiet")
            .short('q')
            .long("quiet")
            .about("Quiet mode, no output to stdout"))
        .get_matches();

    let input = match matches.value_of("file") {
        Some(file) => {
            fs::read_to_string(file).unwrap_or_else(|err| {
                eprintln!("Error reading file {}: {}", file, err);
                process::exit(1);
            })
        }
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer).unwrap_or_else(|err| {
                eprintln!("Error reading from stdin: {}", err);
                process::exit(1);
            });
            buffer
        }
    };

    let ipv4_re = Regex::new(r"\b(?:\d{1,3}\.){3}\d{1,3}\b").unwrap();
    let ipv6_re = Regex::new(r"\b(?:[a-fA-F0-9]{1,4}:){7}[a-fA-F0-9]{1,4}\b").unwrap();
    let fqdn_re = Regex::new(r"\b(?:[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]{2,6}\b").unwrap();

    let mut results = vec![];

    for line in input.lines() {
        for ipv4 in ipv4_re.find_iter(line) {
            results.push(ipv4.as_str().to_string());
        }
        for ipv6 in ipv6_re.find_iter(line) {
            results.push(ipv6.as_str().to_string());
        }
        for fqdn in fqdn_re.find_iter(line) {
            results.push(fqdn.as_str().to_string());
        }
    }

    if let Some(output_file) = matches.value_of("out") {
        fs::write(output_file, results.join("\n")).unwrap_or_else(|err| {
            eprintln!("Error writing to file {}: {}", output_file, err);
            process::exit(1);
        });
    } else if !matches.is_present("quiet") {
        for result in results {
            println!("{}", result);
        }
    }
}