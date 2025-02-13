use structopt::StructOpt;
mod coding;
mod errors;
mod models;
mod services;
mod utils;

fn main() {
    env_logger::init();
    let opt = services::cli::Opt::from_args();
    opt.handle();
}
