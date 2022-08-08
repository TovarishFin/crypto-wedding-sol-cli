# crypto_wedding_cli

This is a command line interface (CLI) to interact with the crypto wedding program on Solana.
The CLI was built using [clap](https://github.com/clap-rs/clap).

## Installation

- clone this repo
- `cargo install --path .`
- call using `crypto_wedding_cli` from your terminal

The repo can also be used without installing by calling `cargo run -- <cli-command-here>`.

## Usage

```sh
crypto_wedding_cli --help

crypto_wedding_cli 0.1.0

USAGE:
    crypto_wedding_cli <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    airdrop-funds
    cancel-wedding
    close-partner
    create-and-airdrop-account
    divorce
    get-own-account
    give-answer
    help                          Print this message or the help of the given subcommand(s)
    print-partner
    print-wedding
    setup-partner
    setup-wedding
    update-name
    update-partner
    update-vows
    watch-wedding
```
