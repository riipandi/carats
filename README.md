# FUNStack

[![(Rust)](https://img.shields.io/badge/rust-v1.69-orange.svg?logo=rust)](https://www.rust-lang.org/)
[![Built with cargo-make](https://sagiegurari.github.io/cargo-make/assets/badges/cargo-make.svg)](https://sagiegurari.github.io/cargo-make)
[![Contributions](https://img.shields.io/badge/Contributions-welcome-blue.svg)](./CODE_OF_CONDUCT.md)
[![GitHub contributors](https://badgers.space/github/contributors/riipandi/funstack?color=green&corner_radius=3)](https://github.com/riipandi/funstack/graphs/contributors)
[![Twitter Badge](https://badgen.net/badge/icon/Follow%20Twitter?icon=twitter&label&color=blue&labelColor=black)](https://twitter.com/riipandi)

<hr/>

This is a starter Rust and React project.

> **WARNING!** This project still in development.
> Everything is experimental, breaking changes can happen and the long-term purpose of this project is not yet clear, use at your own risk!

## 🏁 Quick Start

### Prerequisites

The following are required to run the application in development or in production environment:

- [Rust](https://www.rust-lang.org/tools/install) v1.69 or greater.
- [Node.js](https://nodejs.org/en/download) v18.16.0 or greater.
- [PostgreSQL](https://www.postgresql.org/download/) PostgreSQL server v14 or greater.
- [sqlx](https://crates.io/crates/sqlx) for interacting with the database.
- [sqlx-cli](https://crates.io/crates/sqlx-cli) a command line tool for sqlx.
- [cargo-px](https://crates.io/crates/cargo-px), a cargo sub-command designed to augment cargo's capabilities.
- [cargo-watch](https://crates.io/crates/cargo-watch), a tool for watching the project files and recompiling when they change.
- [cargo-make](https://sagiegurari.github.io/cargo-make/#installation), task runner and build tool for Rust project.
- [toml-cli](https://github.com/gnprice/toml-cli), a simple CLI for editing and querying TOML files.
- [Docker](https://docs.docker.com/engine/install), v2.10 or greater, only required when building container image.

### Create New Project

Install [`cargo-generate`](https://crates.io/crates/cargo-generate) sub-command then execute:

```sh
cargo generate --git https://github.com/riipandi/funstack
```

### Generate Secret Key

Before you continue, you need to create `.env` file (you can duplicate `.env.example`) and
fill the `application secret key` with some random string. To generate a secret key, use
the following command:

```sh
cargo px run --quiet --locked -- generate-secret
```

### Up and Running

1. Prepare environment: `cargo make start-compose`
2. Install dependencies: `cargo make deps`
3. Run database migration: `cargo make migrate`
4. Start development: `cargo make dev`

Application will run at `http://localhost:9090`

### Essential Commands

```sh
cargo make dev             # start development
cargo make build           # build binary file
cargo make run             # run release mode
cargo make docker-build    # build docker image
cargo make docker-run      # run the docker image
cargo make docker-push     # publish the docker image
```

## 🧑🏻‍💻 Development

To run the application in development mode, follow the steps below:

1. Clone this repository.
2. Copy `.env.example` to `.env`
3. Change the `DATABASE_URL` variables value.
4. Run `sqlx database create` to create the database from the specified `DATABASE_URL`
5. Run `sqlx migrate run` to run the migrations
6. use `sqlx migrate add -r <migration_name>` to add a new migration

**Note**: Use `sqlx database drop` to revert the change

### Publish Docker Image

```sh
echo $GH_TOKEN | docker login ghcr.io --username CHANGEME --password-stdin
```

### Running Docker Image

```sh
docker run --rm -it --name funstack --env-file .env.docker -p 9090:9090 ghcr.io/riipandi/funstack:edge
```

### Simple Load Testing

Using [`hey`](https://github.com/rakyll/hey) to perform a simple load testing.

```sh
hey -m GET -n 200 -z 10s http://127.0.0.1:9090/api
```

## 🚀 Deployment

Please see the [documentation](./DEPLOY.md) for more detailed information.

## 🧑🏻‍💻 Contributing

Welcome, and thank you for your interest in contributing to this project! There are many ways in which you can contribute,
beyond writing code. You can read this repository’s [Contributing Guidelines](./CONTRIBUTING.md) to learn how to contribute.

## Maintainer

Currently, [Aris Ripandi](htps://ripandis.com) ([@riipandi](https://twitter.com/riipandi)) is the only maintainer.

## License

Licensed under either of [Apache License 2.0][license-apache] or [MIT license][license-mit] at your option.

> Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you,
> as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

Copyrights in this project are retained by their contributors.

See the [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) files for more information.

[license-mit]: https://choosealicense.com/licenses/mit/
[license-apache]: https://choosealicense.com/licenses/apache-2.0/

---

<sub>🤫 Psst! If you like my work you can support me via [GitHub sponsors](https://github.com/sponsors/riipandi).</sub>
