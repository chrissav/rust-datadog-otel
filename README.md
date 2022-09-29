# rust-datadog-otel

Rust example with opentelemetry traces to datadog.

Original code from open-telemetry/opentelemetry-rust.  This fork containerizes the app for kubernetes and makes the app run requests continuously in a loop.

## Usage

First run version 7.22.0 or above of the datadog-agent locally as described [here](https://docs.datadoghq.com/agent/)

For local run:

```shell
$ cargo run
```

For deploying to kubernetes, build the docker image
```
make build
```

Update the `$registry` in deployment.yaml and run
```
make deploy
```
