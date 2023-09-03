use std::{io::Write, fs::OpenOptions, cell::RefCell, rc::Rc};

use clap::{ArgMatches, Arg};

fn main() {
    let args = parse_args();
    let output = args.get_one::<String>("output");
    let out: Box<dyn Fn(&str)> = match output {
        Some(o) => {
            let f = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(o)
                .unwrap();
            let rc = Rc::new(RefCell::new(f));
            Box::new(move |s| {
                let rc_cloned = rc.clone();
                let mut f = rc_cloned.borrow_mut();
                f.write_all(s.as_bytes()).unwrap()
            })
        }
        None => Box::new(|s| {
            std::io::stdout().write_all(s.as_bytes()).unwrap()
        })
    };
    let infile = args.get_one::<String>("file").unwrap();
    let bs = std::fs::read(infile).unwrap();
    let cs = sjis_conv::from_sjis(&bs);
    out(&cs);
}

fn parse_args() -> ArgMatches {
    clap::Command::new("from-sjis")
        .arg(Arg::new("output").short('o').long("output"))
        .arg(Arg::new("file").required(true))
        .get_matches()
}