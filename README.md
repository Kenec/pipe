# Pipe

Pipe enables you to stream logs from multiple sources to Elasticsearch.

## Example

#### **`my-config.json`**
```shell
{
  "destination": {
    "elasticsearch": {
      "host": "http(s)://[username]:[password]@[host]:[port]/"
    }
  },
  "sources": {
    "files": {
      "logs": [
        {
          "name": "syslog",
          "path": "/var/log/system.log"
        },
        {
          "name": "install",
          "path": "/var/log/install.log"
        },
        {
          "name": "auth",
          "path": "/var/log/auth.log"
        }
      ]
    }
  }
}
```
#### **To check the validity of the configuration file**
```shell
pipe check --config my-config.json
```

#### **To stream logs to Elasticsearch**
```shell
pipe stream --config my-config.json
```

## Installation

### Docker
1. Create a custom brige network to be used for running the Elasticsearch, Kibana and Pipe containers.

```shell
docker network create pipe-network --driver bridge
```

#### Run Elasticsearch container
```shell
docker run --name elasticsearch --network pipe-network -p 9200:9200 -p 9300:9300 -e "discovery.type=single-node" docker.elastic.co/elasticsearch/elasticsearch:7.11.2
```

#### Run Kibana container
```shell
docker run --name kibana --network pipe-network -p 5601:5601 docker.elastic.co/kibana/kibana:7.11.2
```

2. The Elasticsearch **host** in the Pipe's config file should be pointing to the Elasticsearch container as such
```shell
....
"elasticsearch": {
  "host": "elasticsearch:9200",
....
```

3. The log path to be streamed to Elasticsearch **MUST** be mounted to the same path in the container otherwise, the Pipe container cannot access the log files.  
Eg: If the sample Pipe's config is
   ```shell
   ....
   "files": {
      "logs": [
        {
          "name": "syslog",
          "path": "/var/log/system.log"
        },
   ...
   ```
   then, the path `/var/log` should be mounted to the Pipe's container.
#### Run Pipe container
```shell
docker pull kenec/pipe
docker run --name pipe --network pipe-network -i -v $(pwd):/mnt -v '<path to logs directory on host>:<path to logs directory on container>' kenec/pipe stream -c /mnt/config.json 
```


## Configuration

:TODO
