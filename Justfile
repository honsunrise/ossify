set shell := ["bash", "-c"]

alias l := lint
alias t := test
alias f := fmt
alias cl := clean
alias ck := check

# Default recipe to run when just is called without arguments
default:
    @just check
    @just fmt
    @just lint
    @just fmt

clean:
    cargo clean

check:
    cargo check

test $RUST_BACKTRACE="full":
    cargo test

fmt:
    taplo fmt
    cargo +nightly fmt

lint:
    cargo clippy --fix --allow-dirty --workspace --all-targets --locked -- -D warnings

ci-check:
    taplo fmt --check
    cargo +nightly fmt --all --check
    cargo check --workspace --all-targets --future-incompat-report --locked

ci-lint:
    cargo clippy --workspace --all-targets --future-incompat-report --locked -- -D warnings
