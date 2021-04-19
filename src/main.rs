use crate::collector::streamer::Streamer;
use std::path::PathBuf;
use structopt::StructOpt;
use crate::config::Config;

mod collector;
mod config;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "pipe",
    about = "Read logs from multiple sources and output to elasticsearch"
)]
struct Pipe {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "check", about = "check the validity of the config file")]
    Check(ConfigPath),

    #[structopt(name = "stream", about = "Activate log streaming to elasticsearch")]
    Stream(ConfigPath),
}

#[derive(Debug, StructOpt)]
pub struct ConfigPath {
    #[structopt(short = "c", long = "config", parse(from_os_str))]
    config: PathBuf,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let pipe_config = Pipe::from_args();

    match pipe_config.cmd {
        Command::Check(configPath) => {
            let config = Config::load(&configPath.config).unwrap();
            println!("{:?}", config.destination.elasticsearch.host);
            println!("{:?}", config.sources.files.logs);
            println!(
            "You are checking if the config file {:?} and the elasticsearch connections are okay!",
            configPath
        )},

        Command::Stream(configPath) => Streamer::new(configPath).stream().await
    }
}
