# Kabuki

Actor library built on top of the Tokio platform.

## Overview

Kabuki provides a simple way to structure concurrent applications built for
the Tokio / Futures platform. It is based on the [actor
model](https://en.wikipedia.org/wiki/Actor_model). An actor is a small unit
of computation that is used to manage state and resources. It receives
messages from other actors and performs some kind of action based on that
input. This way, instead of having state and resources accessed
concurrently, only a single thread access it and concurrent access is
handled through message passing.

## Motivation

[Tokio](https://docs.rs/tokio-core/0.1.2/tokio_core/reactor/index.html) and
[Futures](https://docs.rs/futures/0.1.7/futures/task/index.html) provide a
lightweight task primitive. However, it leaves the details of managing
concurrency up to the developer.

## Usage

First, add this to your `Cargo.toml`:

```toml
[dependencies]
kabuki = { git = "https://github.com/carllerche/kabuki" }
```

Next, add this to your crate:

```rust
extern crate kabuki;
```

And then, use kabuki!

# License

`kabuki` is primarily distributed under the terms of both the MIT license and
the Apache License (Version 2.0), with portions covered by various BSD-like
licenses.

See LICENSE-APACHE, and LICENSE-MIT for details.
