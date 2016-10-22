# Poettering Bot

> A twitter bot helping the lennart to come up with new things to reinvent

## Installation

The bot is written in rust and needs the `egg-mode` and `rustc-serialize` libraries. They will be automatically installed by cargo on build. You need to install rust and cargo.

## Build

Just run
```
cargo build
```

et voila, you have a binary in `target/debug/poett-bot`

## Information

The wordlist is based on aspells [SCOWL](http://app.aspell.net/create), parsed with a 10 line python script (filter.py) to only have nouns.