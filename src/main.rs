mod cli;
mod file;

use structopt::StructOpt;

fn main() {
    env_logger::init();
    let opt = cli::Opt::from_args();
    opt.handle();
}
