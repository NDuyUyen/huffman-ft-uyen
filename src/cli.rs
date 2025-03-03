use super::file;
use huffman;
use log::{error, info};
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
    #[structopt(short = "i", long = "input-type", default_value = "FILE")]
    input_type: IOType,
    #[structopt(long)]
    input: String,
    #[structopt(short = "o", long = "output-type", default_value = "FILE")]
    output_type: IOType,
    #[structopt(long)]
    output: String,
}

impl Opt {
    pub fn handle(&self) {
        let input = match self.input_type {
            IOType::File => file::read_file_content(&self.input),
            IOType::Text => Ok(self.input.clone()),
        };

        match input {
            Ok(input) => {
                let output = match self.command {
                    Command::Compress => huffman::compress(&input),
                    Command::Decompress => huffman::decompress(&input),
                };

                match output {
                    Ok(output) => match self.output_type {
                        IOType::File => {
                            let result = file::write_file(&self.output, &output);
                            match result {
                                Ok(_) => {
                                    info!("Successfully!!!");
                                    info!("Original length: {}", input.len());
                                    info!("New length: {}", output.len());
                                }
                                Err(e) => Opt::print_error(&e.msg),
                            }
                        }
                        IOType::Text => {
                            info!("Output: ```{:?}```", output);
                            info!("Original length: {}", input.len());
                            info!("New length: {}", output.len());
                        }
                    },
                    Err(e) => Opt::print_error(&e.msg),
                }
            }
            Err(e) => Opt::print_error(&e.msg),
        }
    }

    fn print_error(msg: &str) {
        error!("{}", msg);
    }
}
