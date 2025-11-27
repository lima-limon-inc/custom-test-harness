# Custom Miden rust testing harness

This is a POC repository containing what a custom rust testing harness could look like.

The following command will run the custom test suite:

```sh
cargo test
```

This should run the tests present in `crates/usecase/tests/foo.rs`. Despite
looking just like a normal `cargo test` output, it is using a custom test
harness (which is set in `crates/usecase/Cargo.toml`, with the `harness = false`
field).
