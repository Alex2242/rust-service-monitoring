# Rust Services monitoring

[![Build Status](https://travis-ci.org/Alex2242/rust-service-monitoring.svg?branch=master)](https://travis-ci.org/Alex2242/rust-service-monitoring)

Rust-based tool for monitoring the status of various services.

## Probes

- ping: test if host is up
- https: test connection to HTTPS webserver and notifies time before certificate expiration

## Notification channels

- email

## Command-line usage

Run the program:

```bash
./rsm -c config.yaml
```

it also checks [`"./rsm.yaml"`, `"/etc/rsm.yaml"`] for configuration file if none is specified.

## Configuration file format

See `tests/ressources` for examples.
