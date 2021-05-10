use crate::collector::streamer::Streamer;
use crate::validator::file_validator::file_path_validator;
use std::path::PathBuf;
use structopt::StructOpt;
use env_logger::Builder;
use log::LevelFilter;

mod collector;
mod config;
mod validator;

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
    Builder::new()
        .filter(None, LevelFilter::Info)
        .init();

    let pipe_config = Pipe::from_args();

    match pipe_config.cmd {
        Command::Check(config_path) => file_path_validator(config_path.config).await,

        Command::Stream(config_path) => Streamer::new(config_path).stream().await
    }
}
