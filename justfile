alias b := build
alias r := run
alias c := clean
alias ba := build-all
alias cb := clean-build
alias cba := clean-build-all
alias i := install

default:
  just --list

clean:
    cargo clean

run:
    cargo run

build:
    cargo build --release

build-all:
    cargo build --release
    cargo build --release --target x86_64-pc-windows-gnu

clean-build:
    cargo clean
    cargo build --release

clean-build-all:
    cargo clean
    cargo build --release
    cargo build --release --target x86_64-pc-windows-gnu

install:
    cargo install --path .

publish:
    cargo publish