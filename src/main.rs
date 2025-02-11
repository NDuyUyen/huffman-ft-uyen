use structopt::StructOpt;

mod coding;
mod errors;
mod models;
mod services;
mod utils;

fn main() {
    let opt = services::cli::Opt::from_args();
    println!("{:?}", opt);
}
