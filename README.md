Lightweight library for building Rust-WASM services on Cloudflare Workers.


The goal of this library is to make it easy to build fast and
lightweight HTTP-based services in WASM, hosted on Cloudflare Workers. 
To keep things fast and lightweight, there is a strong preference for
significant new capabilities to added as compile-time features or separate
libraries.

## Features

- Fully async
- Request & response bodies can be text, json, or binary
- Non-blocking structured logging via [`service-logging`](https://github.com/stevelr/service-logging)
- Deferred tasks that run after response is returned to client
- Static file handling

## Add-ons

- CORS and OAuth via [`wasm-service-oauth`](https://github.com/stevelr/wasm-service-oauth)

## Getting started

To start a new project,

    wrangler generate -t rust PROJECT \
	    https://github.com/stevelr/rustwasm-service-template

where PROJECT is your project name.

[rustwasm-service-template](https://github.com/stevelr/rustwasm-service-template/blob/master/README.md) 
contains some relevant sample code, as well as
instructions for setting up of Cloudflare and (optionally) Coralogix logging
service.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

## Updates

See [CHANGELOG](./CHANGELOG.md) for recent changes

