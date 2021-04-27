use chrono::{DateTime, Utc};
use serde_json::json;
use crate::ConfigPath;
use std::path::{PathBuf};
use crate::config::Config;
use std::collections::HashMap;
use std::string::String;
use std::time::Duration;
use async_std::task;
use async_std::channel;
use async_std::fs::File;
use async_std::io::BufReader;
use async_std::io::prelude::BufReadExt;

#[derive(Debug)]
struct LogObservabilityKind {
    logs: Vec<HashMap<String, String>>
}

#[derive(Debug)]
struct MetricsObservabilityKind {
    metrics: Vec<HashMap<String, String>>
}

#[derive(Debug)]
struct TracesObservabilityKind {
    traces: Vec<HashMap<String, String>>
}

#[derive(Debug, Clone)]
pub struct Streamer {
    config_path: PathBuf
}

// create vector of struct to hold the object of sources
#[derive(Debug)]
pub struct Sources {
    sources: Vec<Source>,
}

#[derive(Debug)]
struct Source {
    name: String,
    source: PathBuf
}

impl Streamer {
    // Instantiate Streamer with the config path
    pub fn new(config_path: ConfigPath) -> Self {
        Self { config_path: config_path.config }
    }

    // Get sources
    pub async fn load_sources(&self) -> anyhow::Result<Sources> {
        let mut sources: Vec<Source> = vec![];
        let source_from_config = Config::load(&self.config_path).unwrap();

        // get all the source types there are
        let file_source_type = source_from_config.sources.files;
        // TODO: let audio_source_type = source_from_config.audios;


        // check if the source(s) from config are all valid. Otherwise, throw an error.
        // TODO: checksources(file_source_type);

        // get the observability type: Logs, Metrics, Traces
        let mut file_type_logs = LogObservabilityKind{ logs: file_source_type.logs };
        // let file_type_metrics = MetricsObservabilityKind { metrics: file_source_type.metrics };
        // let file_type_traces = TracesObservabilityKind { traces: file_source_type.traces };

        // let audio_type_logs = LogObservabilityKind { logs: audio_source_type.logs);
        // let audio_type_metrics = MetricsObservabilityKind { metrics: audio_source_type.metrics);
        // let audio_type_traces = TracesObservabilityKind { traces: audio_source_type.traces);

        // TODO: pass the array of files to a function and check if they are all valid file paths
        // validate_source(&file_type_logs.logs);

        // TODO: deserialize the payload and parse it to sources
        for file_in_source in file_type_logs.logs.iter_mut() {
            // println!("{:?}", file_in_source["name"]);
            let name = &file_in_source["name"];
            let path = &file_in_source["path"];

            let my_source = Source { name: String::from(name), source: PathBuf::from(path) };
            sources.push(my_source);
        }

        Ok(Sources { sources })
    }

    // Pipe data from sources to Elasticsearch
    pub async fn stream(self) {
        let my_sources: Sources = self.load_sources().await.unwrap();
        let (sender, receiver) = channel::bounded(100);

        my_sources.sources.into_iter().for_each(|source| {
            let streamer = self.clone();
            async_std::task::spawn(streamer.upstream(source,  sender.clone()));
        });

        // let mut stdout = io::stdout();

        loop {
            let line = receiver.recv().await.unwrap();
            let now: DateTime<Utc> = Utc::now();
            let data = json!({ "event": "log", "data": line.0.to_string(), "@timestamp": now.to_rfc3339() });
            let url = format!("http://localhost:9200/{}/logs", line.1.to_string());
            eprintln!("{}", data);
            let res = reqwest::Client::new().post(url).json(&data).send().await;

            // stdout.write_all(line.as_bytes()).await.unwrap();
            // match res {
            //     Err(e) => println!("Error {}", e),
            //     Ok(..) => (),
            // }
            if let Err(e) = res { println!("Error {}", e) }
        }
    }

    async fn upstream(self, stream_source: Source, sender: channel::Sender<(String, String)>) {
        let file = loop {
          let log_name = &stream_source.name;
          match File::open(&stream_source.source.as_path()).await {
              Ok(f) => break (f, log_name),
              Err(_) => {
                  println!("#######  {:?} file opened ####### ", stream_source.source.as_path());
                  task::sleep(Duration::from_secs(1)).await;
                  continue;
              }
          }
        };

        let mut reader = BufReader::new(file.0);
        let mut read_until = 0u64;

        loop {
            let metadata = async_std::fs::metadata(stream_source.source.as_path()).await.unwrap();
            let file_len = metadata.len();

            if read_until < file_len {
                let mut buffer = String::new();
                let read_from_buffer = reader.read_line(&mut buffer).await.unwrap();
                read_until += read_from_buffer as u64;
                sender.send(( buffer, file.1.to_string())).await.unwrap();
            } else {
                task::sleep(Duration::from_secs(1)).await;
            }
        }
    }
}
