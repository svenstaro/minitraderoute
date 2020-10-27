use simplelog::{Config, LevelFilter, SimpleLogger};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Minitraderoute")]
pub struct Opt {
    /// Verbose mode (-v, -vv, -vvv)
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,
}

pub fn parse_cli() {
    // Parse commandline options.
    let opt = Opt::from_args();

    // Set the verbosity level of the logger.
    let level = match opt.verbose {
        0 => LevelFilter::Error,
        1 => LevelFilter::Warn,
        2 => LevelFilter::Info,
        _ => LevelFilter::Debug,
    };
    SimpleLogger::init(level, Config::default()).unwrap();
}
