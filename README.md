# Rust ORM

This is a project that I am writing to familiarize myself with Rust and several
libraries, including Tide and Diesel.

## Setup

1. Copy `.env.sample` to `.env` and update the variables.
    - Remember to change `JWT_SECRET` for safety reasons.
2. Run migrations with `diesel migration run`.
    - You may need to run `cargo install diesel_cli --no-default-features --features postgres` to install the Diesel CLI.