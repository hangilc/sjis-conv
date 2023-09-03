use std::{fs::OpenOptions, io::Write};

use clap::{Arg, ArgMatches};

fn main() {
    let args = parse_args();
    let output = args.get_one::<String>("output").unwrap();
    let mut f = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output)
        .unwrap();
    match args.get_one::<String>("file") {
        Some(file) => {
            let src = std::fs::read_to_string(file).unwrap();
            let bs = sjis_conv::to_sjis(&src);
            f.write_all(&bs).unwrap();
        }
        None => {}
    }
}

fn parse_args() -> ArgMatches {
    clap::Command::new("from-sjis")
        .arg(Arg::new("output").short('o').long("output").required(true))
        .arg(Arg::new("file"))
        .get_matches()
}
