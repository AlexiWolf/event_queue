# Generic Event Queue 

[![CI](https://github.com/AlexiWolf/event_queue/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexiWolf/event_queue/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/l/generic_event_queue)](https://github.com/AlexiWolf/event_queue#license)
[![Crates.io](https://img.shields.io/crates/v/generic_event_queue)](https://crates.io/crates/generic_event_queue)

A generic event-queue API.

The Event-Queue API in this crate was originally crated as part of
[Wolf Engine's](https://crates.io/crates/wolf_engine) event-handling system, 
but it's proven useful in other projects.  I decided to make it into a 
stand-alone crate, after copy-pasting it into multiple projects.

## Getting Started

To use the latest release version:

```TOML
# Cargo.toml

[dependencies]
generic_event_queue = "*"
```

To use the latest development version:

```TOML
# Cargo.toml

[dependencies]
generic_event_queue = { git = "https://github.com/AlexiWolf/generic_event_queue" }
```

[See the docs](https://docs.rs/generic_event_queue/latest/generic_event_queue/)
for usage instructions.

## Status

Generic Event Queue is currently in development.  You should expect missing 
features, bugs, changing APIs, and other spooky stuff until release 1.0.

# License

Generic Event Queue is licensed under either:

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

At your option.

Unless you explicitly state otherwise, any contribution intentionally 
submitted for inclusion in the work by you, as defined in the Apache-2.0 
license, shall be dual licensed as above, without additional terms or 
conditions.

