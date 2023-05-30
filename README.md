# FPR Rust executor

This is a simple multi-language executor written in Rust for the FPR project.

## Usage dev mode

```bash
cargo run -- --debug --exec-type python --port 8070 --script-path ./sample.py --listener-timeout 30000
```

## Build images manually

Build the base image containing the distroless cc image and the executor binary.
```bash
docker build -f executor-base.Dockerfile -t fpr-executor-base:latest .
```

Build the python image containing the base image and the python runtime.
```bash
docker build -f executor-python.Dockerfile -t fpr-executor-python:latest .
```

Build the sample image containing the python image and the sample code.
```bash
docker build -f executor-sample.Dockerfile -t fpr-executor-sample:latest .
```

## Run the sample image

```bash
docker run --rm --name fpr-executor-sample-local -p 8070:8070 fpr-executor-sample:latest
```