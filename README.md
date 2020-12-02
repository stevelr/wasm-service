Base library for serverless WASM on Cloudflare Workers.

## Features

- Fully async
- Request/response bodies can be text, json(serialized), or binary
- Non-blocking structured logging
- Deferred tasks that run after response is returned to client

## Getting started

To get started with a WASM service, use 

    wrangler generate -t rust PROJECT \
	    https://github.com/stevelr/wasm-service-template

where PROJECT is your project name.

Follow instructions in 
[wasm-service-template](https://github.com/stevelr/wasm-service-template/README.md)

for Cloudflare setup and (optional) Coralogix logging setup

## Dependencies

This crate takes advantage of some recent updates in 
the popular reqwest crate (v0.10.9)
that make it possible to do outgoing http requests 
from a Cloudflare worker. It is now possible
to write libraries using http clients that compile 
for both wasm targets and other non-wasm targets such as 
native windows/mac/linux, without any feature switches.

