use std::{thread, time};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "pipe", about = "Read logs from multiple sources and output to elasticsearch")]
struct Pipe {
    #[structopt(subcommand)]
    cmd: Command
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "check", about = "check the validity of the config file")]
    Check(Config),

    #[structopt(name = "stream", about = "Activate log streaming to elasticsearch")]
    Stream(Config),
}

#[derive(Debug, StructOpt)]
struct Config {
    #[structopt(short = "c", long = "config", parse(from_os_str))]
    config: PathBuf,
}

fn main() {
    let timer = time::Duration::from_secs(1);
    let config = Pipe::from_args();

    match &config.cmd {
        Command::Check(config) => println!("You are checking if the config file {:?} and the elasticsearch connections are okay!", config),
        Command::Stream(config) => println!("You are now streaming logs {:?}, from multiple sources in batches, to elasticsearch", config),
    }

    loop {
        thread::sleep(timer);

        println!("Reading the target!");
        println!("{:?}", config);
    }
}
