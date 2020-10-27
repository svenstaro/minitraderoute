use simplelog::{Config, LevelFilter, SimpleLogger};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Minitraderoute")]
pub struct Opt {
    /// Verbose mode (-v, -vv, -vvv)
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,

    /// Set flag to disable audio
    #[structopt(short, long)]
    pub no_audio: bool,
}

pub fn parse_cli(opt: &Opt) {
    // Set the verbosity level of the logger.
    let level = match opt.verbose {
        0 => LevelFilter::Error,
        1 => LevelFilter::Warn,
        2 => LevelFilter::Info,
        _ => LevelFilter::Debug,
    };
    SimpleLogger::init(level, Config::default()).unwrap();
}
