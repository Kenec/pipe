use chrono::{DateTime, Utc};
use reqwest;
use serde_json::json;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::ConfigPath;
use std::path::{PathBuf, Path};
use crate::config::Config;
use std::collections::HashMap;
use std::borrow::Borrow;
use url::{Url, ParseError};
use std::string::String;
use std::thread;
use std::time::Duration;

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

#[derive(Debug)]
pub struct Streamer {
    config_path: PathBuf
}

// create vector of struct to hold the object of sources
#[derive(Debug)]
struct Sources {
    sources: Vec<source>,
}

#[derive(Debug)]
struct source {
    name: String,
    source: PathBuf
}

impl Streamer {
    // Instantiate Streamer with the config path
    pub fn new(config_path: ConfigPath) -> Self {
        Self { config_path: config_path.config }
    }

    // Get sources
    pub fn load_sources(&self) -> anyhow::Result<Sources> {
        let mut sources: Vec<source> = vec![];
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

            let my_source = source { name: String::from(name), source: PathBuf::from(path) };
            sources.push(my_source);
        }

        Ok(Sources { sources })
    }

    // Pipe data from sources to Elasticsearch
    pub async fn stream(&self) {
        let my_sources:Sources = self.load_sources().unwrap();

        for i in my_sources.sources.iter() {
            println!("name: {} and source: {:?}", i.name, i.source);
            self.upstream(i.name.to_string(), i.source.as_path().display().to_string()).await
        }
    }

    async fn upstream(&self, name: String, path: String) {

    }
}
