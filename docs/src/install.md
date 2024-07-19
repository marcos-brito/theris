# Installation

At the time being there are two ways of installing theris. Even tho both methods requires building from source, the process still
easy.

## Using Cargo

First thing you'll need is installing **Rust** and **Cargo**. Follow [this](https://www.rust-lang.org/tools/install) and you should be
good.

Then run:

```sh
cargo install --git https://github.com/marcos-brito/theris theris
```

> Make sure ~/.cargo/bin is in your `$PATH` variable

## Cloning the source

Assuming you have **Git**, **Rust** and **Cargo** installed, first clone the repository:

```sh
git clone https://github.com/marcos-brito/theris && cd theris
```

After that you'll need to compile the code:

```sh
cargo build --release
```

The final step is move the compiled binary to somewhere you can use:

```sh
cp ./target/release/theris ~/.local/bin
```
