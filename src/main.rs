extern crate clap;
#[macro_use]
extern crate log;
extern crate simplelog;

use clap::{Arg, App};
use simplelog::{CombinedLogger, TermLogger, LevelFilter, Config};

fn main() {
    let matches = App::new("AoC 2018")
            .arg(Arg::with_name("debug")
                 .short("debug")
                 .help("Enables additional logging"))
            .arg(Arg::with_name("DAY")
                 .required(true)
                 .index(1)
                 .help("Which day to run"))
            .get_matches();

    let debug = matches.is_present("debug");
    let day = matches.value_of("DAY").unwrap().parse();

    CombinedLogger::init(
        vec![
            TermLogger::new(if debug { LevelFilter::Debug } else { LevelFilter::Info }, Config::default()).unwrap()
        ]
    ).unwrap();

    if let Err(e) = day {
        error!("day must be supplied as a single integer");
        return;
    }
    let day = day.unwrap();

    match day {
        1 => info!("got day"),
        _ => error!("Day {} not yet implemented", day),
    }
}
