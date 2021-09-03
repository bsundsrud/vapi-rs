# Vapi-rs

Rust bindings, wrappers, and utilities for dealing with Varnish-Cache's
shared memory segment for logging and stats.

## vapi-sys

Rust bindings to the libvarnishapi C-lib.  Requires `libvarnishapi-dev`
or equivalent to be installed on your system.  Low level and unsafe.

## vapi

Safe Rust abstraction over `vapi-sys`.

## vapi-logger

Line-oriented logger for Varnish, intended to be used for ingestion into
a system like Elasticsearch via Logstash or similar.

See its [README](vapi-logger/README.md) for details.
