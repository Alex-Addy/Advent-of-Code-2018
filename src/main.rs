extern crate clap;
#[macro_use]
extern crate log;
extern crate simplelog;

use std::io;

use clap::{App, Arg};
use simplelog::{CombinedLogger, Config, LevelFilter, TermLogger};

mod solutions;

fn main() {
    let matches = App::new("AoC 2018")
        .arg(
            Arg::with_name("debug")
                .short("debug")
                .help("Enables additional logging"),
        ).arg(
            Arg::with_name("DAY")
                .required(true)
                .index(1)
                .help("Which day to run"),
        ).get_matches();

    let debug = matches.is_present("debug");
    let day = matches.value_of("DAY").unwrap().parse();

    CombinedLogger::init(vec![
        TermLogger::new(
            if debug {
                LevelFilter::Debug
            } else {
                LevelFilter::Info
            },
            Config::default(),
        ).unwrap(),
    ]).unwrap();

    if let Err(_) = day {
        error!("day must be supplied as a single integer");
        return;
    }
    let day = day.unwrap();
    let stdin = io::stdin();

    match day {
        1 => solutions::day01::work(stdin.lock()),
        2 => solutions::day02::work(stdin.lock()),
        _ => error!("Day {} not yet implemented", day),
    }
}
