use std::{io::Error, str::FromStr};

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "cmd", about = "Command")]
enum Command {
    #[structopt(name = "compress")]
    Compress,
    #[structopt(name = "decompress")]
    Decompress,
}

#[derive(Debug)]
enum IOType {
    Text,
    File,
}

impl FromStr for IOType {
    type Err = Error;

    fn from_str(i: &str) -> Result<Self, Self::Err> {
        match i {
            "TEXT" => Ok(Self::Text),
            "FILE" => Ok(Self::File),
            _ => Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid IO Type",
            )),
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
pub struct Opt {
    #[structopt(subcommand)]
    command: Command,
    #[structopt(short = "i", long = "input-type", default_value = "TEXT")]
    input_type: IOType,
    #[structopt(long)]
    input: String,
    #[structopt(short = "o", long = "output-type", default_value = "TEXT")]
    output_type: IOType,
    #[structopt(long)]
    output: String,
}
