# deciduously-com

From the ashes, it rises... a deciduously-com for 2020 and beyond.

## Requirements

- [Rust 2018](https://www.rust-lang.org/) - stable toolchain
- [Node/NPM](https://nodejs.org/en/)
- [Docker](https://www.docker.com/)

## Usage

### NPM Scripts

- `dev`: start dev server on port 3000, watching for source changes
- `prod`: build and start production Docker container on port 8080 - must stop container via docker
- `run`: run local image
- `lint`: run linters
- `test`: run tests
- `test:watch`: run tests, watching for changes

### Executable Options

```shell
❯ cargo run -- -h
   Compiling deciduously-com v0.1.0 (/home/ben/code/deciduously-com)
    Finished dev [unoptimized + debuginfo] target(s) in 2.42s
     Running `target/debug/deciduously-com -h`
 INFO  deciduously_com::config > Set verbosity to info
deciduously-com 0.1.0
deciduously-com backend

USAGE:
    deciduously-com [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --address <address>    Server address - default [default: 127.0.0.1]
    -p, --port <port>          Server port 0-65535 [default: 3000]
```

Options set in `src/config.toml` override these defaults, but options passed at the command line override `config.toml`.

## Dependencies

### Crates

- [askama](https://github.com/djc/askama) - Templates
- [hyper](https://hyper.rs/) - HTTP
- [lazy_static](https://github.com/rust-lang-nursery/lazy-static.rs) - Runtime-evaluated statics
- [log](https://github.com/rust-lang/log) - Logging macros
- [pretty_env_logger](https://github.com/seanmonstar/pretty-env-logger) - Pretty log output
- [structopt](https://github.com/TeXitoi/structopt) - CLI

### Style

- [TailwindCSS](https://tailwindcss.com/)
- [highlight.js](https://highlightjs.org/)
- [Postcss](https://postcss.org/)
- [Autoprefixer](https://github.com/postcss/autoprefixer)
- [Purgecss](https://purgecss.com/)
- [Cssnano](https://cssnano.co/)
- [Stylelint](https://stylelint.io/)
