# example-microbit
micro:bit v2 via rust


```sh
$ cargo generate \
  --git https://github.com/rtic-rs/app-template \
  --branch main \
  --name my-app
```

```sh
$ cargo install cargo-binutils
$ rustup +stable component add llvm-tools-preview
$ cargo add cargo-embed
$ cargo install flip-link
$ cargo install probe-run
$ cargo install defmt
```

```sh
$ probe-run --list-chips
```
