set dotenv-load
alias r := run
alias t := test

_default:
    @just --list

# Run the server
run:
    trunk serve --open

# Run the tests
test:
    cargo test

# Run RustyWind check
tw-check:
    rustywind --check-formatted .

# Run RustyWind fix
tw-fix:
    rustywind --write .