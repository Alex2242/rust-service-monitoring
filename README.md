# Rust Services monitoring

[![Build Status](https://github.com/Alex2242/rust-service-monitoring/actions/workflows/rust.yml/badge.svg)](https://github.com/Alex2242/rust-service-monitoring/actions/workflows/rust.yml)

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
