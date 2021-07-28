# Ookma-Kyi

> A Rust implementation of the now defunct Battlemail game

Battlemail was an email game originally written by Paul Gouge. The concept is
fairly simple: Players pick 6 attack and 6 defense moves which are sent to their
opponent. The opponent then picks 6 attack and 6 defense moves. Once submitted,
both players are then treated to a display of martial arts based on the moves
both players picked.

## Prerequisites

* [Rust 1.50+](https://www.rust-lang.org)
* [SQLite 3+](https://www.sqlite.org)

## Install

```sh
cargo build
```

## Configuration

Before running the application, you will need to configure the following
environment variables:

```sh
export PORT=<port to listen for incoming connections>
```

## Usage

start the server
```sh
cargo run
```

and access your site or localhost to access the app.

```
http://www.your-domain-here.tld
-or-
127.0.0.1
```

## License

[MIT]: https://opensource.org/licenses/MIT

`ookma-kyi` is open source software released under the [MIT license][MIT].

## Sponsored by
[![DigitalOcean](https://opensource.nyc3.cdn.digitaloceanspaces.com/attribution/assets/SVG/DO_Logo_horizontal_blue.svg)](https://m.do.co/c/9f92698d759c)