# Oricalchum
A lightweight actor library, allowing for the creation and management of actors in a concurrent environment. 

## Features

- Asynchronous message handling using Tokio
- Procedural macro for actor tracking

## Getting Started

To get started with Oricalchum, add the following dependencies to your `Cargo.toml` file:

```toml
[dependencies]
oricalchum = "0.1.0"
oricalchum_derive = "0.1.0"
tokio = { version = "1", features = ["full"] }
