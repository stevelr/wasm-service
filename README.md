Lightweight library for building Rust-WASM services on Cloudflare Workers.


The goal of this library is to make it easy to build fast and
lightweight HTTP-based services in WASM, hosted on Cloudflare Workers. 
To keep things fast and lightweight, there is a strong preference for
significant new capabilities to added as compile-time features or separate
libraries.

## Features

- Fully async
- Request & response bodies can be text, json, or binary
- Non-blocking structured logging
- Deferred tasks that run after response is returned to client

## Add-ons

- CORS handling and OAuth2

## Getting started

To start a new project,

    wrangler generate -t rust PROJECT \
	    https://github.com/stevelr/rustwasm-service-template

where PROJECT is your project name.

[rustwasm-service-template](https://github.com/stevelr/rustwasm-service-template/blob/master/README.md) 
contains some relevant sample code, as well as
instructions for setting up of Cloudflare and (optionally) Coralogix logging
service.

## Updates (v0.3)

* Changes to support add-ons. 
See [CHANGELOG](./CHANGELOG.md) for recent changes including 
breaking api changes to `Handler.handle` and `Runnable.run` functions.


