use chrono::{DateTime, Utc};
use reqwest;
use std::io::{BufReader, BufRead};
use std::fs::{File};
use serde_json::json;

pub async fn stream() {
    let file = File::open("/var/log/system.log").expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut lines_iter = reader.lines().map(|l| l.unwrap());
    let client = reqwest::Client::new();

    loop {
        if let Some(result) = lines_iter.next() {
            let now: DateTime<Utc> = Utc::now();
            let data = json!({ "event": "log", "data": &result, "@timestamp": now.to_rfc3339() });
            eprintln!("{}", data);
            let res = client.post("http://localhost:9200/syslogs/logs").json(&data).send().await;
            match res {
                Err(e) => println!("Error {}", e),
                Ok(..) => (),
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}